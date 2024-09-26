use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherAuthRequest {
    pub socket_id: String,
    pub channel_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherAuthResponse {
    pub auth: String,
}

client_macros::api!(post, "v1/pusher.auth", PusherAuthResponse, PusherAuthRequest);
