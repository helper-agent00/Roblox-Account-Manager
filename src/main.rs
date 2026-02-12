#![windows_subsystem = "windows"]

mod account;
mod api;
mod auth;
mod games;
mod theme;
mod ui;

use eframe::egui::{self, RichText};
use theme::Colors;
use ui::{NexusApp, Tab};

#[cfg(windows)]
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, MenuId},
    TrayIcon, TrayIconBuilder,
};
#[cfg(windows)]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(windows)]
use std::sync::OnceLock;

#[cfg(windows)]
static SHOW_ID: OnceLock<MenuId> = OnceLock::new();
#[cfg(windows)]
static QUIT_ID: OnceLock<MenuId> = OnceLock::new();
#[cfg(windows)]
static MINIMIZED_TO_TRAY: AtomicBool = AtomicBool::new(false);

#[cfg(windows)]
mod win_utils {
    use winapi::um::winuser::{FindWindowW, ShowWindow, SetForegroundWindow, SW_HIDE, SW_SHOW, SW_RESTORE};
    use std::ptr::null_mut;
    
    pub fn find_window() -> *mut winapi::shared::windef::HWND__ {
        unsafe {
            let title: Vec<u16> = "Nexus Account Manager\0".encode_utf16().collect();
            FindWindowW(null_mut(), title.as_ptr())
        }
    }
    
    pub fn hide_window() {
        let hwnd = find_window();
        if !hwnd.is_null() {
            unsafe { ShowWindow(hwnd, SW_HIDE); }
        }
    }
    
    pub fn show_window() {
        let hwnd = find_window();
        if !hwnd.is_null() {
            unsafe {
                ShowWindow(hwnd, SW_SHOW);
                ShowWindow(hwnd, SW_RESTORE);
                SetForegroundWindow(hwnd);
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    #[cfg(windows)]
    let tray_menu = {
        let menu = Menu::new();
        let show_item = MenuItem::new("Show", true, None);
        let quit_item = MenuItem::new("Quit", true, None);
        SHOW_ID.set(show_item.id().clone()).ok();
        QUIT_ID.set(quit_item.id().clone()).ok();
        menu.append(&show_item).ok();
        menu.append(&quit_item).ok();
        menu
    };
    
    #[cfg(windows)]
    std::thread::spawn(|| {
        loop {
            if let Ok(event) = MenuEvent::receiver().recv() {
                if let Some(show_id) = SHOW_ID.get() {
                    if &event.id == show_id {
                        MINIMIZED_TO_TRAY.store(false, Ordering::SeqCst);
                        win_utils::show_window();
                    }
                }
                if let Some(quit_id) = QUIT_ID.get() {
                    if &event.id == quit_id {
                        std::process::exit(0);
                    }
                }
            }
        }
    });
    
    #[cfg(windows)]
    let _tray_icon: Option<TrayIcon> = {
        let icon_data = include_bytes!("../icon.ico");
        let icon = load_icon_from_ico(icon_data).unwrap_or_else(|| {
            let width = 32u32;
            let height = 32u32;
            let mut rgba = vec![0u8; (width * height * 4) as usize];
            for y in 0..height {
                for x in 0..width {
                    let idx = ((y * width + x) * 4) as usize;
                    rgba[idx] = 59;
                    rgba[idx + 1] = 130;
                    rgba[idx + 2] = 246;
                    rgba[idx + 3] = 255;
                }
            }
            tray_icon::Icon::from_rgba(rgba, width, height).ok()
        });
        
        if let Some(icon) = icon {
            TrayIconBuilder::new()
                .with_tooltip("Nexus Account Manager")
                .with_icon(icon)
                .with_menu(Box::new(tray_menu))
                .build()
                .ok()
        } else {
            None
        }
    };
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([950.0, 700.0])
            .with_min_inner_size([850.0, 550.0])
            .with_title("Nexus Account Manager"),
        ..Default::default()
    };

    eframe::run_native(
        "Nexus Account Manager",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            theme::setup_dark_theme(&cc.egui_ctx);
            Ok(Box::new(NexusApp::new()))
        }),
    )
}

