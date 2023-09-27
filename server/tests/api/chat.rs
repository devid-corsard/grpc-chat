use rpc_chat::chat::{LoginRequest, LogoutRequest, MessageBody, MessageStatus, Void};
use tonic::Request;

use crate::helpers::spawn_app;

#[tokio::test]
async fn user_can_login_and_gets_token() {
    let mut app = spawn_app().await;
    let name = uuid::Uuid::new_v4().to_string();
    let password = uuid::Uuid::new_v4().to_string();
    let response = app
        .client
        .login(Request::new(LoginRequest {
            name: name.clone(),
            password,
        }))
        .await
        .expect("Failed to get response");
    let session_id = uuid::Uuid::try_parse(response.get_ref().token.as_str()).unwrap();
    dbg!(&session_id);
    assert!(!session_id.is_nil());
}

#[tokio::test]
async fn after_login_user_set_as_logged_in_in_the_db() {
    let mut app = spawn_app().await;
    let name = uuid::Uuid::new_v4().to_string();
    let password = uuid::Uuid::new_v4().to_string();
    app.client
        .login(Request::new(LoginRequest {
            name: name.clone(),
            password,
        }))
        .await
        .expect("Failed to get response");
    let response = app
        .client
        .list_users(Void::default())
        .await
        .expect("Failed to get response");
    let users = response.into_inner().users;
    assert_eq!(name, users[0].name);
    assert_eq!(true, users[0].logged_in);
}

#[tokio::test]
async fn user_can_logout() {
    let mut app = spawn_app().await;
    let name = uuid::Uuid::new_v4().to_string();
    let password = uuid::Uuid::new_v4().to_string();
    let response = app
        .client
        .login(Request::new(LoginRequest {
            name: name.clone(),
            password,
        }))
        .await
        .expect("Failed to get response");
    let session_id = response.into_inner().token;
    app.client
        .logout(LogoutRequest { token: session_id })
        .await
        .expect("Failed to get the logout response");
    let response = app
        .client
        .list_users(Void::default())
        .await
        .expect("Failed to get response");
    let users = response.into_inner().users;
    assert_eq!(name, users[0].name);
    assert_eq!(false, users[0].logged_in);
}

#[tokio::test]
async fn logged_in_user_can_send_a_message_to_another_user() {
    let mut app = spawn_app().await;
    let name = uuid::Uuid::new_v4().to_string();
    let password = uuid::Uuid::new_v4().to_string();
    let response = app
        .client
        .login(Request::new(LoginRequest {
            name: name.clone(),
            password,
        }))
        .await
        .expect("Failed to get response");
    let session_id = response.into_inner().token;
    //new message
    let message = MessageBody {
        token: session_id,
        body: "test message".into(),
        reciever_id: uuid::Uuid::new_v4().to_string(),
        timestamp: Some(prost_types::Timestamp::default()),
    };
    let mut stream = app
        .client
        .send_message(message)
        .await
        .expect("Failed to send message")
        .into_inner();
    let mut streamed_message_statuses = Vec::<MessageStatus>::new();

    while let Some(status) = stream.message().await.expect("Failed to await stream") {
        dbg!(&status);
        streamed_message_statuses.push(status);
    }
    assert_eq!(true, streamed_message_statuses[2].readed);
    assert_eq!(true, streamed_message_statuses[2].sended);
    assert_eq!(true, streamed_message_statuses[2].delivered);
}
