use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetDevicePingResponse {
    pub ok: bool,
}

pub type GetDevicePingResult = crate::APIResult<GetDevicePingResponse>;

client_macros::api!(get, "v1/device.ping", GetDevicePingResult, !);
