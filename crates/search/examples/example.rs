use std::error::Error;
use std::io::stdin;

use milli::{
    GeoSortStrategy, TermsMatchingStrategy, TimeBudget,
};
use search::{message_index_location, Search};
use std::io::BufRead;
use std::thread;

#[tokio::main]
async fn indexer_thread() {
    tokio::fs::create_dir_all(&message_index_location()).await.unwrap();
    let mut indexer = search::MessageIndexer::new(&message_index_location()).await;
    loop {
        let r = indexer.execute().await;
        if let Err(e) = r {
            eprintln!("Error: {:?}", e);
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tokio::fs::create_dir_all(message_index_location()).await?;
    thread::spawn(|| {
        indexer_thread();
    });
    let mut search = Search::new();
    println!("Init complete");

    loop {
        // read query from stdin
        let query = stdin().lock().lines().next().unwrap().unwrap();


        println!("Query: {}", query);
        let results = search.search(
            (!query.trim().is_empty()).then(|| query.trim()),
            TermsMatchingStrategy::Last,
            milli::score_details::ScoringStrategy::Skip,
            false,
            &None,
            &None,
            GeoSortStrategy::default(),
            0,
            20,
            None,
            TimeBudget::max(),
            None,
            None,
        )?;
        println!("Results in {} seconds", results.elapsed.as_secs_f32());
        for result in results.results {
            println!("{}: {}", result.0, result.1.get("message").unwrap());
        }
    }
}
