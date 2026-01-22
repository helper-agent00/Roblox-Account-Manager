// Main UI module

mod account_list;
mod add_account;
mod import_cookie;
mod games_tab;
mod server_browser_tab;
mod account_utils_tab;

use eframe::egui::{self, RichText};
use crate::account::{AccountStatus, AppData, RecentGame, RobloxAccount, UserGame};
use crate::api::{RobloxApi, ServerBrowser};
use crate::auth::{CookieFinder, FoundCookie, MultiInstanceManager};
use crate::theme::Colors;
use std::collections::HashSet;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum Tab {
    #[default]
    Accounts,
    AddAccount,
    Games,
    Servers,
    AccountUtils,
    ImportCookie,
    Settings,
    About,
}

#[derive(Default)]
pub enum Action {
    #[default]
    None,
    VerifyAccount(usize),
    LaunchAccount(usize),
    DeleteAccount(usize),
    SelectAccount(usize),
    ImportCookie,
    FindCookies,
    UseCookie(String),
    SelectGame(String, String),  // place_id, game_name
    ToggleFavoriteGame(String),
    ToggleMultiInstance,
    FetchAccountInfo(usize),
    BatchLaunch,
    RefreshAllCookies,
    RefreshPresence,
    // Server browser actions
    FetchServers,
    FetchMoreServers,
    FetchVipServers,
    FetchMoreVipServers,
    JoinRandomServer(usize),  // account idx
    JoinSelectedServer(usize), // account idx - join currently selected server
    JoinPrivateServerLink(usize), // account idx - join from private server input
    JoinVipWithAccessCode, // join VIP server with manually entered access code
    // Account utilities actions
    LogoutOtherSessions(usize),
    ChangePassword(usize),
    SetDisplayName(usize),
    BlockUserByName(usize, String),     // account idx, target username
    UnblockUserByName(usize, String),
    SendFriendRequestByName(usize, String),
    // Avatar fetching
    FetchAvatars,
    // Browser login
    StartBrowserLogin,
    // Import cookie as a new account (not for existing accounts)
    ImportCookieAsNewAccount,
    // Cookie management
    ShowCookieModal(usize),
    UpdateAccountCookie(usize),
    // Follow User - join their game
    FollowUser(usize), // account idx to use for joining
    // Drag & drop cookie import
    ImportDroppedCookie(String),
    // User games
    AddUserGame(String, String), // place_id, name
    RemoveUserGame(String),      // place_id
}

pub struct NexusApp {
    pub data: AppData,
    pub tab: Tab,
    pub selected: Option<usize>,
    
    // Forms
    pub import_cookie: String,
    pub found_cookies: Vec<FoundCookie>,
    
    // Game selection
    pub place_id: String,
    pub selected_game_name: String,  // Name of the currently selected game
    
    // UI state
    pub delete_confirm: Option<usize>,
    pub status: String,
    pub status_error: bool,
    pub action: Action,
    pub game_search: String,
    
    // Multi-instance
    pub multi_instance: MultiInstanceManager,
    
    // Batch launch
    pub batch_selected: HashSet<usize>,
    #[allow(dead_code)]
    pub batch_launching: bool,
    pub batch_delay: u32,
    
    // Server Browser
    pub server_browser: ServerBrowser,
    
    // Account Utilities
    pub util_target_user: String,
    #[allow(dead_code)]
    pub util_current_password: String,
    pub util_new_password: String,
    pub util_new_display_name: String,
    
    // Avatar textures (cached in memory)
    #[allow(dead_code)]
    pub avatar_textures: std::collections::HashMap<u64, egui::TextureHandle>,
    pub avatars_loading: bool,
    
    // Game icon URLs (universe_id -> icon_url)
    pub game_icons: std::collections::HashMap<u64, String>,
    pub game_icons_loaded: bool,
    
    // Startup state and periodic refresh
    pub startup_fetch_done: bool,
    pub last_presence_refresh: std::time::Instant,
    
    // Browser login session
    pub browser_login_session: Option<crate::auth::WebLoginSession>,
    
    // Cookie management modal
    pub cookie_modal_account_idx: Option<usize>,
    pub cookie_modal_value: String,
    pub cookie_modal_show: bool,
    
    // Follow user modal
    pub follow_user_show: bool,
    pub follow_user_target: String,
    pub follow_user_account_idx: Option<usize>,
    
    // Drag & drop state
    pub drag_drop_active: bool,
    
    // Add user game state
    pub add_user_game_place_id: String,
    pub add_user_game_name: String,
    pub add_user_game_show: bool,
    
    // Tray minimize state
    pub minimized_to_tray: bool,
}

impl NexusApp {
    pub fn new() -> Self {
        let data = AppData::load();
        let place_id = data.last_place_id.clone();
        let multi_enabled = data.multi_instance_enabled;
        let batch_delay = if data.batch_launch_delay == 0 { 5 } else { data.batch_launch_delay };
        
        let mut multi_instance = MultiInstanceManager::new();
        if multi_enabled {
            let _ = multi_instance.enable();
        }
        
        Self {
            data,
            tab: Tab::Accounts,
            selected: None,
            import_cookie: String::new(),
            found_cookies: Vec::new(),
            place_id,
            selected_game_name: String::new(),
            delete_confirm: None,
            status: String::new(),
            status_error: false,
            action: Action::None,
            game_search: String::new(),
            multi_instance,
            batch_selected: HashSet::new(),
            batch_launching: false,
            batch_delay,
            server_browser: ServerBrowser::new(),
            util_target_user: String::new(),
            util_current_password: String::new(),
            util_new_password: String::new(),
            util_new_display_name: String::new(),
            avatar_textures: std::collections::HashMap::new(),
            avatars_loading: false,
            game_icons: std::collections::HashMap::new(),
            game_icons_loaded: false,
            startup_fetch_done: false,
            last_presence_refresh: std::time::Instant::now(),
            browser_login_session: None,
            cookie_modal_account_idx: None,
            cookie_modal_value: String::new(),
            cookie_modal_show: false,
            follow_user_show: false,
            follow_user_target: String::new(),
            follow_user_account_idx: None,
            drag_drop_active: false,
            add_user_game_place_id: String::new(),
            add_user_game_name: String::new(),
            add_user_game_show: false,
            minimized_to_tray: false,
        }
    }

    pub fn set_status(&mut self, msg: impl Into<String>, is_error: bool) {
        self.status = msg.into();
        self.status_error = is_error;
    }

    pub fn render_tab_button(&mut self, ui: &mut egui::Ui, tab: Tab, label: &str) {
        let is_active = self.tab == tab;
        
        let btn = egui::Button::new(
            RichText::new(label)
                .size(13.0)
                .color(if is_active { Colors::TEXT_PRIMARY } else { Colors::TEXT_MUTED })
        )
        .fill(if is_active { Colors::BG_CARD_SELECTED } else { egui::Color32::TRANSPARENT })
        .stroke(if is_active { 
            egui::Stroke::new(1.0, Colors::BORDER_ACCENT) 
        } else { 
            egui::Stroke::NONE 
        })
        .rounding(egui::Rounding::same(6.0));
        
        if ui.add(btn).clicked() {
            self.tab = tab;
        }
    }

