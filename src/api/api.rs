use crate::account::{UserPresence, UserPresenceType};
use std::collections::HashMap;
use std::process::Command;
use std::time::Duration;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Default)]
pub struct AccountInfo {
    pub robux: i64,
    pub friends_count: u32,
    pub is_premium: bool,
}

fn run_async<F, T>(future: F) -> Result<T, String>
where
    F: std::future::Future<Output = Result<T, String>> + Send + 'static,
    T: Send + 'static,
{
    std::thread::scope(|s| {
        s.spawn(|| {
            let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
            rt.block_on(future)
        }).join().unwrap()
    })
}

fn format_cookie_str(cookie: &str) -> String {
    let cookie = cookie.trim();
    if cookie.starts_with("_|WARNING:-DO-NOT-SHARE-THIS.--Sharing-this-will-allow-someone-to-log-in-as-you-and-to-steal-your-ROBUX-and-items.|_") {
        cookie.to_string()
    } else {
        cookie.to_string()
    }
}

pub struct RobloxApi;

impl RobloxApi {
    pub fn validate_cookie(cookie: &str) -> Result<(u64, String), String> {
        let cookie = cookie.to_string();
        
        run_async(async move {
            let client = reqwest::Client::new();
            let formatted_cookie = format_cookie_str(&cookie);

            let resp = client
                .get("https://users.roblox.com/v1/users/authenticated")
                .header("Cookie", format!(".ROBLOSECURITY={}", formatted_cookie))
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;

            if !resp.status().is_success() {
                return Err("Cookie is invalid or expired".to_string());
            }

            let info: serde_json::Value = resp.json().await.map_err(|_| "Failed to parse response")?;
            
            let user_id = info.get("id").and_then(|v| v.as_u64()).ok_or("No user ID in response")?;
            let display_name = info.get("displayName").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();

            Ok((user_id, display_name))
        })
    }

    pub fn get_account_info(cookie: &str, user_id: u64) -> Result<AccountInfo, String> {
        let cookie = cookie.to_string();
        
        run_async(async move {
            let client = reqwest::Client::new();
            let formatted_cookie = format_cookie_str(&cookie);
            let cookie_header = format!(".ROBLOSECURITY={}", formatted_cookie);
            
            let mut info = AccountInfo::default();
            
            if let Ok(resp) = client
                .get("https://economy.roblox.com/v1/user/currency")
                .header("Cookie", &cookie_header)
                .send()
                .await
            {
                if let Ok(data) = resp.json::<serde_json::Value>().await {
                    info.robux = data.get("robux").and_then(|v| v.as_i64()).unwrap_or(0);
                }
            }
            
            if let Ok(resp) = client
                .get(format!("https://friends.roblox.com/v1/users/{}/friends/count", user_id))
                .header("Cookie", &cookie_header)
                .send()
                .await
            {
                if let Ok(data) = resp.json::<serde_json::Value>().await {
                    info.friends_count = data.get("count").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                }
            }
            
            if let Ok(resp) = client
                .get(format!("https://premiumfeatures.roblox.com/v1/users/{}/validate-membership", user_id))
                .header("Cookie", &cookie_header)
                .send()
                .await
            {
                info.is_premium = resp.status().is_success();
            }
            
            Ok(info)
        })
    }

    pub fn get_presence(user_ids: &[u64]) -> Result<HashMap<u64, UserPresence>, String> {
        if user_ids.is_empty() {
            return Ok(HashMap::new());
        }
        
        let user_ids = user_ids.to_vec();
        
        run_async(async move {
            let client = reqwest::Client::new();
            
            let body = serde_json::json!({
                "userIds": user_ids
            });
            
            let resp = client
                .post("https://presence.roblox.com/v1/presence/users")
                .json(&body)
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if !resp.status().is_success() {
                return Err("Failed to fetch presence".to_string());
            }
            
            let data: serde_json::Value = resp.json().await.map_err(|_| "Failed to parse response")?;
            
            let mut result = HashMap::new();
            
            if let Some(presences) = data.get("userPresences").and_then(|v| v.as_array()) {
                for p in presences {
                    let user_id = p.get("userId").and_then(|v| v.as_u64()).unwrap_or(0);
                    let presence_type = p.get("userPresenceType").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
                    let last_location = p.get("lastLocation").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let place_id = p.get("placeId").and_then(|v| v.as_u64());
                    let game_id = p.get("gameId").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let last_online = p.get("lastOnline").and_then(|v| v.as_str()).map(|s| s.to_string());
                    
                    result.insert(user_id, UserPresence {
                        presence_type: UserPresenceType::from_int(presence_type),
                        last_location,
                        place_id,
                        game_id,
                        last_online,
                        game_name: None,
                    });
                }
            }
            
            Ok(result)
        })
    }

