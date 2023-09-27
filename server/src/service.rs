use std::collections::HashMap;
use std::sync::Mutex;

use crate::chat::{chat_server::Chat, LogoutRequest, LogoutResponse};
use crate::chat::{ChatUser, LoginRequest, LoginResponse, Users, Void};
use crate::data::Database;
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
