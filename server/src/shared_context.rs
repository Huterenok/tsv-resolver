use std::{sync::Arc, time::Duration};

use crate::vod_storage::VodStorage;

pub struct SharedContext {
    pub vod_storage: VodStorage,
}

impl SharedContext {
    pub fn new(vod_lifetime: Duration, vod_max_capacity: u64) -> Arc<Self> {
        Arc::new(Self {
            vod_storage: VodStorage::new(vod_lifetime, vod_max_capacity),
        })
    }
}
