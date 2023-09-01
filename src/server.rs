use std::{collections::BTreeMap, sync::OnceLock};
use crate::{errors::Result, clients::Node, gen_server::PID};


#[derive(Debug)]
pub struct Server {
    connected_nodes: BTreeMap<String, Node>,
}


impl Server {
    pub fn new() -> Self {
        return Server {
            connected_nodes: BTreeMap::new(),
        }
    }

    pub async fn start(&self) -> Result<()> {
        Ok(())
    }
}


static SERVER: OnceLock<Server> = OnceLock::new();


pub fn server() -> &'static Server {
    SERVER.get_or_init(|| Server::new())
}


pub async fn send(message: Vec<u8>, pid: PID) {
    todo!()
}


pub async fn call_back(message: Vec<u8>, request_id: String, pid: PID) {
    todo!()
}