    pub fn process_action(&mut self) {
        let action = std::mem::take(&mut self.action);
        
        match action {
            Action::None => {}
            
            Action::SelectAccount(idx) => {
                self.selected = Some(idx);
                self.delete_confirm = None;
            }
            
            Action::VerifyAccount(idx) => {
                // Get data we need first
                let account_data = self.data.accounts.get(idx).map(|a| {
                    (a.username.clone(), a.cookie.clone())
                });
                
                if let Some((username, cookie_opt)) = account_data {
                    self.set_status(format!("Verifying {}...", username), false);
                    
                    if let Some(cookie) = cookie_opt {
                        match RobloxApi::validate_cookie(&cookie) {
                            Ok((user_id, display_name)) => {
                                if let Some(account) = self.data.accounts.get_mut(idx) {
                                    account.user_id = Some(user_id);
                                    account.display_name = Some(display_name.clone());
                                    account.status = AccountStatus::Valid;
                                    account.last_login = Some(chrono::Local::now().format("%Y-%m-%d %H:%M").to_string());
                                }
                                self.data.save();
                                self.set_status(format!(" {} verified", username), false);
                            }
                            Err(e) => {
                                if let Some(account) = self.data.accounts.get_mut(idx) {
                                    account.status = AccountStatus::Invalid;
                                }
                                self.data.save();
                                self.set_status(format!("Invalid: {}", e), true);
                            }
                        }
                    } else {
                        self.set_status("No cookie - import one first", true);
                    }
                }
            }
            
            Action::LaunchAccount(idx) => {
                let account_data = self.data.accounts.get(idx).map(|a| {
                    (a.username.clone(), a.cookie.clone())
                });
                
                if let Some((username, cookie_opt)) = account_data {
                    if let Some(cookie) = cookie_opt {
                        self.set_status(format!("⏳ Launching Roblox instance: {} - please wait...", username), false);
                        
                        let place = if self.place_id.is_empty() { None } else { Some(self.place_id.clone()) };
                        
                        self.data.last_place_id = self.place_id.clone();
                        self.data.save();
                        
                        let result = if self.multi_instance.is_enabled() {
                            RobloxApi::launch_multi_instance(&cookie, place.as_deref())
                        } else {
                            RobloxApi::set_account_and_launch(&cookie, place.as_deref())
                        };
                        
                        match result {
                            Ok(_) => {
                                if !self.place_id.is_empty() {
                                    self.add_recent_game(&self.place_id.clone());
                                }
                                self.set_status(format!("✅ Launched {} - Roblox is starting up", username), false);
                            }
                            Err(e) => self.set_status(format!("Launch failed: {}", e), true),
                        }
                    }
                }
            }
            
            Action::DeleteAccount(idx) => {
                if idx < self.data.accounts.len() {
                    let name = self.data.accounts[idx].username.clone();
                    self.data.accounts.remove(idx);
                    self.data.save();
                    self.selected = None;
                    self.set_status(format!("Deleted {}", name), false);
                }
            }
            
            Action::ImportCookie => {
                let cookie = self.import_cookie.trim().to_string();
                if cookie.is_empty() {
                    self.set_status("Paste a cookie first", true);
                    return;
                }
                
                if let Some(idx) = self.selected {
                    // Get username first to avoid borrow conflict
                    let username = self.data.accounts.get(idx).map(|a| a.username.clone());
                    
                    if let Some(username) = username {
                        self.set_status(format!("Verifying cookie for {}...", username), false);
                        
                        match RobloxApi::validate_cookie(&cookie) {
                            Ok((user_id, display_name)) => {
                                if let Some(account) = self.data.accounts.get_mut(idx) {
                                    account.cookie = Some(cookie);
                                    account.user_id = Some(user_id);
                                    account.display_name = Some(display_name.clone());
                                    account.status = AccountStatus::Valid;
                                    account.last_login = Some(chrono::Local::now().format("%Y-%m-%d %H:%M").to_string());
                                }
                                self.data.save();
                                self.set_status(format!(" Cookie imported for {}", username), false);
                                self.import_cookie.clear();
                                self.tab = Tab::Accounts;
                                
                                // Fetch presence and avatar for the updated account
                                self.fetch_presence_and_avatars();
                            }
                            Err(e) => {
                                self.set_status(format!("Invalid cookie: {}", e), true);
                            }
                        }
                    }
                }
            }
            
            Action::FindCookies => {
                self.set_status("Searching...", false);
                self.found_cookies = CookieFinder::find_all_cookies();
                
                if self.found_cookies.is_empty() {
                    self.set_status("No cookies found", true);
                } else {
                    self.set_status(format!("Found {} cookie(s)", self.found_cookies.len()), false);
                }
            }
            
            Action::UseCookie(cookie) => {
                self.import_cookie = cookie;
                self.action = Action::ImportCookie;
            }
            
            Action::SelectGame(place_id, game_name) => {
                self.place_id = place_id;
                self.selected_game_name = game_name;
                self.data.last_place_id = self.place_id.clone();
                self.data.save();
                // Also update server browser's game name
                self.server_browser.current_game_name = Some(self.selected_game_name.clone());
                self.server_browser.current_place_id = Some(self.place_id.clone());
                self.set_status(format!("Selected: {}", self.selected_game_name), false);
            }
            
            Action::ToggleFavoriteGame(place_id) => {
                if self.data.favorite_games.contains(&place_id) {
                    self.data.favorite_games.retain(|id| id != &place_id);
                } else {
                    self.data.favorite_games.push(place_id);
                }
                self.data.save();
            }
            
            Action::ToggleMultiInstance => {
                match self.multi_instance.toggle() {
                    Ok(enabled) => {
                        self.data.multi_instance_enabled = enabled;
                        self.data.save();
                        if enabled {
                            self.set_status(" Multi-instance enabled - you can now run multiple Roblox clients", false);
                        } else {
                            self.set_status("Multi-instance disabled", false);
                        }
                    }
                    Err(e) => {
                        self.set_status(format!("Failed: {}", e), true);
                    }
                }
            }
            
            Action::FetchAccountInfo(idx) => {
                let account_data = self.data.accounts.get(idx).map(|a| {
                    (a.username.clone(), a.cookie.clone(), a.user_id)
                });
                
                if let Some((username, Some(cookie), Some(user_id))) = account_data {
                    self.set_status(format!("Fetching info for {}...", username), false);
                    
                    match RobloxApi::get_account_info(&cookie, user_id) {
                        Ok(info) => {
                            if let Some(account) = self.data.accounts.get_mut(idx) {
                                account.robux = Some(info.robux);
                                account.friends_count = Some(info.friends_count);
                                account.is_premium = Some(info.is_premium);
                                account.last_info_fetch = Some(chrono::Local::now().format("%Y-%m-%d %H:%M").to_string());
                            }
                            
                            // Also fetch collectibles count
                            if let Ok(collectibles) = RobloxApi::get_inventory_info(user_id) {
                                if let Some(account) = self.data.accounts.get_mut(idx) {
                                    account.collectibles_count = Some(collectibles);
                                }
                            }
                            
                            self.data.save();
                            self.set_status(format!("[OK] Updated info for {}", username), false);
                        }
                        Err(e) => {
                            self.set_status(format!("Failed to fetch info: {}", e), true);
                        }
                    }
                } else {
                    self.set_status("Verify account first", true);
                }
            }
            
            Action::BatchLaunch => {
                if self.batch_selected.is_empty() {
                    self.set_status("Select accounts to batch launch", true);
                    return;
                }
                
                // Enable multi-instance if not already
                if !self.multi_instance.is_enabled() {
                    if self.multi_instance.enable().is_err() {
                        self.set_status("Failed to enable multi-instance for batch launch", true);
                        return;
                    }
                    self.data.multi_instance_enabled = true;
                    self.data.save();
                }
                
                let place = if self.place_id.is_empty() { None } else { Some(self.place_id.clone()) };
                let delay_secs = self.batch_delay;
                let mut launched = 0;
                let mut failed = 0;
                
                let indices: Vec<usize> = self.batch_selected.iter().copied().collect();
                
                // Clone the data we need to avoid borrow conflicts
                let accounts_data: Vec<_> = indices.iter()
                    .filter_map(|idx| self.data.accounts.get(*idx).map(|a| (a.username.clone(), a.cookie.clone())))
                    .collect();
                
                for (i, (username, cookie_opt)) in accounts_data.iter().enumerate() {
                    if let Some(cookie) = cookie_opt {
                        self.set_status(format!("⏳ Launching {} ({}/{}) - please wait...", username, i + 1, accounts_data.len()), false);
                        
                        match RobloxApi::launch_multi_instance(cookie, place.as_deref()) {
                            Ok(_) => {
                                launched += 1;
                                if !self.place_id.is_empty() {
                                    self.add_recent_game(&self.place_id.clone());
                                }
                            }
                            Err(_) => failed += 1,
                        }
                        
                        if i < accounts_data.len() - 1 {
                            std::thread::sleep(std::time::Duration::from_secs(delay_secs as u64));
                        }
                    }
                }
                
                self.batch_selected.clear();
                if failed == 0 {
                    self.set_status(format!("✅ Launched {} accounts - Roblox is starting up", launched), false);
                } else {
                    self.set_status(format!("Launched {}, {} failed", launched, failed), true);
                }
            }
            
            Action::RefreshAllCookies => {
                self.set_status("Verifying all cookies...", false);
                let mut valid = 0;
                let mut invalid = 0;
                
                for idx in 0..self.data.accounts.len() {
                    if let Some(cookie) = self.data.accounts[idx].cookie.clone() {
                        match RobloxApi::validate_cookie(&cookie) {
                            Ok((user_id, display_name)) => {
                                if let Some(account) = self.data.accounts.get_mut(idx) {
                                    account.user_id = Some(user_id);
                                    account.display_name = Some(display_name);
                                    account.status = AccountStatus::Valid;
                                    account.last_login = Some(chrono::Local::now().format("%Y-%m-%d %H:%M").to_string());
                                }
                                valid += 1;
                            }
                            Err(_) => {
                                if let Some(account) = self.data.accounts.get_mut(idx) {
                                    account.status = AccountStatus::Invalid;
                                }
                                invalid += 1;
                            }
                        }
                    }
                }
                
                self.data.save();
                self.set_status(format!(" {} valid, {} invalid", valid, invalid), invalid > 0);
            }
            
            Action::RefreshPresence => {
                // Collect user IDs for valid accounts
                let user_ids: Vec<u64> = self.data.accounts.iter()
                    .filter_map(|a| {
                        if a.status == AccountStatus::Valid {
                            a.user_id
                        } else {
                            None
                        }
                    })
                    .collect();
                
                if user_ids.is_empty() {
                    self.set_status("No valid accounts to check presence", true);
                    return;
                }
                
                self.set_status("Fetching presence...", false);
                
                match RobloxApi::get_presence(&user_ids) {
                    Ok(presences) => {
                        let mut updated = 0;
                        // Track place IDs we need to look up game names for
                        let mut place_ids_to_lookup: std::collections::HashMap<u64, Vec<usize>> = std::collections::HashMap::new();
                        
                        for (idx, account) in self.data.accounts.iter_mut().enumerate() {
                            if let Some(user_id) = account.user_id {
                                if let Some(presence) = presences.get(&user_id) {
                                    let p = presence.clone();
                                    
                                    // If in game but no location name, mark for lookup
                                    if p.presence_type == crate::account::UserPresenceType::InGame {
                                        let needs_lookup = p.last_location.as_ref().map(|s| s.is_empty()).unwrap_or(true);
                                        if needs_lookup {
                                            if let Some(place_id) = p.place_id {
                                                place_ids_to_lookup.entry(place_id).or_default().push(idx);
                                            }
                                        }
                                    }
                                    
                                    account.presence = Some(p);
                                    updated += 1;
                                }
                            }
                        }
                        
                        // Fetch game names for unknown games
                        for (place_id, account_indices) in place_ids_to_lookup {
                            if let Ok((game_name, _)) = RobloxApi::get_game_info(&place_id.to_string()) {
                                for idx in account_indices {
                                    if let Some(account) = self.data.accounts.get_mut(idx) {
                                        if let Some(ref mut presence) = account.presence {
                                            presence.game_name = Some(game_name.clone());
                                        }
                                    }
                                }
                            }
                        }
                        
                        self.set_status(format!("[OK] Updated presence for {} account(s)", updated), false);
                    }
                    Err(e) => {
                        self.set_status(format!("Failed to fetch presence: {}", e), true);
                    }
                }
            }
            
            Action::FetchAvatars => {
                // Collect user IDs for all accounts with user_id
                let user_ids: Vec<u64> = self.data.accounts.iter()
                    .filter_map(|a| a.user_id)
                    .collect();
                
                if user_ids.is_empty() {
                    self.set_status("No accounts with user IDs to fetch avatars", true);
                    return;
                }
                
                self.set_status("Fetching avatars...", false);
                self.avatars_loading = true;
                
                match RobloxApi::get_avatar_thumbnails(&user_ids) {
                    Ok(avatars) => {
                        let mut loaded = 0;
                        
                        // Store avatar URLs in accounts
                        for account in self.data.accounts.iter_mut() {
                            if let Some(user_id) = account.user_id {
                                if let Some(url) = avatars.get(&user_id) {
                                    account.avatar_url = Some(url.clone());
                                    loaded += 1;
                                }
                            }
                        }
                        
                        self.data.save();
                        self.avatars_loading = false;
                        self.set_status(format!(" Fetched {} avatar URLs - loading images...", loaded), false);
                        
                        // Note: The actual texture loading will happen when rendering
                        // We'll need to load images asynchronously
                    }
                    Err(e) => {
                        self.avatars_loading = false;
                        self.set_status(format!("Failed to fetch avatars: {}", e), true);
                    }
                }
            }
            
            // Server Browser Actions
            Action::FetchServers => {
                if self.place_id.is_empty() {
                    self.set_status("Select a game first (Games tab)", true);
                    return;
                }
                
                self.server_browser.loading = true;
                self.server_browser.clear();
                self.server_browser.current_place_id = Some(self.place_id.clone());
                
                // Fetch the game name for display
                match RobloxApi::get_game_info(&self.place_id) {
                    Ok((game_name, _)) => {
                        self.server_browser.current_game_name = Some(game_name);
                    }
                    Err(_) => {
                        // Silently continue without game name - not critical
                    }
                }
                
                match crate::api::fetch_servers(&self.place_id, None) {
                    Ok(response) => {
                        self.server_browser.servers = response.data;
                        self.server_browser.next_cursor = response.next_page_cursor;
                        self.server_browser.loading = false;
                        
                        let game_display = self.server_browser.current_game_name.as_ref()
                            .map(|n| format!(" for {}", n))
                            .unwrap_or_default();
                        self.set_status(format!(" Found {} servers{}", self.server_browser.servers.len(), game_display), false);
                    }
                    Err(e) => {
                        self.server_browser.error = Some(e.clone());
                        self.server_browser.loading = false;
                        self.set_status(format!("Failed: {}", e), true);
                    }
                }
            }
            
            Action::FetchMoreServers => {
                if let Some(ref place_id) = self.server_browser.current_place_id.clone() {
                    if let Some(ref cursor) = self.server_browser.next_cursor.clone() {
                        self.server_browser.loading = true;
                        
                        match crate::api::fetch_servers(place_id, Some(cursor)) {
                            Ok(response) => {
                                self.server_browser.servers.extend(response.data);
                                self.server_browser.next_cursor = response.next_page_cursor;
                                self.server_browser.loading = false;
                                self.set_status(format!(" Loaded {} servers total", self.server_browser.servers.len()), false);
                            }
                            Err(e) => {
                                self.server_browser.error = Some(e.clone());
                                self.server_browser.loading = false;
                                self.set_status(format!("Failed: {}", e), true);
                            }
                        }
                    }
                }
            }
            
            Action::FetchVipServers => {
                if self.place_id.is_empty() {
                    self.set_status("Select a game first (Games tab)", true);
                    return;
                }
                
                // Need account cookie for VIP server fetch
                let cookie = self.selected
                    .and_then(|idx| self.data.accounts.get(idx))
                    .and_then(|a| a.cookie.clone());
                
                if let Some(cookie) = cookie {
                    self.server_browser.loading = true;
                    self.server_browser.vip_servers.clear();
                    self.server_browser.vip_next_cursor = None;
                    self.server_browser.current_place_id = Some(self.place_id.clone());
                    
                    match crate::api::fetch_vip_servers(&cookie, &self.place_id, None) {
                        Ok(response) => {
                            use crate::api::server_browser::ServerType;
                            let mut servers: Vec<_> = response.data.into_iter().map(|v| {
                                crate::api::ServerData {
                                    id: v.id,
                                    name: Some(v.name),
                                    max_players: v.max_players,
                                    playing: v.playing,
                                    vip_server_id: v.vip_server_id,
                                    access_code: v.access_code,
                                    server_type: ServerType::VIP,
                                    ..Default::default()
                                }
                            }).collect();
                            
                            self.server_browser.vip_servers.append(&mut servers);
                            self.server_browser.vip_next_cursor = response.next_page_cursor;
                            self.server_browser.loading = false;
                            self.set_status(format!("⭐ Found {} VIP servers", self.server_browser.vip_servers.len()), false);
                        }
                        Err(e) => {
                            self.server_browser.error = Some(e.clone());
                            self.server_browser.loading = false;
                            self.set_status(format!("Failed to fetch VIP servers: {}", e), true);
                        }
                    }
                } else {
                    self.set_status("Select an account to fetch VIP servers", true);
                }
            }
            
            Action::FetchMoreVipServers => {
                let cookie = self.selected
                    .and_then(|idx| self.data.accounts.get(idx))
                    .and_then(|a| a.cookie.clone());
                    
                if let (Some(cookie), Some(ref place_id), Some(ref cursor)) = (
                    cookie,
                    self.server_browser.current_place_id.clone(),
                    self.server_browser.vip_next_cursor.clone()
                ) {
                    self.server_browser.loading = true;
                    
                    match crate::api::fetch_vip_servers(&cookie, place_id, Some(cursor)) {
                        Ok(response) => {
                            use crate::api::server_browser::ServerType;
                            let mut servers: Vec<_> = response.data.into_iter().map(|v| {
                                crate::api::ServerData {
                                    id: v.id,
                                    name: Some(v.name),
                                    max_players: v.max_players,
                                    playing: v.playing,
                                    vip_server_id: v.vip_server_id,
                                    access_code: v.access_code,
                                    server_type: ServerType::VIP,
                                    ..Default::default()
                                }
                            }).collect();
                            
                            self.server_browser.vip_servers.append(&mut servers);
                            self.server_browser.vip_next_cursor = response.next_page_cursor;
                            self.server_browser.loading = false;
                            self.set_status(format!("⭐ Loaded {} VIP servers total", self.server_browser.vip_servers.len()), false);
                        }
                        Err(e) => {
                            self.server_browser.error = Some(e.clone());
                            self.server_browser.loading = false;
                            self.set_status(format!("Failed: {}", e), true);
                        }
                    }
                }
            }
            
            Action::JoinRandomServer(idx) => {
                let account_data = self.data.accounts.get(idx).map(|a| {
                    (a.username.clone(), a.cookie.clone())
                });
                
                if let Some((username, cookie_opt)) = account_data {
                    if let Some(cookie) = cookie_opt {
                        if self.place_id.is_empty() {
                            self.set_status("Select a game first", true);
                            return;
                        }
                        
                        self.set_status(format!("⏳ Finding random server for {}...", username), false);
                        
                        match crate::api::get_random_server(&self.place_id) {
                            Ok(job_id) => {
                                let multi = self.multi_instance.is_enabled();
                                match RobloxApi::launch_with_job_id(&cookie, Some(&self.place_id), Some(&job_id), multi) {
                                    Ok(_) => {
                                        self.add_recent_game(&self.place_id.clone());
                                        self.set_status(format!("✅ Launched {} to random server - Roblox is starting up", username), false);
                                    }
                                    Err(e) => self.set_status(format!("Launch failed: {}", e), true),
                                }
                            }
                            Err(e) => self.set_status(format!("No servers: {}", e), true),
                        }
                    }
                }
            }
            
            Action::JoinSelectedServer(idx) => {
                let account_data = self.data.accounts.get(idx).map(|a| {
                    (a.username.clone(), a.cookie.clone())
                });
                
                let server = self.server_browser.get_selected_server().cloned();
                
                if let Some((username, cookie_opt)) = account_data {
                    if let Some(cookie) = cookie_opt {
                        if let Some(ref place_id) = self.server_browser.current_place_id.clone() {
                            if let Some(server) = server {
                                let multi = self.multi_instance.is_enabled();
                                
                                // Check if it's a VIP server
                                if server.is_vip() {
                                    if let Some(access_code) = &server.access_code {
                                        self.set_status(format!("⏳ Joining VIP server as {} - please wait...", username), false);
                                        
                                        match RobloxApi::launch_to_private_server(&cookie, place_id, access_code, None, multi) {
                                            Ok(_) => {
                                                self.add_recent_game(place_id);
                                                let server_name = server.name.as_deref().unwrap_or("VIP Server");
                                                self.set_status(format!("✅ {} joining {} - Roblox is starting up", username, server_name), false);
                                            }
                                            Err(e) => self.set_status(format!("Launch failed: {}", e), true),
                                        }
                                    } else {
                                        // Show modal to enter access code
                                        self.server_browser.vip_access_code_input.clear();
                                        self.server_browser.vip_access_code_show = true;
                                        self.server_browser.vip_pending_server_idx = self.server_browser.selected_server;
                                        self.server_browser.vip_pending_account_idx = Some(idx);
                                        self.set_status("Enter VIP server access code to join", false);
                                    }
                                } else {
                                    // Public server - use job ID
                                    self.set_status(format!("⏳ Joining server as {} - please wait...", username), false);
                                    
                                    match RobloxApi::launch_with_job_id(&cookie, Some(place_id), Some(&server.id), multi) {
                                        Ok(_) => {
                                            self.add_recent_game(place_id);
                                            self.set_status(format!("✅ {} joining server - Roblox is starting up", username), false);
                                        }
                                        Err(e) => self.set_status(format!("Launch failed: {}", e), true),
                                    }
                                }
                            } else {
                                self.set_status("No server selected", true);
                            }
                        } else {
                            self.set_status("No game selected", true);
                        }
                    }
                }
            }
            
            Action::JoinVipWithAccessCode => {
                let access_code = self.server_browser.vip_access_code_input.trim().to_string();
                let account_idx = self.server_browser.vip_pending_account_idx;
                
                // Close the modal
                self.server_browser.vip_access_code_show = false;
                
                if access_code.is_empty() {
                    self.set_status("Access code is required", true);
                    return;
                }
                
                if let Some(idx) = account_idx {
                    let account_data = self.data.accounts.get(idx).map(|a| {
                        (a.username.clone(), a.cookie.clone())
                    });
                    
                    if let Some((username, cookie_opt)) = account_data {
                        if let Some(cookie) = cookie_opt {
                            if let Some(ref place_id) = self.server_browser.current_place_id.clone() {
                                let multi = self.multi_instance.is_enabled();
                                self.set_status(format!("⏳ Joining VIP server as {} - please wait...", username), false);
                                
                                match RobloxApi::launch_to_private_server(&cookie, place_id, &access_code, None, multi) {
                                    Ok(_) => {
                                        self.add_recent_game(place_id);
                                        self.set_status(format!("✅ {} joining VIP server - Roblox is starting up", username), false);
                                    }
                                    Err(e) => self.set_status(format!("Launch failed: {}", e), true),
                                }
                            } else {
                                self.set_status("No game selected", true);
                            }
                        }
                    }
                }
            }
            
            Action::JoinPrivateServerLink(idx) => {
                let account_data = self.data.accounts.get(idx).map(|a| {
                    (a.username.clone(), a.cookie.clone())
                });
                
                let input = self.server_browser.private_server_input.clone();
                
                if input.trim().is_empty() {
                    self.set_status("Enter a private server link first", true);
                    return;
                }
                
                if let Some((username, cookie_opt)) = account_data {
                    if let Some(cookie) = cookie_opt {
                        // Parse the private server link
                        if let Some(parsed) = crate::api::PrivateServerLink::parse(&input) {
                            let place_id = if parsed.place_id > 0 {
                                parsed.place_id.to_string()
                            } else if !self.place_id.is_empty() {
                                self.place_id.clone()
                            } else {
                                self.set_status("Could not determine Place ID - select a game first", true);
                                return;
                            };
                            
                            self.set_status(format!("⏳ Joining private server as {} - please wait...", username), false);
                            
                            let multi = self.multi_instance.is_enabled();
                            
                            // If we have a link code, try to get access code first
                            if !parsed.link_code.is_empty() {
                                match crate::api::get_access_code_from_link(&cookie, &place_id, &parsed.link_code) {
                                    Ok(access_code) => {
                                        match RobloxApi::launch_to_private_server(&cookie, &place_id, &access_code, Some(&parsed.link_code), multi) {
                                            Ok(_) => {
                                                self.add_recent_game(&place_id);
                                                self.set_status(format!("✅ {} joining private server - Roblox is starting up", username), false);
                                                self.server_browser.private_server_input.clear();
                                            }
                                            Err(e) => self.set_status(format!("Launch failed: {}", e), true),
                                        }
                                    }
                                    Err(e) => {
                                        // Try direct launch with link code as access code
                                        match RobloxApi::launch_to_private_server(&cookie, &place_id, &parsed.link_code, None, multi) {
                                            Ok(_) => {
                                                self.add_recent_game(&place_id);
                                                self.set_status(format!("✅ {} joining private server - Roblox is starting up", username), false);
                                                self.server_browser.private_server_input.clear();
                                            }
                                            Err(_) => self.set_status(format!("Failed to get access code: {}", e), true),
                                        }
                                    }
                                }
                            } else if let Some(access_code) = parsed.access_code {
                                // Direct access code
                                match RobloxApi::launch_to_private_server(&cookie, &place_id, &access_code, None, multi) {
                                    Ok(_) => {
                                        self.add_recent_game(&place_id);
                                        self.set_status(format!("✅ {} joining private server - Roblox is starting up", username), false);
                                        self.server_browser.private_server_input.clear();
                                    }
                                    Err(e) => self.set_status(format!("Launch failed: {}", e), true),
                                }
                            } else {
                                self.set_status("Invalid private server link format", true);
                            }
                        } else {
                            self.set_status("Could not parse private server link", true);
                        }
                    }
                }
            }
            
            // Account Utilities Actions
            Action::LogoutOtherSessions(idx) => {
                let account_data = self.data.accounts.get(idx).map(|a| {
                    (a.username.clone(), a.cookie.clone())
                });
                
                if let Some((username, cookie_opt)) = account_data {
                    if let Some(cookie) = cookie_opt {
                        self.set_status(format!("Logging out other sessions for {}...", username), false);
                        
                        match RobloxApi::logout_other_sessions(&cookie) {
                            Ok(new_cookie) => {
                                // Update cookie if we got a new one
                                if !new_cookie.is_empty() {
                                    if let Some(account) = self.data.accounts.get_mut(idx) {
                                        account.cookie = Some(new_cookie);
                                    }
                                    self.data.save();
                                }
                                self.set_status(format!(" Logged out other sessions for {}", username), false);
                            }
                            Err(e) => self.set_status(format!("Failed: {}", e), true),
                        }
                    }
                }
            }
            
            Action::ChangePassword(idx) => {
                let account_data = self.data.accounts.get(idx).map(|a| {
                    (a.username.clone(), a.cookie.clone(), a.password.clone())
                });
                
                if let Some((username, cookie_opt, current_pass)) = account_data {
                    if let Some(cookie) = cookie_opt {
                        let new_pass = self.util_new_password.clone();
                        
                        if new_pass.is_empty() {
                            self.set_status("Enter a new password", true);
                            return;
                        }
                        
                        self.set_status(format!("Changing password for {}...", username), false);
                        
                        match RobloxApi::change_password(&cookie, &current_pass, &new_pass) {
                            Ok(new_cookie) => {
                                // Update password and cookie
                                if let Some(account) = self.data.accounts.get_mut(idx) {
                                    account.password = new_pass;
                                    if !new_cookie.is_empty() {
                                        account.cookie = Some(new_cookie);
                                    }
                                }
                                self.data.save();
                                self.util_new_password.clear();
                                self.set_status(format!(" Password changed for {}", username), false);
                            }
                            Err(e) => self.set_status(format!("Failed: {}", e), true),
                        }
                    }
                }
            }
            
            Action::SetDisplayName(idx) => {
                let account_data = self.data.accounts.get(idx).map(|a| {
                    (a.username.clone(), a.cookie.clone(), a.user_id)
                });
                
                if let Some((username, cookie_opt, user_id_opt)) = account_data {
                    if let (Some(cookie), Some(user_id)) = (cookie_opt, user_id_opt) {
                        let new_name = self.util_new_display_name.clone();
                        
                        if new_name.is_empty() {
                            self.set_status("Enter a display name", true);
                            return;
                        }
                        
                        self.set_status(format!("Changing display name for {}...", username), false);
                        
                        match RobloxApi::set_display_name(&cookie, user_id, &new_name) {
                            Ok(_) => {
                                // Update display name
                                if let Some(account) = self.data.accounts.get_mut(idx) {
                                    account.display_name = Some(new_name);
                                }
                                self.data.save();
                                self.util_new_display_name.clear();
                                self.set_status(format!(" Display name changed for {}", username), false);
                            }
                            Err(e) => self.set_status(format!("Failed: {}", e), true),
                        }
                    } else {
                        self.set_status("Account needs to be verified first", true);
                    }
                }
            }
            
            Action::BlockUserByName(idx, username) => {
                let cookie = self.data.accounts.get(idx).and_then(|a| a.cookie.clone());
                
                if let Some(cookie) = cookie {
                    self.set_status(format!("Looking up {}...", username), false);
                    
                    match RobloxApi::get_user_id_by_username(&username) {
                        Ok(target_id) => {
                            self.set_status("Blocking user...", false);
                            match RobloxApi::block_user(&cookie, target_id) {
                                Ok(_) => self.set_status(" User blocked", false),
                                Err(e) => self.set_status(format!("Failed: {}", e), true),
                            }
                        }
                        Err(e) => self.set_status(format!("User not found: {}", e), true),
                    }
                }
            }
            
            Action::UnblockUserByName(idx, username) => {
                let cookie = self.data.accounts.get(idx).and_then(|a| a.cookie.clone());
                
                if let Some(cookie) = cookie {
                    self.set_status(format!("Looking up {}...", username), false);
                    
                    match RobloxApi::get_user_id_by_username(&username) {
                        Ok(target_id) => {
                            self.set_status("Unblocking user...", false);
                            match RobloxApi::unblock_user(&cookie, target_id) {
                                Ok(_) => self.set_status(" User unblocked", false),
                                Err(e) => self.set_status(format!("Failed: {}", e), true),
                            }
                        }
                        Err(e) => self.set_status(format!("User not found: {}", e), true),
                    }
                }
            }
            
            Action::SendFriendRequestByName(idx, username) => {
                let cookie = self.data.accounts.get(idx).and_then(|a| a.cookie.clone());
                
                if let Some(cookie) = cookie {
                    self.set_status(format!("Looking up {}...", username), false);
                    
                    match RobloxApi::get_user_id_by_username(&username) {
                        Ok(target_id) => {
                            self.set_status("Sending friend request...", false);
                            match RobloxApi::send_friend_request(&cookie, target_id) {
                                Ok(_) => self.set_status(" Friend request sent", false),
                                Err(e) => self.set_status(format!("Failed: {}", e), true),
                            }
                        }
                        Err(e) => self.set_status(format!("User not found: {}", e), true),
                    }
                }
            }
            
            Action::StartBrowserLogin => {
                // Collect existing user IDs to avoid re-importing same accounts
                let existing_ids: HashSet<u64> = self.data.accounts
                    .iter()
                    .filter_map(|a| a.user_id)
                    .collect();
                
                match crate::auth::WebLoginSession::start_with_existing_users(existing_ids) {
                    Ok(session) => {
                        self.set_status("Browser login started. Log in and click Play on any game...", false);
                        self.browser_login_session = Some(session);
                    }
                    Err(e) => {
                        self.set_status(format!("Failed to start browser login: {}", e), true);
                    }
                }
            }
            
            Action::ImportCookieAsNewAccount => {
                let cookie = self.import_cookie.trim().to_string();
                if cookie.is_empty() {
                    self.set_status("Paste a cookie first", true);
                    return;
                }
                
                self.set_status("Validating cookie...", false);
                
                match RobloxApi::validate_cookie(&cookie) {
                    Ok((user_id, display_name)) => {
                        // Check if account already exists
                        if self.data.accounts.iter().any(|a| a.user_id == Some(user_id)) {
                            self.set_status("Account already exists!", true);
                            return;
                        }
                        
                        // Get the username
                        let username = crate::api::RobloxApi::get_username_by_id(user_id)
                            .unwrap_or_else(|_| display_name.clone());
                        
                        let account = RobloxAccount {
                            username: username.clone(),
                            password: String::new(),
                            cookie: Some(cookie),
                            user_id: Some(user_id),
                            display_name: Some(display_name),
                            last_login: Some(chrono::Local::now().format("%Y-%m-%d %H:%M").to_string()),
                            status: AccountStatus::Valid,
                            notes: String::new(),
                            group: String::new(),
                            robux: None,
                            friends_count: None,
                            is_premium: None,
                            collectibles_count: None,
                            last_info_fetch: None,
                            presence: None,
                            avatar_url: None,
                        };
                        
                        self.data.accounts.push(account);
                        self.data.save();
                        self.set_status(format!(" Added: {} via cookie import", username), false);
                        self.import_cookie.clear();
                        self.tab = Tab::Accounts;
                        
                        // Fetch presence and avatars for the new account
                        self.fetch_presence_and_avatars();
                    }
                    Err(e) => {
                        self.set_status(format!("Invalid cookie: {}", e), true);
                    }
                }
            }
            
            Action::ShowCookieModal(idx) => {
                if let Some(account) = self.data.accounts.get(idx) {
                    self.cookie_modal_account_idx = Some(idx);
                    self.cookie_modal_value = account.cookie.clone().unwrap_or_default();
                    self.cookie_modal_show = true;
                }
            }
            
            Action::UpdateAccountCookie(idx) => {
                let new_cookie = self.cookie_modal_value.trim().to_string();
                if new_cookie.is_empty() {
                    self.set_status("Cookie cannot be empty", true);
                    return;
                }
                
                // Validate the new cookie
                match RobloxApi::validate_cookie(&new_cookie) {
                    Ok((user_id, display_name)) => {
                        // Get username before mutable borrow
                        let username = self.data.accounts.get(idx)
                            .map(|a| a.username.clone())
                            .unwrap_or_default();
                        
                        if let Some(account) = self.data.accounts.get_mut(idx) {
                            // Update the cookie
                            account.cookie = Some(new_cookie);
                            account.user_id = Some(user_id);
                            account.display_name = Some(display_name.clone());
                            account.status = AccountStatus::Valid;
                            account.last_login = Some(chrono::Local::now().format("%Y-%m-%d %H:%M").to_string());
                        }
                        
                        self.data.save();
                        self.set_status(format!("✅ Cookie updated for {}", username), false);
                        
                        // Close modal
                        self.cookie_modal_show = false;
                        self.cookie_modal_account_idx = None;
                        self.cookie_modal_value.clear();
                        
                        // Refresh presence
                        self.fetch_presence_and_avatars();
                    }
                    Err(e) => {
                        self.set_status(format!("Invalid cookie: {}", e), true);
                    }
                }
            }
            
            Action::FollowUser(account_idx) => {
                // Get target username
                let target_username = self.follow_user_target.trim().to_string();
                if target_username.is_empty() {
                    self.set_status("Enter a username to follow", true);
                    return;
                }
                
                // Get account cookie
                let account_data = self.data.accounts.get(account_idx).map(|a| {
                    (a.username.clone(), a.cookie.clone())
                });
                
                if let Some((account_name, Some(cookie))) = account_data {
                    self.set_status(format!("Looking up {}...", target_username), false);
                    
                    // Get target user's presence
                    match RobloxApi::get_user_presence_by_username(&target_username) {
                        Ok((_, presence)) => {
                            use crate::account::UserPresenceType;
                            
                            match presence.presence_type {
                                UserPresenceType::InGame => {
                                    if let Some(place_id) = presence.place_id {
                                        // Get game name for status message
                                        let game_name = presence.last_location.clone()
                                            .or_else(|| RobloxApi::get_game_info(&place_id.to_string())
                                                .map(|(name, _)| name).ok())
                                            .unwrap_or_else(|| place_id.to_string());
                                        
                                        self.set_status(format!("⏳ Joining {} - please wait...", game_name), false);
                                        
                                        let result = if let Some(ref job_id) = presence.game_id {
                                            RobloxApi::launch_to_server(&cookie, &place_id.to_string(), job_id)
                                        } else {
                                            if self.multi_instance.is_enabled() {
                                                RobloxApi::launch_multi_instance(&cookie, Some(&place_id.to_string()))
                                            } else {
                                                RobloxApi::set_account_and_launch(&cookie, Some(&place_id.to_string()))
                                            }
                                        };
                                        
                                        match result {
                                            Ok(_) => {
                                                self.set_status(format!("✅ {} following {} into {} - Roblox is starting up", account_name, target_username, game_name), false);
                                                self.follow_user_show = false;
                                                self.follow_user_target.clear();
                                                self.follow_user_account_idx = None;
                                            }
                                            Err(e) => self.set_status(format!("Launch failed: {}", e), true),
                                        }
                                    } else {
                                        self.set_status(format!("{} is in a private/hidden game", target_username), true);
                                    }
                                }
                                UserPresenceType::InStudio => {
                                    self.set_status(format!("{} is in Roblox Studio (cannot follow)", target_username), true);
                                }
                                UserPresenceType::Online => {
                                    self.set_status(format!("{} is online but not in a game", target_username), true);
                                }
                                UserPresenceType::Offline => {
                                    self.set_status(format!("{} is offline", target_username), true);
                                }
                            }
                        }
                        Err(e) => {
                            self.set_status(format!("Failed to find user: {}", e), true);
                        }
                    }
                } else {
                    self.set_status("Account needs a valid cookie", true);
                }
            }
            
            Action::ImportDroppedCookie(cookie) => {
                // Validate and import the dropped cookie
                let cookie = cookie.trim().to_string();
                if cookie.is_empty() {
                    self.set_status("Dropped content is empty", true);
                    return;
                }
                
                // Check if it looks like a cookie
                if !cookie.contains("_|WARNING:-DO-NOT-SHARE") && cookie.len() < 100 {
                    self.set_status("Dropped content doesn't look like a valid cookie", true);
                    return;
                }
                
                self.set_status("Validating dropped cookie...", false);
                
                match RobloxApi::validate_cookie(&cookie) {
                    Ok((user_id, display_name)) => {
                        // Check if account already exists
                        if self.data.accounts.iter().any(|a| a.user_id == Some(user_id)) {
                            self.set_status("Account already exists!", true);
                            return;
                        }
                        
                        // Get the username
                        let username = RobloxApi::get_username_by_id(user_id)
                            .unwrap_or_else(|_| display_name.clone());
                        
                        let account = RobloxAccount {
                            username: username.clone(),
                            password: String::new(),
                            cookie: Some(cookie),
                            user_id: Some(user_id),
                            display_name: Some(display_name),
                            last_login: Some(chrono::Local::now().format("%Y-%m-%d %H:%M").to_string()),
                            status: AccountStatus::Valid,
                            notes: String::new(),
                            group: String::new(),
                            robux: None,
                            friends_count: None,
                            is_premium: None,
                            collectibles_count: None,
                            last_info_fetch: None,
                            presence: None,
                            avatar_url: None,
                        };
                        
                        self.data.accounts.push(account);
                        self.data.save();
                        self.set_status(format!("✅ Added: {} via drag & drop", username), false);
                        self.tab = Tab::Accounts;
                        
                        // Fetch presence and avatars for the new account
                        self.fetch_presence_and_avatars();
                    }
                    Err(e) => {
                        self.set_status(format!("Invalid cookie: {}", e), true);
                    }
                }
                
                self.drag_drop_active = false;
            }
            
            Action::AddUserGame(place_id, name) => {
                // Check if game already exists
                if self.data.user_games.iter().any(|g| g.place_id == place_id) {
                    self.set_status("Game already in your list", true);
                    return;
                }
                
                // Fetch universe ID for the place to get thumbnail
                let universe_id = RobloxApi::get_universe_id(&place_id).ok();
                
                let user_game = UserGame {
                    place_id: place_id.clone(),
                    name: name.clone(),
                    universe_id,
                };
                
                self.data.user_games.push(user_game);
                self.data.save();
                
                // If we got a universe ID, fetch the icon
                if let Some(uid) = universe_id {
                    if let Ok(icons) = RobloxApi::get_game_icons(&[uid]) {
                        if let Some(url) = icons.get(&uid) {
                            self.game_icons.insert(uid, url.clone());
                        }
                    }
                }
                
                self.set_status(format!("✅ Added '{}' to your games", name), false);
                self.add_user_game_place_id.clear();
                self.add_user_game_name.clear();
                self.add_user_game_show = false;
            }
            
            Action::RemoveUserGame(place_id) => {
                self.data.user_games.retain(|g| g.place_id != place_id);
                self.data.save();
                self.set_status("Game removed from your list", false);
            }
        }
    }
    
