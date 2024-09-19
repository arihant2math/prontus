use crate::BackendError;
use client::{Bubble, BubbleStats, Message, ProntoClient, UserInfo};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ChannelUsers {
    pub page: u64,
    pub pages: u64,
    pub users: Vec<u64>,
}

pub struct AppData {
    pub user_info: UserInfo,
    pub users: HashMap<u64, UserInfo>,
    pub client: Arc<ProntoClient>,
    pub channel_list: Vec<(Bubble, Option<BubbleStats>)>,
    pub channel_users: HashMap<u64, ChannelUsers>,
    pub current_channel: u64,
    pub message_list: Vec<Message>,
    pub parent_messages: Vec<Message>
}

pub enum InnerAppState {
    Unloaded,
    Loaded(AppData),
}

impl InnerAppState {
    pub fn try_inner(&self) -> Result<&AppData, BackendError> {
        match self {
            InnerAppState::Loaded(data) => Ok(data),
            InnerAppState::Unloaded => Err(BackendError::NotLoaded),
        }
    }

    pub fn try_inner_mut(&mut self) -> Result<&mut AppData, BackendError> {
        match self {
            InnerAppState::Loaded(data) => Ok(data),
            InnerAppState::Unloaded => Err(BackendError::NotLoaded),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<RwLock<InnerAppState>>,
}

impl AppState {
    pub async fn is_loaded(&self) -> bool {
        match self.inner.read().await.deref() {
            InnerAppState::Loaded(_) => true,
            InnerAppState::Unloaded => false,
        }
    }

    pub fn inner(&self) -> &RwLock<InnerAppState> {
        &self.inner
    }

    pub fn unloaded() -> Self {
        Self {
            inner: Arc::new(RwLock::new(InnerAppState::Unloaded)),
        }
    }
}
