use crate::pusher_auth::PusherAuthRequest;
use crate::user_token_login::TokenLoginResponse;
use crate::{pusher_auth, user_token_login, ProntoClient, ResponseError};

impl ProntoClient {
    pub async fn user_token_login(&self, token: &str) -> Result<TokenLoginResponse, ResponseError> {
        // TODO: pass in device info
        Ok(user_token_login::post(
            &self.api_base_url,
            &self.http_client,
            vec![token.to_string()],
        )
        .await?
        .to_result()?)
    }

    pub async fn pusher_auth(
        &self,
        socket_id: &str,
        channel_name: &str,
    ) -> Result<pusher_auth::PusherAuthResponse, ResponseError> {
        Ok(pusher_auth::post(
            &self.api_base_url,
            &self.http_client,
            PusherAuthRequest {
                socket_id: socket_id.to_string(),
                channel_name: channel_name.to_string(),
            },
        )
        .await?)
    }
}
