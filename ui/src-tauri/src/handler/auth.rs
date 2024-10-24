use crate::BackendError;
use client::user_login::{DeviceInfo, UserLoginRequest};
use client::ProntoClient;
use log::info;
use settings::Settings;
use tauri::command;

#[command]
pub async fn get_code(email: String) -> Result<(), BackendError> {
    let _response = client::routes::user_verify::post(
        client::routes::user_verify::UserVerifyRequest::Email(email),
    )
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
            browsername: "Firefox".to_string(),
            browserversion: "".to_string(),
            osname: "Windows".to_string(),
            r#type: "WEB".to_string(),
        },
    })
    .await
    .unwrap()
    .to_result()
    .unwrap();
    let token = &response.users[0].login_token;
    // "https://stanfordohs.pronto.io/api/"
    let base_url = format!(
        "https://{}.pronto.io/api/",
        response.users[0].user.organizations[0].shortname
    );
    let mut settings = Settings::load().await?;
    let auth = settings::Auth {
        base_url: base_url.clone(),
        api_key: token.clone(),
        saved_email: None,
        saved_phone: None,
    };
    settings.auth = Some(auth);
    settings.save().await?;

    let client = ProntoClient::new(base_url.clone(), token).unwrap();
    // TODO: Standardize device info
    let response = client.user_token_login(token).await?;

    info!("Login successful");

    let mut settings = Settings::load().await?;
    settings
        .auth
        .as_mut()
        .ok_or(BackendError::NotAuthenticated)?
        .api_key = response.users[0].access_token.clone();
    settings.save().await?;
    // TODO: Error handling as usual
    Ok(())
}