    pub fn fetch_presence_and_avatars(&mut self) {
        // Collect user IDs for valid accounts
        let user_ids: Vec<u64> = self.data.accounts.iter()
            .filter_map(|a| a.user_id)
            .collect();
        
        if user_ids.is_empty() {
            return;
        }
        
        // Fetch presence
        if let Ok(presences) = RobloxApi::get_presence(&user_ids) {
            let mut place_ids_to_lookup: std::collections::HashMap<u64, Vec<usize>> = std::collections::HashMap::new();
            
            for (idx, account) in self.data.accounts.iter_mut().enumerate() {
                if let Some(user_id) = account.user_id {
                    if let Some(presence) = presences.get(&user_id) {
                        let p = presence.clone();
                        
                        // If in game but no location name, mark for lookup
                        if p.presence_type == crate::account::UserPresenceType::InGame {
                            let needs_lookup = p.last_location.as_ref().map(|s| s.is_empty()).unwrap_or(true);
                            if needs_lookup {
                                if let Some(place_id) = p.place_id {
                                    place_ids_to_lookup.entry(place_id).or_default().push(idx);
                                }
                            }
                        }
                        
                        account.presence = Some(p);
                    }
                }
            }
            
            // Fetch game names for unknown games
            for (place_id, account_indices) in place_ids_to_lookup {
                if let Ok((game_name, _)) = RobloxApi::get_game_info(&place_id.to_string()) {
                    for idx in account_indices {
                        if let Some(account) = self.data.accounts.get_mut(idx) {
                            if let Some(ref mut presence) = account.presence {
                                presence.game_name = Some(game_name.clone());
                            }
                        }
                    }
                }
            }
        }
        
        // Fetch avatars
        if let Ok(avatars) = RobloxApi::get_avatar_thumbnails(&user_ids) {
            for account in self.data.accounts.iter_mut() {
                if let Some(user_id) = account.user_id {
                    if let Some(url) = avatars.get(&user_id) {
                        account.avatar_url = Some(url.clone());
                    }
                }
            }
            self.data.save();
        }
        
        self.last_presence_refresh = std::time::Instant::now();
    }
    
