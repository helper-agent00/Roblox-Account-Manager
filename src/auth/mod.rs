// Auth module - cookie detection, login utilities, and multi-instance

mod cookie_finder;
mod multi_instance;
mod web_login;

pub use cookie_finder::{CookieFinder, FoundCookie};
pub use multi_instance::MultiInstanceManager;
pub use web_login::{LoginResult, WebLoginSession};
