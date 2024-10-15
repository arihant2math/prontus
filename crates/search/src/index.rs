use std::path::PathBuf;
use heed::EnvOpenOptions;

pub trait Index {
    type Error;
    const index_location: PathBuf;

    fn index(&self, location: &PathBuf) -> milli::Result<milli::Index> {
        let mut options = EnvOpenOptions::new();
        options.map_size(128 * 1024 * 1024 * 1024); // 100 GB

        milli::Index::new(options, location.to_str().unwrap())
    }

    async fn init(&self) -> Result<(), Self::Error>;

    async fn execute(&self) -> Result<(), Self::Error>;
}
