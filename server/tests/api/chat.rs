use rpc_chat::chat::LoginRequest;
use tonic::Request;

use crate::helpers::spawn_app;

#[tokio::test]
async fn user_can_login_and_gets_token() {
    let mut app = spawn_app().await;
    let response = app
        .client
        .login(Request::new(LoginRequest {
            name: uuid::Uuid::new_v4().to_string(),
            password: uuid::Uuid::new_v4().to_string(),
        }))
        .await
        .expect("Failed to get response");
    let session_id = uuid::Uuid::try_parse(response.get_ref().token.as_str()).unwrap();
    assert!(!session_id.is_nil());
}
