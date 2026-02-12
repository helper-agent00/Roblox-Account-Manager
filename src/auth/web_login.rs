use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashSet;
use futures::StreamExt;

#[derive(Clone, Debug)]
pub enum LoginResult {
    Success {
        cookie: String,
        user_id: u64,
        username: String,
        display_name: String,
    },
    Cancelled,
    Error(String),
}

#[derive(Clone, Debug)]
pub enum BrowserStatus {
    Launching,
    WaitingForLogin,
    LoggedIn,
}

pub struct WebLoginSession {
    result_receiver: Receiver<LoginResult>,
    cancel_flag: Arc<Mutex<bool>>,
}

impl WebLoginSession {
    /// Start a new browser login session with existing user IDs to skip
    pub fn start_with_existing_users(existing_user_ids: HashSet<u64>) -> Result<Self, String> {
        let (result_sender, result_receiver) = channel();
        let (status_sender, _status_receiver) = channel();
        let cancel_flag = Arc::new(Mutex::new(false));
        let cancel_flag_clone = cancel_flag.clone();
        let existing_ids = existing_user_ids.clone();
        
        thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime");
            
            rt.block_on(async {
                run_browser_session(result_sender, status_sender, cancel_flag_clone, existing_ids).await;
            });
        });
        
        Ok(Self {
            result_receiver,
            cancel_flag,
        })
    }
    
    pub fn try_get_result(&self) -> Option<LoginResult> {
        match self.result_receiver.try_recv() {
            Ok(result) => Some(result),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(LoginResult::Cancelled),
        }
    }
    
    pub fn cancel(&self) {
        if let Ok(mut flag) = self.cancel_flag.lock() {
            *flag = true;
        }
    }
}

async fn run_browser_session(
    result_sender: Sender<LoginResult>,
    status_sender: Sender<BrowserStatus>,
    cancel_flag: Arc<Mutex<bool>>,
    existing_user_ids: HashSet<u64>,
) {
    use chromiumoxide::browser::{Browser, BrowserConfig};
    
    let _ = status_sender.send(BrowserStatus::Launching);
    
    let config = match BrowserConfig::builder()
        .window_size(900, 700)
        .with_head() 
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            let _ = result_sender.send(LoginResult::Error(format!("Failed to configure browser: {}", e)));
            return;
        }
    };
    
    let (mut browser, mut handler) = match Browser::launch(config).await {
        Ok(b) => b,
        Err(e) => {
            let _ = result_sender.send(LoginResult::Error(format!("Failed to launch browser: {}. Make sure Chrome/Chromium is installed.", e)));
            return;
        }
    };
    
    let handler_task = tokio::spawn(async move {
        while let Some(_event) = handler.next().await {
            
        }
    });
    
    let page = match browser.new_page("https://www.roblox.com/login").await {
        Ok(p) => p,
        Err(e) => {
            let _ = result_sender.send(LoginResult::Error(format!("Failed to open page: {}", e)));
            return;
        }
    };
    
    let _ = status_sender.send(BrowserStatus::WaitingForLogin);
    
    let check_interval = std::time::Duration::from_millis(1500);
    let timeout = std::time::Duration::from_secs(300); 
    let start = std::time::Instant::now();
    
    loop {
        if let Ok(cancelled) = cancel_flag.lock() {
            if *cancelled {
                let _ = result_sender.send(LoginResult::Cancelled);
                break;
            }
        }
        
        if start.elapsed() > timeout {
            let _ = result_sender.send(LoginResult::Error("Login timed out after 5 minutes".to_string()));
            break;
        }
        
        // Try to get cookies from the page
        if let Ok(cookies) = page.get_cookies().await {
            for cookie in cookies {
                if cookie.name == ".ROBLOSECURITY" && cookie.value.len() > 100 {
                    // Found the security cookie! Validate it
                    let cookie_value = cookie.value.clone();
                    
                    match crate::api::RobloxApi::validate_cookie(&cookie_value) {
                        Ok((user_id, display_name)) => {
                            // Skip if we already have this user
                            if existing_user_ids.contains(&user_id) {
                                // Already have this account, keep waiting for a different login
                                continue;
                            }
                            
                            let _ = status_sender.send(BrowserStatus::LoggedIn);
                            
                            let username = crate::api::RobloxApi::get_username_by_id(user_id)
                                .unwrap_or_else(|_| display_name.clone());
                            
                            let _ = result_sender.send(LoginResult::Success {
                                cookie: cookie_value,
                                user_id,
                                username,
                                display_name,
                            });
                            
                            let _ = browser.close().await;
                            handler_task.abort();
                            return;
                        }
                        Err(_) => {
                        }
                    }
                }
            }
        }
        
        tokio::time::sleep(check_interval).await;
    }
    
    // Cleanup
    let _ = browser.close().await;
    handler_task.abort();
}
