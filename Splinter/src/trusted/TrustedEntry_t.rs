use builtin_macros::*;

#[allow(unused_imports)]
use builtin::*; // needed for Ghost, but that gets erased.

use crate::trusted::KVStoreTrait_t::*;
use crate::trusted::ClientAPI_t::*;

// Provides an entry point that enforces application of the KVStoreTrait,
// which is how we ensure the implementation calling the API corresponds
// to the refinement proof that belongs with it.

verus!{
broadcast use{vstd::seq_lib::group_seq_properties,
              vstd::map_lib::group_map_properties,
              vstd::set_lib::group_set_properties,
              vstd::multiset::group_multiset_properties};
    pub fn entry<KVStore: KVStoreTrait>() {
        let mut kvstore = KVStore::new();
        let api = ClientAPI::new(Ghost(kvstore.instance_id()));
        kvstore.kvstore_main(api);
    }
}

