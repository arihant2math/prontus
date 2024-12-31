use boa_engine::gc::Tracer;
use boa_engine::job::{FutureJob, JobQueue, NativeJob};
use boa_engine::property::Attribute;
use boa_engine::{Context, Finalize, JsResult, JsValue, NativeFunction, Source, Trace};
use boa_runtime::{Console, ConsoleState, Logger};
use futures_util::{stream::FuturesUnordered, Future, StreamExt};
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::{join, task};

// Tokio is needed because of reqwest
pub struct TokioQueue {
    executor: Runtime,
    futures: RefCell<FuturesUnordered<FutureJob>>,
    jobs: RefCell<VecDeque<NativeJob>>,
}

impl TokioQueue {
    fn new(runtime: Runtime) -> Self {
        Self {
            executor: runtime,
            futures: RefCell::default(),
            jobs: RefCell::default(),
        }
    }
}

impl JobQueue for TokioQueue {
    fn enqueue_promise_job(&self, job: NativeJob, _context: &mut Context) {
        self.jobs.borrow_mut().push_back(job);
    }

    fn run_jobs(&self, context: &mut Context) {
        // Early return in case there were no jobs scheduled.
        if self.jobs.borrow().is_empty() && self.futures.borrow().is_empty() {
            return;
        }

        let context = RefCell::new(context);

        self.executor.block_on(async move {
            // Used to sync the finalization of both tasks
            let finished = Cell::new(0b00u8);

            let fut_queue = async {
                loop {
                    if self.futures.borrow().is_empty() {
                        finished.set(finished.get() | 0b01);
                        if finished.get() >= 0b11 {
                            // All possible futures and jobs were completed. Exit.
                            return;
                        }
                        // All possible jobs were completed, but `jqueue` could have
                        // pending jobs. Yield to the executor to try to progress on
                        // `jqueue` until we have more pending futures.
                        task::yield_now().await;
                        continue;
                    }
                    finished.set(finished.get() & 0b10);

                    // Blocks on all the enqueued futures, driving them all to completion.
                    let futures = &mut std::mem::take(&mut *self.futures.borrow_mut());
                    while let Some(job) = futures.next().await {
                        // Important to schedule the returned `job` into the job queue, since that's
                        // what allows updating the `Promise` seen by ECMAScript for when the future
                        // completes.
                        self.enqueue_promise_job(job, &mut context.borrow_mut());
                    }
                }
            };

            let job_queue = async {
                loop {
                    if self.jobs.borrow().is_empty() {
                        finished.set(finished.get() | 0b10);
                        if finished.get() >= 0b11 {
                            // All possible futures and jobs were completed. Exit.
                            return;
                        }
                        // All possible jobs were completed, but `fqueue` could have
                        // pending futures. Yield to the executor to try to progress on
                        // `fqueue` until we have more pending jobs.
                        task::yield_now().await;
                        continue;
                    };
                    finished.set(finished.get() & 0b01);

                    let jobs = std::mem::take(&mut *self.jobs.borrow_mut());
                    for job in jobs {
                        if let Err(e) = job.call(&mut context.borrow_mut()) {
                            eprintln!("Uncaught {e}");
                        }
                        task::yield_now().await;
                    }
                }
            };

            // Wait for both queues to complete
            join!(fut_queue, job_queue);
        });
    }

    fn enqueue_future_job(&self, future: FutureJob, _context: &mut Context) {
        self.futures.borrow_mut().push(future);
    }
}

pub struct ForwardingLogger {
    forwarder: mpsc::Sender<String>,
}

impl ForwardingLogger {
    pub fn new(forwarder: mpsc::Sender<String>) -> Self {
        Self { forwarder }
    }
}

unsafe impl Trace for ForwardingLogger {
    unsafe fn trace(&self, tracer: &mut Tracer) {
        todo!()
    }

    unsafe fn trace_non_roots(&self) {
        todo!()
    }

    fn run_finalizer(&self) {
        todo!()
    }
}

impl Finalize for ForwardingLogger {}

impl Logger for ForwardingLogger {
    fn log(&self, msg: String, state: &ConsoleState, context: &mut Context) -> JsResult<()> {
        self.forwarder.send(msg).unwrap();
        Ok(())
    }

    fn info(&self, msg: String, state: &ConsoleState, context: &mut Context) -> JsResult<()> {
        self.forwarder.send(msg).unwrap();
        Ok(())
    }

    fn warn(&self, msg: String, state: &ConsoleState, context: &mut Context) -> JsResult<()> {
        self.forwarder.send(msg).unwrap();
        Ok(())
    }

    fn error(&self, msg: String, state: &ConsoleState, context: &mut Context) -> JsResult<()> {
        self.forwarder.send(msg).unwrap();
        Ok(())
    }
}

fn call_client_function(
    _this: &JsValue,
    _args: &[JsValue],
    _context: &mut Context,
) -> impl Future<Output = JsResult<JsValue>> {
    async move {
        // TODO: call client functions
        Ok(JsValue::Null)
    }
}

pub fn create_boa_context(sender: mpsc::Sender<String>) -> Context {
    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create tokio runtime");
    let mut ctx = Context::builder()
        .job_queue(Rc::new(TokioQueue::new(tokio_rt)))
        .build()
        .expect("Failed to create boa context");
    let console = Console::init_with_logger(&mut ctx, ForwardingLogger::new(sender));
    ctx.register_global_property(Console::NAME, console, Attribute::all())
        .expect("the console builtin shouldn't exist");
    ctx.register_global_builtin_callable(
        "client_function".parse().unwrap(),
        1,
        NativeFunction::from_async_fn(call_client_function),
    )
    .expect("Failed to register client_function");
    ctx
}

pub struct ScriptingEngine {
    context: Arc<Mutex<Context>>,
    pub receiver: mpsc::Receiver<String>,
}

impl ScriptingEngine {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let context = Arc::new(Mutex::new(create_boa_context(sender)));
        Self { context, receiver }
    }

    pub fn run_code(&self, code: &str) -> JsResult<()> {
        let mut context = self.context.lock().unwrap();
        context.eval(Source::from_bytes(code))?;
        context.run_jobs();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::create_boa_context;
    use boa_engine::{JsResult, Source};

    #[test]
    fn test_main() -> JsResult<()> {
        let js_code = r#"
              let two = 1 + 1;
              let definitely_not_four = two + "2";
              client_function().then((result) => {
                  console.log(result);
              });

              definitely_not_four
          "#;

        let (sender, receiver) = std::sync::mpsc::channel();
        // Instantiate the execution context
        let mut context = create_boa_context(sender);

        // Parse the source code
        let result = context.eval(Source::from_bytes(js_code))?;
        context.run_jobs();
        while let Ok(msg) = receiver.try_recv() {
            println!("{}", msg);
        }
        println!("{}", result.display());

        Ok(())
    }
}