    pub fn get_game_info(place_id: &str) -> Result<(String, String), String> {
        let place_id = place_id.to_string();
        
        run_async(async move {
            let client = reqwest::Client::new();
            
            let universe_resp = client
                .get(format!("https://apis.roblox.com/universes/v1/places/{}/universe", place_id))
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            let universe_id = if universe_resp.status().is_success() {
                let data: serde_json::Value = universe_resp.json().await.map_err(|_| "Failed to parse universe response")?;
                data.get("universeId").and_then(|v| v.as_u64()).map(|v| v.to_string())
            } else {
                None
            };
            
            if let Some(ref uid) = universe_id {
                let game_resp = client
                    .get(format!("https://games.roblox.com/v1/games?universeIds={}", uid))
                    .send()
                    .await
                    .map_err(|e| format!("Game request failed: {}", e))?;
                
                if game_resp.status().is_success() {
                    let data: serde_json::Value = game_resp.json().await.map_err(|_| "Failed to parse game response")?;
                    if let Some(games) = data.get("data").and_then(|d| d.as_array()) {
                        if let Some(game) = games.first() {
                            let name = game.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Game").to_string();
                            return Ok((name, uid.clone()));
                        }
                    }
                }
            }
            
            let resp = client
                .get(format!("https://games.roblox.com/v1/games/multiget-place-details?placeIds={}", place_id))
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if resp.status().is_success() {
                let data: serde_json::Value = resp.json().await.map_err(|_| "Failed to parse response")?;
                
                if let Some(game) = data.as_array().and_then(|arr| arr.first()) {
                    let name = game.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Game").to_string();
                    let uid = game.get("universeId").and_then(|v| v.as_u64()).map(|v| v.to_string()).unwrap_or_default();
                    return Ok((name, uid));
                }
            }
            
            Err("Game not found".to_string())
        })
    }