    pub fn refresh_presence_only(&mut self) {
        let user_ids: Vec<u64> = self.data.accounts.iter()
            .filter_map(|a| {
                if a.status == crate::account::AccountStatus::Valid {
                    a.user_id
                } else {
                    None
                }
            })
            .collect();
        
        if user_ids.is_empty() {
            return;
        }
        
        if let Ok(presences) = RobloxApi::get_presence(&user_ids) {
            let mut place_ids_to_lookup: std::collections::HashMap<u64, Vec<usize>> = std::collections::HashMap::new();
            
            for (idx, account) in self.data.accounts.iter_mut().enumerate() {
                if let Some(user_id) = account.user_id {
                    if let Some(presence) = presences.get(&user_id) {
                        let p = presence.clone();
                        
                        if p.presence_type == crate::account::UserPresenceType::InGame {
                            let needs_lookup = p.last_location.as_ref().map(|s| s.is_empty()).unwrap_or(true);
                            if needs_lookup {
                                if let Some(place_id) = p.place_id {
                                    place_ids_to_lookup.entry(place_id).or_default().push(idx);
                                }
                            }
                        }
                        
                        account.presence = Some(p);
                    }
                }
            }
            
            for (place_id, account_indices) in place_ids_to_lookup {
                if let Ok((game_name, _)) = RobloxApi::get_game_info(&place_id.to_string()) {
                    for idx in account_indices {
                        if let Some(account) = self.data.accounts.get_mut(idx) {
                            if let Some(ref mut presence) = account.presence {
                                presence.game_name = Some(game_name.clone());
                            }
                        }
                    }
                }
            }
        }
        
        self.last_presence_refresh = std::time::Instant::now();
    }
    
