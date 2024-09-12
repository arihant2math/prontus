use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::{Auth, Theme};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum ThemeV0 {
    Light,
    Dark,
    Auto,
}

impl Default for ThemeV0 {
    fn default() -> Self {
        ThemeV0::Auto
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct AuthV0 {
    pub saved_email: Option<String>,
    pub saved_phone: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct AppearanceV0 {
    pub theme: ThemeV0
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct OptionsV0 {
    pub rich_text: bool,
    pub notifications: bool,
    pub experiments: bool
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct SettingsFileV0 {
    pub auth: AuthV0,
    pub appearance: AppearanceV0,
    pub options: OptionsV0
}

impl From<ThemeV0> for Theme {
    fn from(theme: ThemeV0) -> Self {
        match theme {
            ThemeV0::Light => Theme::Light,
            ThemeV0::Dark => Theme::Dark,
            ThemeV0::Auto => Theme::Auto,
        }
    }
}

impl From<AuthV0> for Auth {
    fn from(auth: AuthV0) -> Self {
        Auth {
            saved_email: auth.saved_email,
            saved_phone: auth.saved_phone,
            api_key: auth.api_key,
        }
    }
}


impl From<AppearanceV0> for crate::Appearance {
    fn from(appearance: AppearanceV0) -> Self {
        crate::Appearance {
            theme: appearance.theme.into(),
        }
    }
}

impl From<OptionsV0> for crate::Options {
    fn from(options: OptionsV0) -> Self {
        crate::Options {
            rich_text: options.rich_text,
            notifications: options.notifications,
            experiments: options.experiments,
        }
    }
}

impl From<SettingsFileV0> for crate::Settings {
    fn from(settings: SettingsFileV0) -> Self {
        crate::Settings {
            auth: settings.auth.into(),
            appearance: settings.appearance.into(),
            options: settings.options.into(),
        }
    }
}

impl From<Theme> for ThemeV0 {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => ThemeV0::Light,
            Theme::Dark => ThemeV0::Dark,
            Theme::Auto => ThemeV0::Auto,
        }
    }
}

impl From<Auth> for AuthV0 {
    fn from(auth: Auth) -> Self {
        AuthV0 {
            saved_email: auth.saved_email,
            saved_phone: auth.saved_phone,
            api_key: auth.api_key,
        }
    }
}

impl From<crate::Appearance> for AppearanceV0 {
    fn from(appearance: crate::Appearance) -> Self {
        AppearanceV0 {
            theme: appearance.theme.into(),
        }
    }
}

impl From<crate::Options> for OptionsV0 {
    fn from(options: crate::Options) -> Self {
        OptionsV0 {
            rich_text: options.rich_text,
            notifications: options.notifications,
            experiments: options.experiments,
        }
    }
}

impl From<crate::Settings> for SettingsFileV0 {
    fn from(settings: crate::Settings) -> Self {
        SettingsFileV0 {
            auth: settings.auth.into(),
            appearance: settings.appearance.into(),
            options: settings.options.into(),
        }
    }
}