#[cfg(windows)]
fn load_icon_from_ico(data: &[u8]) -> Option<Option<tray_icon::Icon>> {
    if data.len() < 22 {
        return None;
    }
    
    let count = u16::from_le_bytes([data[4], data[5]]) as usize;
    if count == 0 {
        return None;
    }
    
    let mut best_offset = 0usize;
    let mut best_size = 0u32;
    let mut best_area = 0u32;
    
    for i in 0..count {
        let entry_offset = 6 + i * 16;
        if entry_offset + 16 > data.len() {
            break;
        }
        
        let width = if data[entry_offset] == 0 { 256u32 } else { data[entry_offset] as u32 };
        let height = if data[entry_offset + 1] == 0 { 256u32 } else { data[entry_offset + 1] as u32 };
        let size = u32::from_le_bytes([
            data[entry_offset + 8],
            data[entry_offset + 9],
            data[entry_offset + 10],
            data[entry_offset + 11],
        ]);
        let offset = u32::from_le_bytes([
            data[entry_offset + 12],
            data[entry_offset + 13],
            data[entry_offset + 14],
            data[entry_offset + 15],
        ]) as usize;
        
        if width == 32 && height == 32 {
            best_offset = offset;
            best_size = size;
            break;
        } else if width * height > best_area {
            best_offset = offset;
            best_size = size;
            best_area = width * height;
        }
    }
    
    if best_size == 0 || best_offset + best_size as usize > data.len() {
        return None;
    }
    
    let image_data = &data[best_offset..best_offset + best_size as usize];
    
    if image_data.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        if let Ok(img) = image::load_from_memory(image_data) {
            let rgba = img.to_rgba8();
            let (w, h) = rgba.dimensions();
            return Some(tray_icon::Icon::from_rgba(rgba.into_raw(), w, h).ok());
        }
    }
    
    None
}

