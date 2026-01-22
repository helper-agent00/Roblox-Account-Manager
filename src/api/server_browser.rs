// Server browser API

use serde::{Deserialize, Serialize};

// Server data from Roblox API (supports both public and VIP)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ServerData {
    pub id: String,
    #[serde(rename = "maxPlayers")]
    pub max_players: u32,
    pub playing: u32,
    pub fps: Option<f32>,
    pub ping: Option<u32>,
    // VIP server fields
    pub name: Option<String>,
    #[serde(rename = "vipServerId")]
    pub vip_server_id: Option<u64>,
    #[serde(rename = "accessCode")]
    pub access_code: Option<String>,
    #[serde(skip)]
    pub server_type: ServerType,
}

impl ServerData {
    // Get fill percentage
    pub fn fill_percent(&self) -> f32 {
        if self.max_players == 0 {
            0.0
        } else {
            (self.playing as f32 / self.max_players as f32) * 100.0
        }
    }
    
    // Check if server is full
    pub fn is_full(&self) -> bool {
        self.playing >= self.max_players
    }
    
    // Check if server has players
    pub fn has_players(&self) -> bool {
        self.playing > 0
    }
    
    // Check if this is a VIP server
    pub fn is_vip(&self) -> bool {
        self.server_type == ServerType::VIP || self.access_code.is_some()
    }
    
    // Get display name (VIP servers have names)
    pub fn display_name(&self) -> String {
        if let Some(ref name) = self.name {
            if !name.is_empty() {
                return name.clone();
            }
        }
        // Truncate job ID for display
        if self.id.len() > 12 {
            format!("{}...", &self.id[..12])
        } else {
            self.id.clone()
        }
    }
    
    // Get the job ID or access code for launching
    pub fn get_join_id(&self) -> &str {
        if self.is_vip() {
            self.access_code.as_deref().unwrap_or(&self.id)
        } else {
            &self.id
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
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

// Response from servers API
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ServersResponse {
    #[serde(rename = "previousPageCursor")]
    pub previous_page_cursor: Option<String>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
    pub data: Vec<ServerData>,
}

// Sort column for server list
#[derive(Default, Clone, Copy, PartialEq)]
pub enum SortColumn {
    #[default]
    None,
    Players,
    Ping,
    Fill,
}

// Sort direction
#[derive(Default, Clone, Copy, PartialEq)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

impl SortDirection {
    pub fn toggle(&self) -> Self {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }
    
    pub fn arrow(&self) -> &'static str {
        match self {
            SortDirection::Ascending => "^",
            SortDirection::Descending => "v",
        }
    }
}

// Server browser state
#[derive(Default)]
pub struct ServerBrowser {
    pub servers: Vec<ServerData>,
    pub vip_servers: Vec<ServerData>,
    pub loading: bool,
    pub error: Option<String>,
    pub current_place_id: Option<String>,
    pub current_game_name: Option<String>,
    pub next_cursor: Option<String>,
    pub vip_next_cursor: Option<String>,
    pub selected_server: Option<usize>,
    pub sort_column: SortColumn,
    pub sort_direction: SortDirection,
    pub show_vip_servers: bool,
    // Private server link input
    pub private_server_input: String,
    // VIP access code modal
    pub vip_access_code_input: String,
    pub vip_access_code_show: bool,
    pub vip_pending_server_idx: Option<usize>,
    pub vip_pending_account_idx: Option<usize>,
}

impl ServerBrowser {
    pub fn new() -> Self {
        Self::default()
    }
    
    // Clear current servers
    pub fn clear(&mut self) {
        self.servers.clear();
        self.vip_servers.clear();
        self.next_cursor = None;
        self.vip_next_cursor = None;
        self.selected_server = None;
        self.error = None;
        self.current_game_name = None;
    }
    
    // Get active server list based on current view
    pub fn active_servers(&self) -> &Vec<ServerData> {
        if self.show_vip_servers {
            &self.vip_servers
        } else {
            &self.servers
        }
    }
    
    // Has more pages to load
    pub fn has_more(&self) -> bool {
        if self.show_vip_servers {
            self.vip_next_cursor.is_some()
        } else {
            self.next_cursor.is_some()
        }
    }
    
    // Toggle sort on a column
    pub fn toggle_sort(&mut self, column: SortColumn) {
        if self.sort_column == column {
            self.sort_direction = self.sort_direction.toggle();
        } else {
            self.sort_column = column;
            self.sort_direction = SortDirection::Descending; // Default to descending for new column
        }
    }
    
    // Get sorted server indices
    pub fn get_sorted_indices(&self) -> Vec<usize> {
        let servers = self.active_servers();
        let mut indices: Vec<usize> = (0..servers.len()).collect();
        
        match self.sort_column {
            SortColumn::None => {}
            SortColumn::Players => {
                indices.sort_by(|&a, &b| {
                    let cmp = servers[a].playing.cmp(&servers[b].playing);
                    if self.sort_direction == SortDirection::Ascending { cmp } else { cmp.reverse() }
                });
            }
            SortColumn::Ping => {
                indices.sort_by(|&a, &b| {
                    let ping_a = servers[a].ping.unwrap_or(9999);
                    let ping_b = servers[b].ping.unwrap_or(9999);
                    let cmp = ping_a.cmp(&ping_b);
                    if self.sort_direction == SortDirection::Ascending { cmp } else { cmp.reverse() }
                });
            }
            SortColumn::Fill => {
                indices.sort_by(|&a, &b| {
                    let cmp = servers[a].fill_percent().partial_cmp(&servers[b].fill_percent()).unwrap_or(std::cmp::Ordering::Equal);
                    if self.sort_direction == SortDirection::Ascending { cmp } else { cmp.reverse() }
                });
            }
        }
        
        indices
    }
    
    // Get selected server's job ID
    #[allow(dead_code)]
    pub fn get_selected_job_id(&self) -> Option<&str> {
        let servers = self.active_servers();
        self.selected_server
            .and_then(|idx| servers.get(idx))
            .map(|s| s.get_join_id())
    }
    
    // Get selected server
    pub fn get_selected_server(&self) -> Option<&ServerData> {
        let servers = self.active_servers();
        self.selected_server.and_then(|idx| servers.get(idx))
    }
}

// Helper to run async code on a separate thread to avoid runtime nesting
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

// Fetch servers for a place
pub fn fetch_servers(place_id: &str, cursor: Option<&str>) -> Result<ServersResponse, String> {
    let place_id = place_id.to_string();
    let cursor = cursor.map(|s| s.to_string());
    
    run_async(async move {
        let client = reqwest::Client::new();
        
        let mut url = format!(
            "https://games.roblox.com/v1/games/{}/servers/public?sortOrder=Asc&limit=25",
            place_id
        );
        
        if let Some(c) = cursor {
            url.push_str(&format!("&cursor={}", c));
        }
        
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        
        if !resp.status().is_success() {
            return Err(format!("Failed to fetch servers: HTTP {}", resp.status()));
        }
        
        let data: ServersResponse = resp.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        Ok(data)
    })
}

// Get a random server from available servers
pub fn get_random_server(place_id: &str) -> Result<String, String> {
    let response = fetch_servers(place_id, None)?;
    
    // Filter to servers that have players and aren't full
    let available: Vec<_> = response.data.iter()
        .filter(|s| s.has_players() && !s.is_full())
        .collect();
    
    if available.is_empty() {
        return Err("No available servers found".to_string());
    }
    
    // Pick a random one
    let idx = rand::random::<usize>() % available.len();
    Ok(available[idx].id.clone())
}
