use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;
use client::{Bubble, BubbleStats, Message, ProntoClient, UserInfo};
use crate::BackendError;

pub struct AppData {
    pub user_info: UserInfo,
    pub users: HashMap<u64, UserInfo>,
    pub client: Arc<ProntoClient>,
    pub channel_list: Vec<(Bubble, BubbleStats)>,
    pub current_channel: u64,
    pub message_list: Vec<Message>
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
    pub inner: Arc<RwLock<InnerAppState>>
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
            inner: Arc::new(RwLock::new(InnerAppState::Unloaded))
        }
    }
}