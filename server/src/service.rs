use std::collections::HashMap;
use std::sync::Mutex;

use crate::chat::{chat_server::Chat, LogoutRequest, LogoutResponse};
use crate::chat::{ChatUser, LoginRequest, LoginResponse, MessageBody, MessageStatus, Users, Void};
use crate::data::Database;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Code, Request, Response, Status};

#[derive(Debug)]
pub struct Credentials {
    pub name: String,
    pub password: String,
}

impl Credentials {
    fn parse(req: LoginRequest) -> Self {
        Self {
            name: req.name,
            password: req.password,
        }
    }
}

#[derive(Debug)]
pub struct MyChat {
    pub db: Mutex<Database>,
}

impl Default for MyChat {
    fn default() -> Self {
        Self {
            db: Mutex::new(Database {
                users: HashMap::new(),
                sessions: HashMap::new(),
            }),
        }
    }
}

#[tonic::async_trait]
impl Chat for MyChat {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let creds = Credentials::parse(request.into_inner());
        let mut db = self.db.lock().map_err(internal)?;
        let token = db.login_user(creds).map_err(internal)?;
        Ok(Response::new(LoginResponse {
            token: token.to_string(),
        }))
    }

    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        let token = request.into_inner().token.try_into().map_err(external)?;
        let mut db = self.db.lock().map_err(internal)?;
        db.logout_user(&token).map_err(internal)?;
        Ok(Response::new(LogoutResponse::default()))
    }

    async fn list_users(&self, _: Request<Void>) -> Result<Response<Users>, Status> {
        let db = self.db.lock().map_err(internal)?;
        let users = db.list_all_users().map_err(internal)?;
        let users = users
            .into_iter()
            .map(|u| ChatUser {
                name: u.name,
                logged_in: u.logged_in,
            })
            .collect();
        Ok(Response::new(Users { users }))
    }

    type SendMessageStream = ReceiverStream<Result<MessageStatus, Status>>;

    async fn send_message(
        &self,
        _request: Request<MessageBody>,
    ) -> Result<Response<Self::SendMessageStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        dbg!(_request);
        let mut message_status = MessageStatus::default();

        tokio::spawn(async move {
            for _ in 0..3 {
                if message_status.sended == false {
                    message_status.sended = true;
                    tx.send(Ok(message_status.clone()))
                        .await
                        .expect("Failed to send mess status");
                    println!(" /// Message sended");
                    continue;
                }
                if message_status.delivered == false {
                    message_status.delivered = true;
                    tx.send(Ok(message_status.clone()))
                        .await
                        .expect("Failed to send mess status");
                    println!(" /// Message delivered");
                    continue;
                }
                if message_status.readed == false {
                    message_status.readed = true;
                    tx.send(Ok(message_status.clone()))
                        .await
                        .expect("Failed to send mess status");
                    println!(" /// Message readed");
                    continue;
                }
            }

            println!(" /// done sending");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

fn internal<T>(_e: T) -> Status
where
    T: std::fmt::Debug + std::fmt::Display,
{
    Status::new(Code::Internal, "Ooops something goes wrong")
}

fn external<T>(e: T) -> Status
where
    T: std::fmt::Debug + std::fmt::Display + Into<String>,
{
    Status::new(Code::InvalidArgument, e)
}
