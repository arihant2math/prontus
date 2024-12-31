mod index;
mod message_index;

use crate::message_index::get_index;
pub use message_index::{IndexerSettings, MessageIndexer};
pub use milli;
use milli::score_details::ScoringStrategy;
use milli::tokenizer::Language;
use milli::{
    execute_search, filtered_universe, AscDesc, DefaultSearchLogger, DocumentId, GeoSortStrategy,
    Index, SearchContext, TermsMatchingStrategy, TimeBudget,
};
use serde_json::{Map, Value};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use thiserror::Error;

pub struct Search {
    index: Index,
    logger: DefaultSearchLogger,
}

pub struct SearchResults {
    pub results: Vec<(DocumentId, Map<String, Value>)>,
    pub elapsed: Duration,
}

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Serde json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Heed error: {0}")]
    HeedError(#[from] heed::Error),
    #[error("Milli error: {0}")]
    MilliError(#[from] milli::Error),
}

impl Search {
    pub fn new(index_path: &PathBuf) -> Self {
        Search {
            index: get_index(index_path).unwrap(),
            logger: DefaultSearchLogger,
        }
    }

    pub fn search(
        &mut self,
        query: Option<&str>,
        terms_matching_strategy: TermsMatchingStrategy,
        scoring_strategy: ScoringStrategy,
        exhaustive_number_hits: bool,
        sort_criteria: &Option<Vec<AscDesc>>,
        distinct: &Option<String>,
        geo_strategy: GeoSortStrategy,
        from: usize,
        length: usize,
        words_limit: Option<usize>,
        time_budget: TimeBudget,
        ranking_score_threshold: Option<f64>,
        locales: Option<&Vec<Language>>,
    ) -> Result<SearchResults, SearchError> {
        let txn = self.index.read_txn()?;
        let start = Instant::now();
        let mut ctx = SearchContext::new(&self.index, &txn)?;
        let universe = filtered_universe(ctx.index, ctx.txn, &None)?;
        let docs = execute_search(
            &mut ctx,
            query,
            terms_matching_strategy,
            scoring_strategy,
            exhaustive_number_hits,
            universe,
            sort_criteria,
            distinct,
            geo_strategy,
            from,
            length,
            words_limit,
            &mut DefaultSearchLogger,
            &mut self.logger,
            time_budget,
            ranking_score_threshold,
            locales,
        )?;
        let elapsed = start.elapsed();
        let documents = self
            .index
            .documents(&txn, docs.documents_ids.iter().copied())
            .unwrap()
            .into_iter()
            .map(|(id, obkv)| {
                let mut object = serde_json::Map::default();
                for (fid, fid_name) in self.index.fields_ids_map(&txn).unwrap().iter() {
                    if let Some(value) = obkv.get(fid) {
                        let value: Value = serde_json::from_slice(value).unwrap();
                        object.insert(fid_name.to_owned(), value);
                    }
                }
                (id, object)
            })
            .collect::<Vec<_>>();
        drop(txn);
        Ok(SearchResults {
            results: documents,
            elapsed,
        })
    }
}
