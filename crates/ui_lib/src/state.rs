use client::{
    Announcement, Bubble, BubbleStats, Membership, Message, ProntoClient, Task, UserInfo,
};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Copy, Clone, Debug, Error)]
pub enum UnlockError {
    #[error("Not loaded")]
    NotLoaded,
}

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
    pub channel_list: Vec<(Bubble, Option<BubbleStats>, Option<Membership>)>,
    pub channel_users: HashMap<u64, ChannelUsers>,
    pub current_channel: Bubble,
    pub message_list: Vec<Message>,
    pub parent_messages: Vec<Message>,
    pub announcements: Vec<Announcement>,
    pub tasks: Vec<Task>,
}

pub enum InnerAppState {
    Unloaded,
    Loaded(AppData),
}

impl InnerAppState {
    pub fn try_inner(&self) -> Result<&AppData, UnlockError> {
        match self {
            InnerAppState::Loaded(data) => Ok(data),
            InnerAppState::Unloaded => Err(UnlockError::NotLoaded),
        }
    }

    pub fn try_inner_mut(&mut self) -> Result<&mut AppData, UnlockError> {
        match self {
            InnerAppState::Loaded(data) => Ok(data),
            InnerAppState::Unloaded => Err(UnlockError::NotLoaded),
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
