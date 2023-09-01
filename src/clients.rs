use std::sync::OnceLock;
use tokio::net::TcpStream;
use ulid::Ulid;


/// A foreign node. Used for connections to other servers,
/// NOT users.
#[derive(Debug)]
pub struct Node {
    /// The address of this node.
    pub addr: String,
    /// The internal stream where client communication with this node is happening.
    stream: TcpStream
}

static NODE_ID: OnceLock<String> = OnceLock::new();

pub fn node_id() -> &'static String {
    return NODE_ID.get_or_init(|| {
        let uid = Ulid::new();
        uid.to_string()
    })
}