    pub fn check_browser_login_result(&mut self) {
        if let Some(ref session) = self.browser_login_session {
            if let Some(result) = session.try_get_result() {
                match result {
                    crate::auth::LoginResult::Success { cookie, user_id, username, display_name } => {
                        // Check if account already exists
                        if self.data.accounts.iter().any(|a| a.user_id == Some(user_id)) {
                            self.set_status(format!("Account {} already exists!", username), true);
                        } else {
                            // Create new account
                            let account = RobloxAccount {
                                username: username.clone(),
                                password: String::new(),  // No password with browser login
                                cookie: Some(cookie),
                                user_id: Some(user_id),
                                display_name: Some(display_name),
                                last_login: Some(chrono::Local::now().format("%Y-%m-%d %H:%M").to_string()),
                                status: AccountStatus::Valid,
                                notes: String::new(),
                                group: String::new(),
                                robux: None,
                                friends_count: None,
                                is_premium: None,
                                collectibles_count: None,
                                last_info_fetch: None,
                                presence: None,
                                avatar_url: None,
                            };
                            
                            self.data.accounts.push(account);
                            self.data.save();
                            self.set_status(format!(" Added: {} via browser login", username), false);
                            self.tab = Tab::Accounts;
                            
                            // Fetch presence and avatars for the new account
                            self.fetch_presence_and_avatars();
                        }
                    }
                    crate::auth::LoginResult::Cancelled => {
                        self.set_status("Browser login cancelled", false);
                    }
                    crate::auth::LoginResult::Error(e) => {
                        self.set_status(format!("Browser login failed: {}", e), true);
                    }
                }
                // Clear the session
                self.browser_login_session = None;
            }
        }
    }
    
