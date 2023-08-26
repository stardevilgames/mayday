use hash_rings::carp::Ring;
use crate::nodes::{Node as ServerNode, NODE_MAP, node_map};
use std::{collections::BTreeMap, sync::OnceLock};

pub static HASH_RINGS: OnceLock<BTreeMap<String, Ring<String>>> = OnceLock::new();


/// Turn the ID into a consistent Murmur3 hash,
/// then try finding it in the hash ring.
pub fn from_id(
    id: String,
    index: String,
) -> Option<&'static ServerNode> {
    let rings = HASH_RINGS.get_or_init(|| BTreeMap::new());

    if let Some(ring) = rings.get(&index) {
        let node = ring.get_node(&id);
        return node_map().get(node).copied();
    }

    None
}
