// Private Server and VIP Server utilities

use regex::Regex;
use serde::{Deserialize, Serialize};

/// Types of servers
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ServerType {
    #[default]
    Public,
    VIP,
    Private,
}

impl ServerType {
    pub fn label(&self) -> &'static str {
        match self {
            ServerType::Public => "Public",
            ServerType::VIP => "VIP",
            ServerType::Private => "Private",
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            ServerType::Public => "🌐",
            ServerType::VIP => "⭐",
            ServerType::Private => "🔒",
        }
    }
}

/// VIP Server data from Roblox API
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VipServerData {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "vipServerId")]
    pub vip_server_id: Option<u64>,
    #[serde(rename = "accessCode")]
    pub access_code: Option<String>,
    #[serde(rename = "maxPlayers", default)]
    pub max_players: u32,
    #[serde(default)]
    pub playing: u32,
    pub owner: Option<VipServerOwner>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VipServerOwner {
    #[serde(rename = "hasVerifiedBadge")]
    pub has_verified_badge: Option<bool>,
    pub id: Option<u64>,
    pub name: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

/// Response from VIP servers API
#[derive(Deserialize)]
pub struct VipServersResponse {
    #[serde(rename = "previousPageCursor")]
    pub previous_page_cursor: Option<String>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
    pub data: Vec<VipServerData>,
}

/// Parsed private server link
#[derive(Clone, Debug, Default)]
pub struct PrivateServerLink {
    pub place_id: u64,
    pub link_code: String,
    pub access_code: Option<String>,
}

impl PrivateServerLink {
    /// Parse a private server link from various formats
    /// Supported formats:
    /// - https://www.roblox.com/games/123456789/GameName?privateServerLinkCode=XXXXX
    /// - privateServerLinkCode=XXXXX
    /// - VIP:access_code (legacy format from example)
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();
        
        // Format 1: Full URL with privateServerLinkCode
        if input.contains("privateServerLinkCode=") {
            let link_code_re = Regex::new(r"privateServerLinkCode=([a-zA-Z0-9_-]+)").ok()?;
            let place_id_re = Regex::new(r"/games/(\d+)").ok()?;
            
            let link_code = link_code_re.captures(input)?.get(1)?.as_str().to_string();
            let place_id = place_id_re.captures(input)
                .and_then(|c| c.get(1))
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            
            return Some(Self {
                place_id,
                link_code,
                access_code: None,
            });
        }
        
        // Format 2: VIP:access_code (from example project)
        if input.starts_with("VIP:") {
            let access_code = input.strip_prefix("VIP:")?.to_string();
            return Some(Self {
                place_id: 0,
                link_code: String::new(),
                access_code: Some(access_code),
            });
        }
        
        // Format 3: Just the link code
        if input.len() > 10 && input.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Some(Self {
                place_id: 0,
                link_code: input.to_string(),
                access_code: None,
            });
        }
        
        None
    }
    
    /// Check if this is a valid private server link
    pub fn is_valid(&self) -> bool {
        !self.link_code.is_empty() || self.access_code.is_some()
    }
    
    /// Get the link code or access code for launching
    pub fn get_code(&self) -> Option<&str> {
        if !self.link_code.is_empty() {
            Some(&self.link_code)
        } else {
            self.access_code.as_deref()
        }
    }
}

/// Favorite game that can store private server info
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FavoriteServer {
    pub name: String,
    pub place_id: u64,
    pub private_server_link: Option<String>,
    pub access_code: Option<String>,
    pub last_played: Option<String>,
}

impl FavoriteServer {
    pub fn new(name: String, place_id: u64) -> Self {
        Self {
            name,
            place_id,
            ..Default::default()
        }
    }
    
    pub fn with_private_link(mut self, link: &str) -> Self {
        self.private_server_link = Some(link.to_string());
        self
    }
    
    pub fn with_access_code(mut self, code: &str) -> Self {
        self.access_code = Some(code.to_string());
        self
    }
    