    pub fn load_game_icons(&mut self) {
        use crate::games::POPULAR_GAMES;
        
        // Collect universe IDs from popular games
        let mut universe_ids: Vec<u64> = POPULAR_GAMES.iter()
            .filter_map(|g| g.universe_id)
            .collect();
        
        // Also collect from user games
        for user_game in &self.data.user_games {
            if let Some(uid) = user_game.universe_id {
                if !universe_ids.contains(&uid) {
                    universe_ids.push(uid);
                }
            }
        }
        
        if universe_ids.is_empty() {
            return;
        }
        
        // Fetch icons
        if let Ok(icons) = RobloxApi::get_game_icons(&universe_ids) {
            self.game_icons = icons;
        }
    }
    
    pub fn add_recent_game(&mut self, place_id: &str) {
        // Try to get game name
        let name = RobloxApi::get_game_info(place_id)
            .map(|(n, _)| n)
            .unwrap_or_else(|_| format!("Place {}", place_id));
        
        // Check if already in recent
        if let Some(game) = self.data.recent_games.iter_mut().find(|g| g.place_id == place_id) {
            game.play_count += 1;
            game.last_played = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
        } else {
            // Add new
            self.data.recent_games.insert(0, RecentGame {
                place_id: place_id.to_string(),
                name,
                last_played: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
                play_count: 1,
            });
            
            // Keep only last 10
            if self.data.recent_games.len() > 10 {
                self.data.recent_games.truncate(10);
            }
        }
        
        self.data.save();
    }
    
