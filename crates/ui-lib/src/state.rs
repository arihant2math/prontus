use client::{
    Announcement, Bubble, BubbleStats, Membership, Message, ProntoClient, Task, UserInfo,
};
use dashmap::DashMap;
use settings::Settings;
use std::cell::UnsafeCell;
use std::sync::{
    atomic::AtomicBool,
    Arc,
    RwLock
};
use thiserror::Error;

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
    pub users: DashMap<u64, UserInfo>,
    pub client: Arc<ProntoClient>,
    pub channel_list: RwLock<Vec<(Bubble, Option<BubbleStats>, Option<Membership>)>>,
    pub channel_users: DashMap<u64, ChannelUsers>,
    pub current_channel: RwLock<Bubble>,
    pub message_list: RwLock<Vec<Message>>,
    pub parent_messages: RwLock<Vec<Message>>,
    pub announcements: RwLock<Vec<Announcement>>,
    pub tasks: RwLock<Vec<Task>>,
    // TODO: include thread id too
    pub typing_users: DashMap<u64, Vec<u64>>,
    pub is_typing: AtomicBool,
    pub settings: RwLock<Settings>,
}

pub enum InnerAppState {
    Unloaded,
    Loaded(AppData),
}

/// AppStateV2 is a non-bottlenecked version of AppState
#[derive(Clone)]
pub struct AppState {
    pub loaded: Arc<AtomicBool>,
    inner: Arc<UnsafeCell<InnerAppState>>,
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

impl AppState {
    pub fn unloaded() -> Self {
        Self {
            loaded: Arc::new(AtomicBool::new(false)),
            inner: Arc::new(UnsafeCell::new(InnerAppState::Unloaded)),
        }
    }

    pub async fn is_loaded(&self) -> bool {
        self.loaded.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn try_inner(&self) -> Result<&AppData, UnlockError> {
        if self.loaded.load(std::sync::atomic::Ordering::Relaxed) {
            match unsafe { &*self.inner.get() } {
                InnerAppState::Loaded(data) => Ok(data),
                InnerAppState::Unloaded => Err(UnlockError::NotLoaded),
            }
        } else {
            Err(UnlockError::NotLoaded)
        }
    }

    pub fn load(&self, data: AppData) {
        unsafe {
            *self.inner.get() = InnerAppState::Loaded(data);
        }
        self.loaded.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