    pub fn get_auth_ticket(cookie: &str) -> Result<String, String> {
        let cookie = cookie.to_string();
        
        run_async(async move {
            let client = reqwest::Client::builder()
                .user_agent("Roblox/WinInet")
                .build()
                .map_err(|e| format!("Failed to build client: {}", e))?;
            let formatted_cookie = format_cookie_str(&cookie);

            let csrf_resp = client
                .post("https://auth.roblox.com/v1/authentication-ticket/")
                .header("Cookie", format!(".ROBLOSECURITY={}", formatted_cookie))
                .header("Content-Type", "application/json")
                .header("Referer", "https://www.roblox.com/games/4924922222/Brookhaven-RP")
                .send()
                .await
                .map_err(|e| format!("Network error: {}", e))?;

            let csrf_token = csrf_resp
                .headers()
                .get("x-csrf-token")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string())
                .ok_or("Failed to get CSRF token")?;

            let ticket_resp = client
                .post("https://auth.roblox.com/v1/authentication-ticket/")
                .header("Cookie", format!(".ROBLOSECURITY={}", formatted_cookie))
                .header("X-CSRF-TOKEN", &csrf_token)
                .header("Content-Type", "application/json")
                .header("Referer", "https://www.roblox.com/games/4924922222/Brookhaven-RP")
                .send()
                .await
                .map_err(|e| format!("Ticket request failed: {}", e))?;

            if !ticket_resp.status().is_success() {
                return Err(format!("Failed to get auth ticket: HTTP {}", ticket_resp.status()));
            }

            let ticket = ticket_resp
                .headers()
                .get("rbx-authentication-ticket")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string())
                .ok_or("No authentication ticket in response")?;

            Ok(ticket)
        })
    }

    #[cfg(windows)]
    pub fn set_account_and_launch(cookie: &str, place_id: Option<&str>) -> Result<(), String> {
        Self::launch_with_options(cookie, place_id, false)
    }
    
    #[cfg(windows)]
    pub fn launch_multi_instance(cookie: &str, place_id: Option<&str>) -> Result<(), String> {
        Self::launch_with_options(cookie, place_id, true)
    }

    #[cfg(windows)]
    fn launch_with_options(cookie: &str, place_id: Option<&str>, multi_instance: bool) -> Result<(), String> {
        Self::launch_with_job_id(cookie, place_id, None, multi_instance)
    }
    
    /// Launch with a specific Job ID (server)
    #[cfg(windows)]
    pub fn launch_with_job_id(cookie: &str, place_id: Option<&str>, job_id: Option<&str>, multi_instance: bool) -> Result<(), String> {
        if !multi_instance {
            Command::new("taskkill").args(["/F", "/IM", "RobloxPlayerBeta.exe"]).creation_flags(CREATE_NO_WINDOW).output().ok();
            Command::new("taskkill").args(["/F", "/IM", "Roblox.exe"]).creation_flags(CREATE_NO_WINDOW).output().ok();
            std::thread::sleep(Duration::from_millis(500));
        }

        let auth_ticket = Self::get_auth_ticket(cookie)?;

        let browser_tracker_id: u64 = rand::random::<u64>() % 1_000_000_000_000;
        
        let place_id_str = place_id.unwrap_or("1818").trim();
        let place_id_num = if place_id_str.is_empty() { "1818" } else { place_id_str };
        
        let launcher_url = if let Some(jid) = job_id {
            format!(
                "https://assetgame.roblox.com/game/PlaceLauncher.ashx?request=RequestGameJob&browserTrackerId={}&placeId={}&gameId={}&isPlayTogetherGame=false&isTeleport=true",
                browser_tracker_id, place_id_num, jid
            )
        } else {
            format!(
                "https://assetgame.roblox.com/game/PlaceLauncher.ashx?request=RequestGame&browserTrackerId={}&placeId={}&isPlayTogetherGame=false",
                browser_tracker_id, place_id_num
            )
        };
        
        let launch_url = format!(
            "roblox-player:1+launchmode:play+gameinfo:{}+launchtime:{}+placelauncherurl:{}+browsertrackerid:{}+robloxLocale:en_us+gameLocale:en_us+channel:+LaunchExp:InApp",
            auth_ticket,
            chrono::Utc::now().timestamp_millis(),
            urlencoding::encode(&launcher_url),
            browser_tracker_id
        );

        Command::new("cmd")
            .args(["/C", "start", "", &launch_url])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| format!("Failed to launch: {}", e))?;

        Ok(())
    }

    #[cfg(not(windows))]
    pub fn set_account_and_launch(_cookie: &str, _place_id: Option<&str>) -> Result<(), String> {
        Err("Account switching only works on Windows".to_string())
    }
    
    #[cfg(windows)]
    pub fn launch_to_server(cookie: &str, place_id: &str, job_id: &str) -> Result<(), String> {
        Self::launch_with_job_id(cookie, Some(place_id), Some(job_id), false)
    }
    
    /// Launch to a VIP/private server with access code and optional link code
    #[cfg(windows)]
    pub fn launch_to_private_server(
        cookie: &str,
        place_id: &str,
        access_code: &str,
        link_code: Option<&str>,
        multi_instance: bool,
    ) -> Result<(), String> {
        if !multi_instance {
            Command::new("taskkill").args(["/F", "/IM", "RobloxPlayerBeta.exe"]).creation_flags(CREATE_NO_WINDOW).output().ok();
            Command::new("taskkill").args(["/F", "/IM", "Roblox.exe"]).creation_flags(CREATE_NO_WINDOW).output().ok();
            std::thread::sleep(Duration::from_millis(500));
        }

        let auth_ticket = Self::get_auth_ticket(cookie)?;
        let browser_tracker_id: u64 = rand::random::<u64>() % 1_000_000_000_000;
        
        let launcher_url = if let Some(lc) = link_code {
            format!(
                "https://assetgame.roblox.com/game/PlaceLauncher.ashx?request=RequestPrivateGame&placeId={}&accessCode={}&linkCode={}",
                place_id, access_code, lc
            )
        } else {
            format!(
                "https://assetgame.roblox.com/game/PlaceLauncher.ashx?request=RequestPrivateGame&placeId={}&accessCode={}",
                place_id, access_code
            )
        };
        
        let launch_url = format!(
            "roblox-player:1+launchmode:play+gameinfo:{}+launchtime:{}+placelauncherurl:{}+browsertrackerid:{}+robloxLocale:en_us+gameLocale:en_us+channel:+LaunchExp:InApp",
            auth_ticket,
            chrono::Utc::now().timestamp_millis(),
            urlencoding::encode(&launcher_url),
            browser_tracker_id
        );

        Command::new("cmd")
            .args(["/C", "start", "", &launch_url])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| format!("Failed to launch: {}", e))?;

        Ok(())
    }
    
    #[cfg(not(windows))]
    pub fn launch_to_private_server(
        _cookie: &str,
        _place_id: &str,
        _access_code: &str,
        _link_code: Option<&str>,
        _multi_instance: bool,
    ) -> Result<(), String> {
        Err("Private server joining only works on Windows".to_string())
    }
    
    #[cfg(not(windows))]
    pub fn launch_to_server(_cookie: &str, _place_id: &str, _job_id: &str) -> Result<(), String> {
        Err("Server joining only works on Windows".to_string())
    }
    
    async fn get_csrf_token_async(client: &reqwest::Client, cookie: &str) -> Result<String, String> {
        let formatted_cookie = format_cookie_str(cookie);
        
        let resp = client
            .post("https://auth.roblox.com/v1/authentication-ticket")
            .header("Cookie", format!(".ROBLOSECURITY={}", formatted_cookie))
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        
        resp.headers()
            .get("x-csrf-token")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
            .ok_or_else(|| "Failed to get CSRF token".to_string())
    }
    
    pub fn logout_other_sessions(cookie: &str) -> Result<String, String> {
        let cookie = cookie.to_string();
        
        run_async(async move {
            let client = reqwest::Client::new();
            let formatted_cookie = format_cookie_str(&cookie);
            let csrf = Self::get_csrf_token_async(&client, &cookie).await?;
            
            let resp = client
                .post("https://www.roblox.com/authentication/signoutfromallsessionsandreauthenticate")
                .header("Cookie", format!(".ROBLOSECURITY={}", formatted_cookie))
                .header("X-CSRF-TOKEN", &csrf)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .header("Referer", "https://www.roblox.com/")
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if resp.status().is_success() {
                // Try to get new cookie from response
                for c in resp.cookies() {
                    if c.name() == ".ROBLOSECURITY" {
                        return Ok(c.value().to_string());
                    }
                }
                Ok(String::new()) // Success but no new cookie
            } else {
                Err(format!("Failed: HTTP {}", resp.status()))
            }
        })
    }
    
    pub fn change_password(cookie: &str, current_password: &str, new_password: &str) -> Result<String, String> {
        let cookie = cookie.to_string();
        let current_password = current_password.to_string();
        let new_password = new_password.to_string();
        
        run_async(async move {
            let client = reqwest::Client::new();
            let formatted_cookie = format_cookie_str(&cookie);
            let csrf = Self::get_csrf_token_async(&client, &cookie).await?;
            
            let body = serde_json::json!({
                "currentPassword": current_password,
                "newPassword": new_password
            });
            
            let resp = client
                .post("https://auth.roblox.com/v2/user/passwords/change")
                .header("Cookie", format!(".ROBLOSECURITY={}", formatted_cookie))
                .header("X-CSRF-TOKEN", &csrf)
                .header("Content-Type", "application/json")
                .header("Referer", "https://www.roblox.com/")
                .json(&body)
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if resp.status().is_success() {
                // Try to get new cookie from response
                for c in resp.cookies() {
                    if c.name() == ".ROBLOSECURITY" {
                        return Ok(c.value().to_string());
                    }
                }
                Ok(String::new())
            } else {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                Err(format!("Failed: HTTP {} - {}", status, body))
            }
        })
    }
    
    pub fn set_display_name(cookie: &str, user_id: u64, new_display_name: &str) -> Result<(), String> {
        let cookie = cookie.to_string();
        let new_display_name = new_display_name.to_string();
        
        run_async(async move {
            let client = reqwest::Client::new();
            let formatted_cookie = format_cookie_str(&cookie);
            let csrf = Self::get_csrf_token_async(&client, &cookie).await?;
            
            let body = serde_json::json!({
                "newDisplayName": new_display_name
            });
            
            let resp = client
                .patch(format!("https://users.roblox.com/v1/users/{}/display-names", user_id))
                .header("Cookie", format!(".ROBLOSECURITY={}", formatted_cookie))
                .header("X-CSRF-TOKEN", &csrf)
                .header("Content-Type", "application/json")
                .header("Referer", "https://www.roblox.com/")
                .json(&body)
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if resp.status().is_success() {
                Ok(())
            } else {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                Err(format!("Failed: HTTP {} - {}", status, body))
            }
        })
    }
    
    pub fn block_user(cookie: &str, user_id: u64) -> Result<(), String> {
        let cookie = cookie.to_string();
        
        run_async(async move {
            let client = reqwest::Client::new();
            let formatted_cookie = format_cookie_str(&cookie);
            let csrf = Self::get_csrf_token_async(&client, &cookie).await?;
            
            let resp = client
                .post(format!("https://accountsettings.roblox.com/v1/users/{}/block", user_id))
                .header("Cookie", format!(".ROBLOSECURITY={}", formatted_cookie))
                .header("X-CSRF-TOKEN", &csrf)
                .header("Content-Type", "application/json")
                .header("Referer", "https://www.roblox.com/")
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if resp.status().is_success() {
                Ok(())
            } else {
                Err(format!("Failed: HTTP {}", resp.status()))
            }
        })
    }
    
    pub fn unblock_user(cookie: &str, user_id: u64) -> Result<(), String> {
        let cookie = cookie.to_string();
        
        run_async(async move {
            let client = reqwest::Client::new();
            let formatted_cookie = format_cookie_str(&cookie);
            let csrf = Self::get_csrf_token_async(&client, &cookie).await?;
            
            let resp = client
                .post(format!("https://accountsettings.roblox.com/v1/users/{}/unblock", user_id))
                .header("Cookie", format!(".ROBLOSECURITY={}", formatted_cookie))
                .header("X-CSRF-TOKEN", &csrf)
                .header("Content-Type", "application/json")
                .header("Referer", "https://www.roblox.com/")
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if resp.status().is_success() {
                Ok(())
            } else {
                Err(format!("Failed: HTTP {}", resp.status()))
            }
        })
    }
    
    pub fn send_friend_request(cookie: &str, user_id: u64) -> Result<(), String> {
        let cookie = cookie.to_string();
        
        run_async(async move {
            let client = reqwest::Client::new();
            let formatted_cookie = format_cookie_str(&cookie);
            let csrf = Self::get_csrf_token_async(&client, &cookie).await?;
            
            let resp = client
                .post(format!("https://friends.roblox.com/v1/users/{}/request-friendship", user_id))
                .header("Cookie", format!(".ROBLOSECURITY={}", formatted_cookie))
                .header("X-CSRF-TOKEN", &csrf)
                .header("Content-Type", "application/json")
                .header("Referer", "https://www.roblox.com/")
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if resp.status().is_success() {
                Ok(())
            } else {
                Err(format!("Failed: HTTP {}", resp.status()))
            }
        })
    }
    
    pub fn get_user_id_by_username(username: &str) -> Result<u64, String> {
        let username = username.to_string();
        
        run_async(async move {
            let client = reqwest::Client::new();
            
            let body = serde_json::json!({
                "usernames": [username],
                "excludeBannedUsers": false
            });
            
            let resp = client
                .post("https://users.roblox.com/v1/usernames/users")
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if !resp.status().is_success() {
                return Err(format!("Failed: HTTP {}", resp.status()));
            }
            
            let data: serde_json::Value = resp.json().await
                .map_err(|e| format!("Failed to parse: {}", e))?;
            
            data.get("data")
                .and_then(|d| d.as_array())
                .and_then(|arr| arr.first())
                .and_then(|user| user.get("id"))
                .and_then(|id| id.as_u64())
                .ok_or_else(|| "User not found".to_string())
        })
    }
    
    pub fn get_username_by_id(user_id: u64) -> Result<String, String> {
        run_async(async move {
            let client = reqwest::Client::new();
            
            let resp = client
                .get(format!("https://users.roblox.com/v1/users/{}", user_id))
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if !resp.status().is_success() {
                return Err(format!("Failed: HTTP {}", resp.status()));
            }
            
            let data: serde_json::Value = resp.json().await
                .map_err(|e| format!("Failed to parse: {}", e))?;
            
            data.get("name")
                .and_then(|n| n.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| "Username not found".to_string())
        })
    }
    
    pub fn get_user_presence_by_username(username: &str) -> Result<(u64, crate::account::UserPresence), String> {
        let username = username.to_string();
        
        run_async(async move {
            let client = reqwest::Client::new();
            
            // First get user ID from username
            let body = serde_json::json!({
                "usernames": [username],
                "excludeBannedUsers": false
            });
            
            let resp = client
                .post("https://users.roblox.com/v1/usernames/users")
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if !resp.status().is_success() {
                return Err(format!("Failed: HTTP {}", resp.status()));
            }
            
            let data: serde_json::Value = resp.json().await
                .map_err(|e| format!("Failed to parse: {}", e))?;
            
            let user_id = data.get("data")
                .and_then(|d| d.as_array())
                .and_then(|arr| arr.first())
                .and_then(|user| user.get("id"))
                .and_then(|id| id.as_u64())
                .ok_or_else(|| "User not found".to_string())?;
            
            // Now get presence for this user
            let presence_body = serde_json::json!({
                "userIds": [user_id]
            });
            
            let presence_resp = client
                .post("https://presence.roblox.com/v1/presence/users")
                .json(&presence_body)
                .send()
                .await
                .map_err(|e| format!("Presence request failed: {}", e))?;
            
            if !presence_resp.status().is_success() {
                return Err("Failed to fetch presence".to_string());
            }
            
            let presence_data: serde_json::Value = presence_resp.json().await
                .map_err(|_| "Failed to parse presence")?;
            
            if let Some(presences) = presence_data.get("userPresences").and_then(|v| v.as_array()) {
                if let Some(p) = presences.first() {
                    use crate::account::{UserPresence, UserPresenceType};
                    
                    let presence_type = p.get("userPresenceType").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
                    let last_location = p.get("lastLocation").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let place_id = p.get("placeId").and_then(|v| v.as_u64());
                    let game_id = p.get("gameId").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let last_online = p.get("lastOnline").and_then(|v| v.as_str()).map(|s| s.to_string());
                    
                    let presence = UserPresence {
                        presence_type: UserPresenceType::from_int(presence_type),
                        last_location,
                        place_id,
                        game_id,
                        last_online,
                        game_name: None,
                    };
                    
                    return Ok((user_id, presence));
                }
            }
            
            Err("Failed to get user presence".to_string())
        })
    }

    pub fn get_avatar_thumbnails(user_ids: &[u64]) -> Result<HashMap<u64, String>, String> {
        if user_ids.is_empty() {
            return Ok(HashMap::new());
        }
        
        let user_ids = user_ids.to_vec();
        
        run_async(async move {
            let client = reqwest::Client::new();
            
            let ids_str: String = user_ids.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            
            let url = format!(
                "https://thumbnails.roblox.com/v1/users/avatar-headshot?userIds={}&size=48x48&format=Png&isCircular=false",
                ids_str
            );
            
            let resp = client
                .get(&url)
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if !resp.status().is_success() {
                return Err(format!("Failed: HTTP {}", resp.status()));
            }
            
            let data: serde_json::Value = resp.json().await
                .map_err(|e| format!("Failed to parse: {}", e))?;
            
            let mut result = HashMap::new();
            
            if let Some(avatars) = data.get("data").and_then(|d| d.as_array()) {
                for avatar in avatars {
                    if let (Some(target_id), Some(image_url)) = (
                        avatar.get("targetId").and_then(|v| v.as_u64()),
                        avatar.get("imageUrl").and_then(|v| v.as_str())
                    ) {
                        if avatar.get("state").and_then(|v| v.as_str()) == Some("Completed") {
                            result.insert(target_id, image_url.to_string());
                        }
                    }
                }
            }
            
            Ok(result)
        })
    }
    
    pub fn get_inventory_info(user_id: u64) -> Result<u32, String> {
        let user_id = user_id;
        
        run_async(async move {
            let client = reqwest::Client::new();
            
            let resp = client
                .get(format!("https://inventory.roblox.com/v1/users/{}/assets/collectibles?limit=10", user_id))
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if !resp.status().is_success() {
                return Err(format!("Failed: HTTP {}", resp.status()));
            }
            
            let data: serde_json::Value = resp.json().await
                .map_err(|e| format!("Failed to parse: {}", e))?;
            
            // Return count of items
            let count = data.get("data")
                .and_then(|d| d.as_array())
                .map(|arr| arr.len() as u32)
                .unwrap_or(0);
            
            Ok(count)
        })
    }
    
    pub fn get_game_icons(universe_ids: &[u64]) -> Result<HashMap<u64, String>, String> {
        if universe_ids.is_empty() {
            return Ok(HashMap::new());
        }
        
        let universe_ids = universe_ids.to_vec();
        
        run_async(async move {
            let client = reqwest::Client::new();
            
            let ids_str: String = universe_ids.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            
            let url = format!(
                "https://thumbnails.roblox.com/v1/games/icons?universeIds={}&returnPolicy=PlaceHolder&size=150x150&format=Png&isCircular=false",
                ids_str
            );
            
            let resp = client
                .get(&url)
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if !resp.status().is_success() {
                return Err(format!("Failed: HTTP {}", resp.status()));
            }
            
            let data: serde_json::Value = resp.json().await
                .map_err(|e| format!("Failed to parse: {}", e))?;
            
            let mut result = HashMap::new();
            
            if let Some(icons) = data.get("data").and_then(|d| d.as_array()) {
                for icon in icons {
                    if let (Some(target_id), Some(image_url)) = (
                        icon.get("targetId").and_then(|v| v.as_u64()),
                        icon.get("imageUrl").and_then(|v| v.as_str())
                    ) {
                        if icon.get("state").and_then(|v| v.as_str()) == Some("Completed") {
                            result.insert(target_id, image_url.to_string());
                        }
                    }
                }
            }
            
            Ok(result)
        })
    }
    
    #[allow(dead_code)]
    pub fn get_universe_id(place_id: &str) -> Result<u64, String> {
        let place_id = place_id.to_string();
        
        run_async(async move {
            let client = reqwest::Client::new();
            
            let resp = client
                .get(format!("https://apis.roblox.com/universes/v1/places/{}/universe", place_id))
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;
            
            if !resp.status().is_success() {
                return Err(format!("Failed: HTTP {}", resp.status()));
            }
            
            let data: serde_json::Value = resp.json().await
                .map_err(|e| format!("Failed to parse: {}", e))?;
            
            data.get("universeId")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| "Universe ID not found".to_string())
        })
    }
}
