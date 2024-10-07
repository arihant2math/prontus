use std::error::Error;
use std::io::stdin;
use std::time::Instant;

use milli::{
    execute_search, filtered_universe, DefaultSearchLogger, GeoSortStrategy, SearchContext,
    SearchLogger, TermsMatchingStrategy, TimeBudget,
};
use std::io::BufRead;
use std::sync::Arc;
use std::thread;

#[tokio::main]
async fn indexer_thread() {
    let index = Arc::new(search::get_index("D:\\tmp-milli-data\\milli-data").unwrap());
    let mut indexer = search::MessageIndexer::new("D:\\tmp-milli-data\\milli-data", "D:\\tmp-milli-data\\message_index_info.json").await;
    loop {
        let r = indexer.execute(index.clone()).await;
        if let Err(e) = r {
            eprintln!("Error: {:?}", e);
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: do no hardcode
    let dataset = "D:\\tmp-milli-data\\milli-data";
    let index = Arc::new(search::get_index(dataset)?);

    thread::spawn(|| {
        indexer_thread();
    });
    let mut default_logger = DefaultSearchLogger;
    println!("Init complete");

    loop {
        // read query from stdin
        let query = stdin().lock().lines().next().unwrap().unwrap();
        let logger: &mut dyn SearchLogger<_> = &mut default_logger;


        println!("Query: {}", query);
        let txn = index.read_txn()?;
        let start = Instant::now();
        let mut ctx = SearchContext::new(&index, &txn)?;
        let universe = filtered_universe(ctx.index, ctx.txn, &None)?;

        let docs = execute_search(
            &mut ctx,
            (!query.trim().is_empty()).then(|| query.trim()),
            TermsMatchingStrategy::Last,
            milli::score_details::ScoringStrategy::Skip,
            false,
            universe,
            &None,
            &None,
            GeoSortStrategy::default(),
            0,
            20,
            None,
            &mut DefaultSearchLogger,
            logger,
            TimeBudget::max(),
            None,
            None,
        )?;

        let elapsed = start.elapsed();
        println!("completed in {}us, docids: {:?}", elapsed.as_micros(), docs.documents_ids);

        let documents = index
            .documents(&txn, docs.documents_ids.iter().copied())
            .unwrap()
            .into_iter()
            .map(|(id, obkv)| {
                let mut object = serde_json::Map::default();
                for (fid, fid_name) in index.fields_ids_map(&txn).unwrap().iter() {
                    if let Some(value) = obkv.get(fid) {
                        let value: serde_json::Value = serde_json::from_slice(value).unwrap();
                        object.insert(fid_name.to_owned(), value);
                    }
                }
                (id, object)
            })
            .collect::<Vec<_>>();
        drop(txn);

        for (_, document) in documents {
            println!("{}", document.get("message").unwrap());
        }
    }
}
