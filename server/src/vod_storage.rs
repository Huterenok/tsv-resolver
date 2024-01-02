use std::time::Duration;

use moka::future::Cache;
use reqwest::Url;
use tracing::debug;

#[derive(Clone)]
struct TimedVodEntry {
    response: String,
}

#[derive(Clone)]
pub struct VodStorage {
    vods: Cache<Url, Option<TimedVodEntry>>,
}

impl VodStorage {
    pub fn new(vod_lifetime: Duration, max_capacity: u64) -> Self {
        debug!(
            "Vod Storage initialized with lifetime={:?}, max_capacity={}",
            vod_lifetime, max_capacity
        );
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(vod_lifetime)
            .build();

        Self { vods: cache }
    }

    pub async fn fetch(&self, url: Url) -> anyhow::Result<String> {
        let cached = self
            .vods
            .entry(url.clone())
            .or_insert(Default::default())
            .await;

        let value = cached.value();
        match value {
            Some(entry) => {
                debug!("Fetched response for url {} from cache", url);
                Ok(entry.response.clone())
            }
            _ => {
                if value.is_some() {
                    debug!("Response for url {} is expired, refetch...", url);
                } else {
                    debug!("No response found for url {} in cache, fetching...", url);
                }
                let fetched = crate::twitch_kraken::fetch(url.clone()).await;
                match fetched {
                    Ok(resp) => {
                        debug!(
                            "Successfully fetched response for url {}, save in cache",
                            url
                        );
                        self.vods
                            .insert(
                                url,
                                Some(TimedVodEntry {
                                    response: resp.clone(),
                                }),
                            )
                            .await;
                        Ok(resp)
                    }
                    Err(e) => {
                        debug!("Fetch for url {} failed, maybe next time...", url);
                        Err(e)
                    }
                }
            }
        }
    }
}