    pub fn is_private(&self) -> bool {
        self.private_server_link.is_some() || self.access_code.is_some()
    }
}

// Helper to run async code
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

/// Fetch VIP servers for a place (requires authentication)
pub fn fetch_vip_servers(cookie: &str, place_id: &str, cursor: Option<&str>) -> Result<VipServersResponse, String> {
    let cookie = cookie.to_string();
    let place_id = place_id.to_string();
    let cursor = cursor.map(|s| s.to_string());
    
    run_async(async move {
        let client = reqwest::Client::new();
        
        let mut url = format!(
            "https://games.roblox.com/v1/games/{}/servers/VIP?sortOrder=Asc&limit=25",
            place_id
        );
        
        if let Some(c) = cursor {
            url.push_str(&format!("&cursor={}", c));
        }
        
        let resp = client
            .get(&url)
            .header("Cookie", format!(".ROBLOSECURITY={}", cookie))
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        
        if !resp.status().is_success() {
            return Err(format!("Failed to fetch VIP servers: HTTP {}", resp.status()));
        }
        
        let data: VipServersResponse = resp.json().await
            .map_err(|e| format!("Failed to parse VIP servers response: {}", e))?;
        
        Ok(data)
    })
}

/// Get access code from a private server link
pub fn get_access_code_from_link(cookie: &str, place_id: &str, link_code: &str) -> Result<String, String> {
    let cookie = cookie.to_string();
    let place_id = place_id.to_string();
    let link_code = link_code.to_string();
    
    run_async(async move {
        let client = reqwest::Client::new();
        
        // First get CSRF token
        let csrf_resp = client
            .post("https://auth.roblox.com/v1/authentication-ticket")
            .header("Cookie", format!(".ROBLOSECURITY={}", cookie))
            .send()
            .await
            .map_err(|e| format!("CSRF request failed: {}", e))?;
        
        let csrf_token = csrf_resp
            .headers()
            .get("x-csrf-token")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_default();
        
        // Request the page with the private server link to get access code
        let url = format!(
            "https://www.roblox.com/games/{}/?privateServerLinkCode={}",
            place_id, link_code
        );
        
        let resp = client
            .get(&url)
            .header("Cookie", format!(".ROBLOSECURITY={}", cookie))
            .header("X-CSRF-TOKEN", &csrf_token)
            .header("Referer", "https://www.roblox.com/")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        
        let body = resp.text().await.map_err(|e| format!("Failed to read response: {}", e))?;
        
        // Parse access code from response
        // Looking for: Roblox.GameLauncher.joinPrivateGame(placeId, 'access-code-here')
        let re = Regex::new(r"Roblox\.GameLauncher\.joinPrivateGame\(\d+,\s*'([^']+)'").map_err(|e| e.to_string())?;
        
        if let Some(caps) = re.captures(&body) {
            if let Some(code) = caps.get(1) {
                return Ok(code.as_str().to_string());
            }
        }
        
        // Alternative pattern
        let re2 = Regex::new(r#"accessCode['":\s]+([a-f0-9-]+)"#).map_err(|e| e.to_string())?;
        if let Some(caps) = re2.captures(&body) {
            if let Some(code) = caps.get(1) {
                return Ok(code.as_str().to_string());
            }
        }
        
        Err("Could not extract access code from private server link".to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_private_server_link() {
        // Full URL format
        let link = PrivateServerLink::parse(
            "https://www.roblox.com/games/123456789/TestGame?privateServerLinkCode=abc123xyz"
        );
        assert!(link.is_some());
        let link = link.unwrap();
        assert_eq!(link.place_id, 123456789);
        assert_eq!(link.link_code, "abc123xyz");
        
        // VIP format
        let link = PrivateServerLink::parse("VIP:some-access-code-here");
        assert!(link.is_some());
        let link = link.unwrap();
        assert_eq!(link.access_code, Some("some-access-code-here".to_string()));
        
        // Just link code
        let link = PrivateServerLink::parse("abc123xyz-defg-hijk");
        assert!(link.is_some());
    }
}