    pub fn render_settings_tab(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading(RichText::new("⚙ Settings").size(20.0).color(Colors::TEXT_PRIMARY));
            ui.add_space(16.0);
            
            // Multi-Instance Section
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("🎮 Multi-Instance Mode").size(16.0).color(Colors::TEXT_PRIMARY).strong());
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let is_enabled = self.multi_instance.is_enabled();
                            let btn_text = if is_enabled { " Enabled" } else { "Disabled" };
                            let btn_color = if is_enabled { Colors::ACCENT_GREEN } else { Colors::BG_LIGHT };
                            
                            let btn = egui::Button::new(
                                RichText::new(btn_text).color(Colors::TEXT_PRIMARY)
                            )
                            .fill(btn_color)
                            .rounding(egui::Rounding::same(6.0))
                            .min_size(egui::vec2(100.0, 32.0));
                            
                            if ui.add(btn).clicked() {
                                self.action = Action::ToggleMultiInstance;
                            }
                        });
                    });
                    
                    ui.add_space(8.0);
                    ui.label(RichText::new(
                        "When enabled, you can run multiple Roblox clients simultaneously with different accounts."
                    ).color(Colors::TEXT_MUTED).size(13.0));
                    
                    ui.add_space(8.0);
                    
                    if self.multi_instance.is_enabled() {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("●").color(Colors::ACCENT_GREEN).size(12.0));
                            ui.label(RichText::new("Holding ROBLOX_singletonMutex - multiple clients allowed")
                                .color(Colors::TEXT_MUTED).size(12.0));
                        });
                    } else {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("○").color(Colors::TEXT_MUTED).size(12.0));
                            ui.label(RichText::new("Standard mode - only one Roblox client at a time")
                                .color(Colors::TEXT_MUTED).size(12.0));
                        });
                    }
                });
            
            ui.add_space(16.0);
            
            // Batch Launch Settings
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.label(RichText::new("👥 Batch Launch Settings").size(16.0).color(Colors::TEXT_PRIMARY).strong());
                    ui.add_space(12.0);
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Delay between launches:").color(Colors::TEXT_SECONDARY));
                        ui.add_space(8.0);
                        
                        let mut delay = self.batch_delay as f32;
                        let slider = egui::Slider::new(&mut delay, 1.0..=30.0)
                            .suffix(" seconds")
                            .integer();
                        
                        if ui.add(slider).changed() {
                            self.batch_delay = delay as u32;
                            self.data.batch_launch_delay = self.batch_delay;
                            self.data.save();
                        }
                    });
                    
                    ui.add_space(8.0);
                    ui.label(RichText::new(
                        "Time to wait between launching each account in batch mode. Higher values are safer to avoid rate limits."
                    ).color(Colors::TEXT_MUTED).size(12.0));
                });
            
            ui.add_space(16.0);
            
            // Minimize to Tray Section
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Minimize to Tray").size(16.0).color(Colors::TEXT_PRIMARY).strong());
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let is_enabled = self.data.minimize_to_tray;
                            let btn_text = if is_enabled { " Enabled" } else { "Disabled" };
                            let btn_color = if is_enabled { Colors::ACCENT_GREEN } else { Colors::BG_LIGHT };
                            
                            let btn = egui::Button::new(
                                RichText::new(btn_text).color(Colors::TEXT_PRIMARY)
                            )
                            .fill(btn_color)
                            .rounding(egui::Rounding::same(6.0))
                            .min_size(egui::vec2(100.0, 32.0));
                            
                            if ui.add(btn).clicked() {
                                self.data.minimize_to_tray = !self.data.minimize_to_tray;
                                self.data.save();
                            }
                        });
                    });
                    
                    ui.add_space(8.0);
                    ui.label(RichText::new(
                        "When enabled, minimizing the window will hide it to the system tray instead of the taskbar."
                    ).color(Colors::TEXT_MUTED).size(13.0));
                });
            
            ui.add_space(16.0);
            // Account Maintenance
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.label(RichText::new("🔄 Account Maintenance").size(16.0).color(Colors::TEXT_PRIMARY).strong());
                    ui.add_space(12.0);
                    
                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new(
                            RichText::new("Verify All Cookies").color(Colors::TEXT_PRIMARY).size(13.0)
                        ).fill(Colors::ACCENT_BLUE).rounding(egui::Rounding::same(6.0)).min_size(egui::vec2(140.0, 36.0))).clicked() {
                            self.action = Action::RefreshAllCookies;
                        }
                        
                        ui.add_space(8.0);
                        ui.label(RichText::new("Check validity of all account cookies").color(Colors::TEXT_MUTED).size(12.0));
                    });
                    
                    ui.add_space(12.0);
                    
                    // Account count info
                    let valid_count = self.data.accounts.iter().filter(|a| a.status == AccountStatus::Valid).count();
                    let total_count = self.data.accounts.len();
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("Accounts: {} total, {} valid", total_count, valid_count))
                            .color(Colors::TEXT_MUTED).size(12.0));
                    });
                });
            
            ui.add_space(16.0);
            
            // Info section
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.label(RichText::new("ℹ How Multi-Instance Works").size(14.0).color(Colors::ACCENT_BLUE).strong());
                    ui.add_space(8.0);
                    ui.label(RichText::new(
                        "Roblox uses a system mutex (ROBLOX_singletonMutex) to prevent multiple clients. \
                        When multi-instance is enabled, this manager holds that mutex, allowing each new \
                        Roblox client to launch without being blocked."
                    ).color(Colors::TEXT_MUTED).size(12.0));
                    
                    ui.add_space(12.0);
                    ui.label(RichText::new("⚠ Note").size(14.0).color(Colors::ACCENT_YELLOW).strong());
                    ui.add_space(4.0);
                    ui.label(RichText::new(
                        "Close any running Roblox clients before enabling this feature. \
                        Keep this manager open while using multi-instance."
                    ).color(Colors::TEXT_MUTED).size(12.0));
                });
            
            ui.add_space(16.0);
            
            // Recent Games (clear option)
            if !self.data.recent_games.is_empty() {
                egui::Frame::none()
                    .fill(Colors::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(16.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("📋 Recent Games").size(16.0).color(Colors::TEXT_PRIMARY).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.add(egui::Button::new(
                                    RichText::new("Clear History").color(Colors::TEXT_MUTED).size(11.0)
                                ).fill(Colors::BG_LIGHT).rounding(egui::Rounding::same(4.0))).clicked() {
                                    self.data.recent_games.clear();
                                    self.data.save();
                                }
                            });
                        });
                        
                        ui.add_space(8.0);
                        ui.label(RichText::new(format!("{} recent game(s) tracked", self.data.recent_games.len()))
                            .color(Colors::TEXT_MUTED).size(12.0));
                    });
            }
        });
    }
    
    pub fn render_about_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);
            
            // Nexus Underground Title
            ui.label(RichText::new("Nexus Underground")
                .size(32.0)
                .color(Colors::ACCENT_BLUE)
                .strong());
            
            ui.add_space(24.0);
            
            // Discord Button
            let discord_btn = egui::Button::new(
                RichText::new("💬 Join Discord").color(egui::Color32::WHITE).size(16.0)
            )
            .fill(egui::Color32::from_rgb(88, 101, 242))
            .rounding(egui::Rounding::same(8.0));
            
            if ui.add_sized([180.0, 44.0], discord_btn).clicked() {
                let _ = webbrowser::open("https://discord.gg/U8ehekqN64");
            }
        });
    }
    
    pub fn render_cookie_modal(&mut self, ctx: &egui::Context) {
        let account_name = self.cookie_modal_account_idx
            .and_then(|idx| self.data.accounts.get(idx))
            .map(|a| a.username.clone())
            .unwrap_or_default();
        
        egui::Window::new("Cookie Management")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .fixed_size(egui::vec2(500.0, 350.0))
            .frame(egui::Frame::none()
                .fill(Colors::BG_MEDIUM)
                .stroke(egui::Stroke::new(2.0, Colors::BORDER_ACCENT))
                .rounding(egui::Rounding::same(12.0))
                .inner_margin(egui::Margin::same(20.0))
            )
            .show(ctx, |ui| {
                // Header
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🍪").size(24.0));
                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Cookie Management")
                            .color(Colors::TEXT_PRIMARY)
                            .size(18.0)
                            .strong());
                        ui.label(RichText::new(format!("Account: {}", account_name))
                            .color(Colors::TEXT_MUTED)
                            .size(12.0));
                    });
                });
                
                ui.add_space(16.0);
                ui.separator();
                ui.add_space(12.0);
                
                // Cookie display/edit
                ui.label(RichText::new(".ROBLOSECURITY Cookie")
                    .color(Colors::TEXT_SECONDARY)
                    .size(12.0)
                    .strong());
                ui.add_space(6.0);
                
                egui::Frame::none()
                    .fill(Colors::BG_DARK)
                    .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                    .rounding(egui::Rounding::same(6.0))
                    .inner_margin(egui::Margin::same(10.0))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(120.0)
                            .show(ui, |ui| {
                                ui.add_sized(
                                    [ui.available_width(), 100.0],
                                    egui::TextEdit::multiline(&mut self.cookie_modal_value)
                                        .font(egui::TextStyle::Monospace)
                                        .text_color(Colors::TEXT_PRIMARY)
                                        .frame(false)
                                );
                            });
                    });
                
                ui.add_space(12.0);
                
                // Warning
                egui::Frame::none()
                    .fill(Colors::ACCENT_YELLOW.linear_multiply(0.1))
                    .rounding(egui::Rounding::same(6.0))
                    .inner_margin(egui::Margin::same(10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("⚠").size(14.0));
                            ui.add_space(6.0);
                            ui.label(RichText::new("Never share your cookie with anyone. It grants full access to your account.")
                                .color(Colors::TEXT_SECONDARY)
                                .size(11.0));
                        });
                    });
                
                ui.add_space(16.0);
                
                // Buttons
                ui.horizontal(|ui| {
                    // Copy button
                    if let Some(idx) = self.cookie_modal_account_idx {
                        let copy_btn = egui::Button::new(
                            RichText::new("📋 Copy").size(13.0).color(Colors::TEXT_PRIMARY)
                        )
                        .fill(Colors::BG_LIGHT)
                        .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                        .rounding(egui::Rounding::same(6.0));
                        
                        if ui.add_sized([80.0, 36.0], copy_btn).clicked() {
                            if let Some(account) = self.data.accounts.get(idx) {
                                if let Some(ref cookie) = account.cookie {
                                    ui.output_mut(|o| o.copied_text = cookie.clone());
                                    self.set_status("Cookie copied to clipboard", false);
                                }
                            }
                        }
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Cancel button
                        let cancel_btn = egui::Button::new(
                            RichText::new("Cancel").size(13.0).color(Colors::TEXT_SECONDARY)
                        )
                        .fill(Colors::BG_LIGHT)
                        .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                        .rounding(egui::Rounding::same(6.0));
                        
                        if ui.add_sized([90.0, 36.0], cancel_btn).clicked() {
                            self.cookie_modal_show = false;
                            self.cookie_modal_account_idx = None;
                            self.cookie_modal_value.clear();
                        }
                        
                        ui.add_space(8.0);
                        
                        // Update button
                        if let Some(idx) = self.cookie_modal_account_idx {
                            let update_btn = egui::Button::new(
                                RichText::new("Update Cookie").size(13.0).color(egui::Color32::WHITE)
                            )
                            .fill(Colors::ACCENT_GREEN)
                            .rounding(egui::Rounding::same(6.0));
                            
                            if ui.add_sized([130.0, 36.0], update_btn).clicked() {
                                self.action = Action::UpdateAccountCookie(idx);
                            }
                        }
                    });
                });
            });
    }
    
    pub fn render_follow_user_modal(&mut self, ctx: &egui::Context) {
        let account_name = self.follow_user_account_idx
            .and_then(|idx| self.data.accounts.get(idx))
            .map(|a| a.username.clone())
            .unwrap_or_default();
        
        egui::Window::new("Follow User")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .fixed_size(egui::vec2(400.0, 280.0))
            .frame(egui::Frame::none()
                .fill(Colors::BG_MEDIUM)
                .stroke(egui::Stroke::new(2.0, Colors::ACCENT_BLUE))
                .rounding(egui::Rounding::same(12.0))
                .inner_margin(egui::Margin::same(20.0))
            )
            .show(ctx, |ui| {
                // Header
                ui.horizontal(|ui| {
                    ui.label(RichText::new("👥").size(24.0));
                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Follow User Into Game")
                            .color(Colors::TEXT_PRIMARY)
                            .size(18.0)
                            .strong());
                        ui.label(RichText::new(format!("Using: {}", account_name))
                            .color(Colors::TEXT_MUTED)
                            .size(12.0));
                    });
                });
                
                ui.add_space(16.0);
                ui.separator();
                ui.add_space(12.0);
                
                // Description
                ui.label(RichText::new("Enter the username of the player you want to follow. If they're in a game, you'll join them!")
                    .color(Colors::TEXT_SECONDARY)
                    .size(12.0));
                
                ui.add_space(12.0);
                
                // Username input
                ui.label(RichText::new("Target Username")
                    .color(Colors::TEXT_SECONDARY)
                    .size(12.0)
                    .strong());
                ui.add_space(4.0);
                
                egui::Frame::none()
                    .fill(Colors::BG_DARK)
                    .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                    .rounding(egui::Rounding::same(6.0))
                    .inner_margin(egui::Margin::same(10.0))
                    .show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), 24.0],
                            egui::TextEdit::singleline(&mut self.follow_user_target)
                                .hint_text("Enter username to follow...")
                                .text_color(Colors::TEXT_PRIMARY)
                                .frame(false)
                        );
                    });
                
                ui.add_space(12.0);
                
                // Info note
                egui::Frame::none()
                    .fill(Colors::ACCENT_BLUE.linear_multiply(0.1))
                    .rounding(egui::Rounding::same(6.0))
                    .inner_margin(egui::Margin::same(10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("ℹ").size(14.0));
                            ui.add_space(6.0);
                            ui.label(RichText::new("Works best if the user's game is set to public/friends.")
                                .color(Colors::TEXT_SECONDARY)
                                .size(11.0));
                        });
                    });
                
                ui.add_space(16.0);
                
                // Buttons
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Cancel button
                        let cancel_btn = egui::Button::new(
                            RichText::new("Cancel").size(13.0).color(Colors::TEXT_SECONDARY)
                        )
                        .fill(Colors::BG_LIGHT)
                        .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                        .rounding(egui::Rounding::same(6.0));
                        
                        if ui.add_sized([90.0, 36.0], cancel_btn).clicked() {
                            self.follow_user_show = false;
                            self.follow_user_target.clear();
                            self.follow_user_account_idx = None;
                        }
                        
                        ui.add_space(8.0);
                        
                        // Follow button
                        if let Some(idx) = self.follow_user_account_idx {
                            let follow_btn = egui::Button::new(
                                RichText::new("👥 Follow User").size(13.0).color(egui::Color32::WHITE)
                            )
                            .fill(Colors::ACCENT_BLUE)
                            .rounding(egui::Rounding::same(6.0));
                            
                            let enabled = !self.follow_user_target.trim().is_empty();
                            ui.add_enabled_ui(enabled, |ui| {
                                if ui.add_sized([120.0, 36.0], follow_btn).clicked() {
                                    self.action = Action::FollowUser(idx);
                                }
                            });
                        }
                    });
                });
            });
    }
}
