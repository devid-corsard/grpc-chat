use std::{collections::HashMap, sync::Mutex};

use rpc_chat::server::MyChat;
use tonic::transport::{Channel, Endpoint, Server, Uri};

use rpc_chat::{
    chat::{chat_client::ChatClient, chat_server::ChatServer},
    data::Database,
};

pub struct TestApp {
    pub client: ChatClient<Channel>,
}

pub async fn spawn_app() -> TestApp {
    let (client, server) = tokio::io::duplex(1024);

    let chat = MyChat {
        db: Mutex::new(Database {
            users: HashMap::new(),
            sessions: HashMap::new(),
        }),
    };

    tokio::spawn(async move {
        Server::builder()
            .add_service(ChatServer::new(chat))
            .serve_with_incoming(tokio_stream::iter(vec![Ok::<_, std::io::Error>(server)]))
            .await
    });

    // Move client to an option so we can _move_ the inner value
    // on the first attempt to connect. All other attempts will fail.
    let mut client = Some(client);
    let channel = Endpoint::try_from("http://[::]:50051")
        .expect("Failed create Endpoint")
        .connect_with_connector(tower::service_fn(move |_: Uri| {
            let client = client.take();

            async move {
                if let Some(client) = client {
                    Ok(client)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Client already taken",
                    ))
                }
            }
        }))
        .await
        .expect("Failed create channel");

    let client = ChatClient::new(channel);

    TestApp { client }
}
