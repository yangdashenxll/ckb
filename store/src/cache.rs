use ckb_app_config::StoreConfig;
use ckb_types::{
    bytes::Bytes,
    core::{HeaderView, TransactionView, UncleBlockVecView},
    packed::{Byte32, ProposalShortIdVec},
};
use ckb_util::Mutex;
use lru_cache::LruCache;

pub struct StoreCache {
    pub headers: Mutex<LruCache<Byte32, HeaderView>>,
    pub cell_data: Mutex<LruCache<(Byte32, u32), (Bytes, Byte32)>>,
    pub block_proposals: Mutex<LruCache<Byte32, ProposalShortIdVec>>,
    pub block_tx_hashes: Mutex<LruCache<Byte32, Vec<Byte32>>>,
    pub block_uncles: Mutex<LruCache<Byte32, UncleBlockVecView>>,
    pub cellbase: Mutex<LruCache<Byte32, TransactionView>>,
}

impl Default for StoreCache {
    fn default() -> Self {
        StoreCache::from_config(StoreConfig::default())
    }
}

impl StoreCache {
    pub fn from_config(config: StoreConfig) -> Self {
        StoreCache {
            headers: Mutex::new(LruCache::new(config.header_cache_size)),
            cell_data: Mutex::new(LruCache::new(config.cell_data_cache_size)),
            block_proposals: Mutex::new(LruCache::new(config.block_proposals_cache_size)),
            block_tx_hashes: Mutex::new(LruCache::new(config.block_tx_hashes_cache_size)),
            block_uncles: Mutex::new(LruCache::new(config.block_uncles_cache_size)),
            cellbase: Mutex::new(LruCache::new(config.cellbase_cache_size)),
        }
    }
}
