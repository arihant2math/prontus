use client::ProntoClient;
use client::user_login::{DeviceInfo, UserLoginRequest};
use settings::Settings;
use tauri::command;
use crate::BackendError;

#[command]
pub async fn get_code(email: String) -> Result<(), BackendError> {
    let _response = client::routes::user_verify::post(client::routes::user_verify::UserVerifyRequest::Email(email))
        .await
        .unwrap()
        .to_result();
    // TODO: Error handling
    Ok(())
}

#[command]
pub async fn send_code(email: String, code: String) -> Result<(), BackendError> {
    let response = client::routes::user_login::post(UserLoginRequest {
        email,
        code,
        // TODO: Fix
        device: DeviceInfo {
            browsername: "".to_string(),
            browserversion: "".to_string(),
            osname: "".to_string(),
            r#type: "".to_string(),
        },
    })
        .await
        .unwrap()
        .to_result()
        .unwrap();
    let token = &response.users[0].login_token;
    let mut settings = Settings::load().await?;
    settings.auth.api_key = Some(token.clone());
    settings.save().await?;
    // TODO: This is the part where we can switch base urls
    let client =
        ProntoClient::new("https://stanfordohs.pronto.io/api/".to_string(), token).unwrap();
    // TODO: Standardize device info
    let response = client
        .user_token_login(
            token,
        )
        .await?;

    println!("Login successful");

    let mut settings = Settings::load().await?;
    settings.auth.api_key = Some(response.users[0].access_token.clone());
    settings.save().await?;
    // TODO: Error handling as usual
    Ok(())
}
