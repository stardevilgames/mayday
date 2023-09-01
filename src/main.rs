mod clients;
mod server;
mod errors;
mod messages;
mod gen_server;
use server::Server;


#[tokio::main]
async fn main() {
    // TODO: support for daemonizing
    let serv = Server::new();

    serv.start().await.unwrap();
}
