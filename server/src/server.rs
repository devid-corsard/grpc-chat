use std::sync::{Arc, Mutex};

use crate::{chat_server::Chat, LogoutRequest, LogoutResponse};
use crate::{Database, LoginRequest, LoginResponse};
use tonic::{transport::Server, Code, Request, Response, Status};

#[derive(Debug)]
pub struct MyChat {
    db: Mutex<Database>,
}

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

#[tonic::async_trait]
impl Chat for MyChat {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let creds = Credentials::parse(request.into_inner());
        let mut db = self
            .db
            .lock()
            .map_err(|_| Status::new(Code::Internal, "Ooops something goes wrong"))?;
        let token = db
            .login_user(creds)
            .map_err(|_| Status::new(Code::Internal, "Ooops something goes wrong"))?;
        Ok(Response::new(LoginResponse {
            token: token.to_string(),
        }))
    }
    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        Ok(Response::new(LogoutResponse::default()))
    }
}
