use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum AccountStatus {
    #[default]
    NotVerified,
    Valid,
    Invalid,
    Requires2FA,
}

impl AccountStatus {
    pub fn label(&self) -> &str {
        match self {
            AccountStatus::NotVerified => "Not Verified",
            AccountStatus::Valid => "Valid",
            AccountStatus::Invalid => "Invalid",
            AccountStatus::Requires2FA => "2FA Required",
        }
    }
    
    pub fn color(&self) -> egui::Color32 {
        match self {
            AccountStatus::NotVerified => egui::Color32::from_rgb(120, 120, 130),
            AccountStatus::Valid => egui::Color32::from_rgb(80, 200, 120),
            AccountStatus::Invalid => egui::Color32::from_rgb(220, 80, 80),
            AccountStatus::Requires2FA => egui::Color32::from_rgb(230, 180, 80),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub enum UserPresenceType {
    #[default]
    Offline,
    Online,
    InGame,
    InStudio,
}

impl UserPresenceType {
    pub fn label(&self) -> &str {
        match self {
            UserPresenceType::Offline => "Offline",
            UserPresenceType::Online => "Online",
            UserPresenceType::InGame => "In Game",
            UserPresenceType::InStudio => "In Studio",
        }
    }
    
    pub fn color(&self) -> egui::Color32 {
        match self {
            UserPresenceType::Offline => egui::Color32::from_rgb(120, 120, 130),
            UserPresenceType::Online => egui::Color32::from_rgb(0, 162, 255),    // Blue
            UserPresenceType::InGame => egui::Color32::from_rgb(2, 183, 87),     // Green
            UserPresenceType::InStudio => egui::Color32::from_rgb(70, 41, 216),  // Purple
        }
    }
    
    pub fn from_int(val: u8) -> Self {
        match val {
            1 => UserPresenceType::Online,
            2 => UserPresenceType::InGame,
            3 => UserPresenceType::InStudio,
            _ => UserPresenceType::Offline,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct UserPresence {
    pub presence_type: UserPresenceType,
    pub last_location: Option<String>,
    pub place_id: Option<u64>,
    pub game_id: Option<String>,
    pub last_online: Option<String>,
    #[serde(skip)]
    pub game_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RobloxAccount {
    pub username: String,
    pub password: String,
    pub cookie: Option<String>,
    pub user_id: Option<u64>,
    pub display_name: Option<String>,
    pub last_login: Option<String>,
    pub status: AccountStatus,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub group: String,
    #[serde(default)]
    pub robux: Option<i64>,
    #[serde(default)]
    pub friends_count: Option<u32>,
    #[serde(default)]
    pub is_premium: Option<bool>,
    #[serde(default)]
    pub collectibles_count: Option<u32>,
    #[serde(default)]
    pub last_info_fetch: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(skip)]
    pub presence: Option<UserPresence>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RecentGame {
    pub place_id: String,
    pub name: String,
    pub last_played: String,
    pub play_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UserGame {
    pub place_id: String,
    pub name: String,
    pub universe_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AppData {
    pub accounts: Vec<RobloxAccount>,
    pub last_place_id: String,
    #[serde(default)]
    pub favorite_games: Vec<String>,
    #[serde(default)]
    pub multi_instance_enabled: bool,
    #[serde(default)]
    pub recent_games: Vec<RecentGame>,
    #[serde(default)]
    pub user_games: Vec<UserGame>,
    #[serde(default)]
    pub auto_refresh_cookies: bool,
    #[serde(default)]
    pub batch_launch_delay: u32,
    #[serde(default)]
    pub minimize_to_tray: bool,
}

impl AppData {
    pub fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("NexusAccountManager");
        fs::create_dir_all(&path).ok();
        path.push("accounts.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|data| serde_json::from_str(&data).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Ok(data) = serde_json::to_string_pretty(self) {
            fs::write(path, data).ok();
        }
    }
}
