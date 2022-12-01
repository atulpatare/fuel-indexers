extern crate alloc;

use fuel_indexer_macros::indexer;

// noinspection RsUnresolvedReference
#[indexer(manifest = "../fuel-indexers/receipts/manifest.yaml")]
pub mod nft_indexer_module {
    pub fn handle_block_data(_data: BlockData) {

    }
}