impl eframe::App for NexusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(windows)]
        {
            self.minimized_to_tray = MINIMIZED_TO_TRAY.load(Ordering::SeqCst);
        }
        
        if !self.startup_fetch_done && !self.data.accounts.is_empty() {
            self.startup_fetch_done = true;
            self.fetch_presence_and_avatars();
        }
        
        if !self.game_icons_loaded {
            self.game_icons_loaded = true;
            self.load_game_icons();
        }
        
        if self.last_presence_refresh.elapsed().as_secs() >= 60 {
            self.refresh_presence_only();
        }
        
        self.check_browser_login_result();
        
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    if let Some(ref bytes) = file.bytes {
                        if let Ok(content) = String::from_utf8(bytes.to_vec()) {
                            let trimmed = content.trim();
                            if trimmed.contains("_|WARNING:-DO-NOT-SHARE") || trimmed.len() > 100 {
                                self.action = ui::Action::ImportDroppedCookie(trimmed.to_string());
                                self.drag_drop_active = false;
                                return;
                            }
                        }
                    } else if let Some(ref path) = file.path {
                        if let Ok(content) = std::fs::read_to_string(path) {
                            let trimmed = content.trim();
                            if trimmed.contains("_|WARNING:-DO-NOT-SHARE") || trimmed.len() > 100 {
                                self.action = ui::Action::ImportDroppedCookie(trimmed.to_string());
                                self.drag_drop_active = false;
                                return;
                            }
                        }
                    }
                }
            }
            self.drag_drop_active = i.raw.hovered_files.len() > 0;
        });
        
        self.process_action();
        
        egui::SidePanel::left("nav_sidebar")
            .exact_width(172.0)
            .resizable(false)
            .frame(egui::Frame::none()
                .fill(Colors::NAV_BG)
                .stroke(egui::Stroke::new(1.0, Colors::NAV_BORDER))
                .inner_margin(egui::Margin::same(0.0))
            )
            .show(ctx, |ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);
                
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.label(RichText::new("N").size(24.0).color(Colors::ACCENT_BLUE).strong());
                    ui.label(RichText::new("EXUS").size(15.0).color(Colors::TEXT_MUTED).strong());
                });
                ui.add_space(3.0);
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.label(RichText::new("Account Manager").size(10.5).color(Colors::TEXT_MUTED));
                });
                
                ui.add_space(16.0);
                
                let sep_rect = ui.available_rect_before_wrap();
                ui.painter().hline(
                    sep_rect.left() + 14.0..=sep_rect.right() - 14.0,
                    sep_rect.top(),
                    egui::Stroke::new(0.5, Colors::NAV_BORDER),
                );
                ui.add_space(16.0);
                
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.label(RichText::new("MAIN").size(9.0).color(Colors::TEXT_MUTED).strong());
                });
                ui.add_space(8.0);
                
                self.render_sidebar_button(ui, Tab::Accounts, "◆", "Accounts");
                self.render_sidebar_button(ui, Tab::AddAccount, "+", "Add Account");
                self.render_sidebar_button(ui, Tab::Games, "▣", "Games");
                self.render_sidebar_button(ui, Tab::Servers, "◎", "Servers");
                
                ui.add_space(16.0);
                
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.label(RichText::new("TOOLS").size(9.0).color(Colors::TEXT_MUTED).strong());
                });
                ui.add_space(8.0);
                
                self.render_sidebar_button(ui, Tab::AccountUtils, "⚙", "Utilities");
                self.render_sidebar_button(ui, Tab::About, "ⓘ", "About");
                self.render_sidebar_button(ui, Tab::Settings, "☰", "Settings");
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(12.0);
                    
                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        let valid = self.data.accounts.iter()
                            .filter(|a| a.status == crate::account::AccountStatus::Valid)
                            .count();
                        ui.label(RichText::new(format!("{}/{}", valid, self.data.accounts.len()))
                            .size(11.0).color(Colors::ACCENT_GREEN));
                        ui.add_space(4.0);
                        ui.label(RichText::new("valid")
                            .size(10.0).color(Colors::TEXT_MUTED));
                    });
                    
                    ui.add_space(8.0);
                    
                    #[cfg(windows)]
                    {
                        ui.horizontal(|ui| {
                            ui.add_space(12.0);
                            if ui.add(egui::Button::new(
                                RichText::new("⏷ Minimize").size(10.0).color(Colors::TEXT_MUTED)
                            ).fill(egui::Color32::TRANSPARENT).stroke(egui::Stroke::NONE))
                            .on_hover_text("Minimize to tray")
                            .clicked() {
                                if self.data.minimize_to_tray {
                                    MINIMIZED_TO_TRAY.store(true, Ordering::SeqCst);
                                    self.minimized_to_tray = true;
                                    win_utils::hide_window();
                                } else {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                                }
                            }
                        });
                        ui.add_space(4.0);
                    }
                });
            });
        
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Colors::BG_DARK).inner_margin(egui::Margin::same(0.0)))
            .show(ctx, |ui| {
                if self.drag_drop_active {
                    let screen_rect = ui.max_rect();
                    ui.painter().rect_filled(
                        screen_rect,
                        egui::Rounding::same(0.0),
                        egui::Color32::from_rgba_unmultiplied(10, 80, 180, 120),
                    );
                    ui.painter().rect_stroke(
                        screen_rect.shrink(8.0),
                        egui::Rounding::same(14.0),
                        egui::Stroke::new(2.5, Colors::ACCENT_BLUE),
                    );
                    ui.put(
                        screen_rect,
                        egui::Label::new(
                            RichText::new("🍪  Drop Cookie File Here")
                                .size(26.0)
                                .color(egui::Color32::WHITE)
                                .strong()
                        ),
                    );
                    return;
                }
                
                if !self.status.is_empty() {
                    let color = if self.status_error { Colors::ACCENT_RED } else { Colors::ACCENT_GREEN };
                    let bg_color = if self.status_error { 
                        Colors::ACCENT_RED.linear_multiply(0.08) 
                    } else { 
                        Colors::ACCENT_GREEN.linear_multiply(0.08) 
                    };
                    
                    egui::Frame::none()
                        .fill(bg_color)
                        .stroke(egui::Stroke::new(0.5, color.linear_multiply(0.3)))
                        .inner_margin(egui::Margin::symmetric(16.0, 6.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                theme::draw_status_circle(ui, color, 6.0);
                                ui.add_space(6.0);
                                ui.label(RichText::new(&self.status).color(color).size(12.0));
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.add(egui::Button::new(
                                        RichText::new("✕").size(10.0).color(Colors::TEXT_MUTED)
                                    ).fill(egui::Color32::TRANSPARENT).stroke(egui::Stroke::NONE)).clicked() {
                                        self.status.clear();
                                    }
                                });
                            });
                        });
                }
                
                egui::Frame::none()
                    .fill(Colors::BG_DARK)
                    .inner_margin(egui::Margin::same(20.0))
                    .show(ui, |ui| {
                        match self.tab {
                            Tab::Accounts => self.render_accounts_tab(ui),
                            Tab::AddAccount => self.render_add_account_tab(ui),
                            Tab::Games => self.render_games_tab(ui),
                            Tab::Servers => self.render_servers_tab(ui),
                            Tab::AccountUtils => self.render_account_utils_tab(ui),
                            Tab::ImportCookie => self.render_import_cookie_tab(ui),
                            Tab::Settings => self.render_settings_tab(ui),
                            Tab::About => self.render_about_tab(ui),
                        }
                    });
            });
        
        if self.cookie_modal_show {
            self.render_cookie_modal(ctx);
        }
        
        if self.follow_user_show {
            self.render_follow_user_modal(ctx);
        }
        
        if self.server_browser.vip_access_code_show {
            self.render_vip_access_code_modal(ctx);
        }
    }
}
