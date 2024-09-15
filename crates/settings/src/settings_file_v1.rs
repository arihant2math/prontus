use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::{Auth, Theme};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum ThemeV1 {
    Light,
    Dark,
    Auto,
}

impl Default for ThemeV1 {
    fn default() -> Self {
        ThemeV1::Auto
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct SidebarV1 {
    pub show_dm_profile_pictures: bool,
    pub hide_categories: bool,
    pub hide_recents_categories: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct AuthV1 {
    pub saved_email: Option<String>,
    pub saved_phone: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct AppearanceV1 {
    pub theme: ThemeV1,
    pub sidebar: SidebarV1,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct OptionsV1 {
    pub rich_text: bool,
    pub notifications: bool,
    pub experiments: bool,
    pub error_reporting: bool,
    pub analytics: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct SettingsFileV1 {
    pub auth: AuthV1,
    pub appearance: AppearanceV1,
    pub options: OptionsV1
}

impl From<ThemeV1> for Theme {
    fn from(theme: ThemeV1) -> Self {
        match theme {
            ThemeV1::Light => Theme::Light,
            ThemeV1::Dark => Theme::Dark,
            ThemeV1::Auto => Theme::Auto,
        }
    }
}

impl From<SidebarV1> for crate::Sidebar {
    fn from(sidebar: SidebarV1) -> Self {
        crate::Sidebar {
            show_dm_profile_pictures: sidebar.show_dm_profile_pictures,
            hide_categories: sidebar.hide_categories,
            hide_recents_categories: sidebar.hide_recents_categories,
        }
    }
}

impl From<AuthV1> for Auth {
    fn from(auth: AuthV1) -> Self {
        Auth {
            saved_email: auth.saved_email,
            saved_phone: auth.saved_phone,
            api_key: auth.api_key,
        }
    }
}


impl From<AppearanceV1> for crate::Appearance {
    fn from(appearance: AppearanceV1) -> Self {
        crate::Appearance {
            theme: appearance.theme.into(),
            sidebar: appearance.sidebar.into(),
        }
    }
}

impl From<OptionsV1> for crate::Options {
    fn from(options: OptionsV1) -> Self {
        crate::Options {
            rich_text: options.rich_text,
            notifications: options.notifications,
            experiments: options.experiments,
            error_reporting: options.error_reporting,
            analytics: options.analytics,
        }
    }
}

impl From<SettingsFileV1> for crate::Settings {
    fn from(settings: SettingsFileV1) -> Self {
        crate::Settings {
            auth: settings.auth.into(),
            appearance: settings.appearance.into(),
            options: settings.options.into(),
        }
    }
}

impl From<Theme> for ThemeV1 {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Self::Light,
            Theme::Dark => Self::Dark,
            Theme::Auto => Self::Auto,
        }
    }
}

impl From<crate::Sidebar> for SidebarV1 {
    fn from(sidebar: crate::Sidebar) -> Self {
        Self {
            show_dm_profile_pictures: sidebar.show_dm_profile_pictures,
            hide_categories: sidebar.hide_categories,
            hide_recents_categories: sidebar.hide_recents_categories,
        }
    }
}

impl From<crate::Auth> for AuthV1 {
    fn from(auth: crate::Auth) -> Self {
        Self {
            saved_email: auth.saved_email,
            saved_phone: auth.saved_phone,
            api_key: auth.api_key,
        }
    }
}

impl From<crate::Appearance> for AppearanceV1 {
    fn from(appearance: crate::Appearance) -> Self {
        Self {
            theme: appearance.theme.into(),
            sidebar: appearance.sidebar.into(),
        }
    }
}

impl From<crate::Options> for OptionsV1 {
    fn from(options: crate::Options) -> Self {
        Self {
            rich_text: options.rich_text,
            notifications: options.notifications,
            experiments: options.experiments,
            error_reporting: options.error_reporting,
            analytics: options.analytics,
        }
    }
}

impl From<crate::Settings> for SettingsFileV1 {
    fn from(settings: crate::Settings) -> Self {
        Self {
            auth: settings.auth.into(),
            appearance: settings.appearance.into(),
            options: settings.options.into(),
        }
    }
}
