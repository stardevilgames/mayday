use std::sync::OnceLock;
use tokio::net::TcpStream;
use std::collections::BTreeMap;


/// A foreign node. Used for connections to other servers,
/// NOT users.
#[derive(Debug)]
pub struct Node {
    /// The address of this node.
    pub addr: String,
    /// The internal stream where client communication with this node is happening.
    stream: TcpStream
}


pub static FOREIGN_NODES: OnceLock<Vec<Node>> = OnceLock::new();
pub static NODE_MAP: OnceLock<BTreeMap<String, &Node>> = OnceLock::new();


pub fn foreign_nodes() -> &'static Vec<Node> {
    FOREIGN_NODES.get_or_init(|| Vec::new())
}


pub fn node_map() -> &'static BTreeMap<String, &'static Node> {
    NODE_MAP.get_or_init(|| BTreeMap::new())
}
