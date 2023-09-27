use rpc_chat::{chat::chat_server::ChatServer, service::MyChat};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("ChatServer listening on: {}", addr);

    let chat = MyChat::default();

    let svc = ChatServer::new(chat);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
