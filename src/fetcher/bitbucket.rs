use crate::{
    common::{SimpleFetcher, SimpleUrlFetcher},
    impl_fetcher,
};

pub struct FetchFromBitBucket;
impl_fetcher!(FetchFromBitBucket);

impl<'a> SimpleFetcher<'a> for FetchFromBitBucket {
    const NAME: &'static str = "fetchFromBitBucket";

    fn host(&'a self) -> Option<&'a str> {
        None
    }
}

impl<'a> SimpleUrlFetcher<'a> for FetchFromBitBucket {
    fn get_url(&self, owner: &str, repo: &str, rev: &str) -> String {
        format!("https://bitbucket.org/{owner}/{repo}/get/{rev}.tar.gz")
    }
}
