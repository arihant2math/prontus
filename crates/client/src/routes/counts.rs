// Example: https://stanfordohs.pronto.io/api/clients/messages/91960333/receipts/counts

// TODO: finish implementation

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Data {
    pub total_read: u64,
    pub total_unread: u64,
}

#[derive(Serialize, Deserialize)]
struct Root {
    pub data: Data,
}
