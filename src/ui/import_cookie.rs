// Import cookie tab

use eframe::egui::{self, RichText, Color32};
use crate::theme::{self, Colors};
use super::{Action, NexusApp, Tab};

impl NexusApp {
    pub fn render_import_cookie_tab(&mut self, ui: &mut egui::Ui) {
        let account_name = self.selected
            .and_then(|idx| self.data.accounts.get(idx))
            .map(|a| a.username.clone())
            .unwrap_or_else(|| "Unknown".to_string());
        
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(400.0);
                
                // Back button
                if ui.add(theme::secondary_button("← Back to Accounts")).clicked() {
                    self.tab = Tab::Accounts;
                }
                
                ui.add_space(16.0);
                
                ui.label(RichText::new("Import Cookie").color(Colors::TEXT_PRIMARY).size(22.0).strong());
                ui.label(RichText::new(format!("For account: {}", account_name)).color(Colors::ACCENT_BLUE).size(14.0));
                
                ui.add_space(20.0);
                
                // Auto-find card
                egui::Frame::none()
                    .fill(Colors::BG_CARD)
                    .stroke(egui::Stroke::new(2.0, Colors::BORDER_ACCENT))
                    .rounding(egui::Rounding::same(10.0))
                    .inner_margin(egui::Margin::same(16.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("🔍 Auto-Find Cookie").color(Colors::TEXT_PRIMARY).size(15.0).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.add_sized([120.0, 34.0], theme::primary_button("Search")).clicked() {
                                    self.action = Action::FindCookies;
                                }
                            });
                        });
                        
                        ui.add_space(8.0);
                        ui.label(RichText::new("Searches Registry, Chrome, Edge, Firefox").color(Colors::TEXT_MUTED).size(12.0));
                        
                        // Show found cookies
                        if !self.found_cookies.is_empty() {
                            ui.add_space(12.0);
                            
                            egui::Frame::none()
                                .fill(Colors::ACCENT_GREEN.linear_multiply(0.1))
                                .stroke(egui::Stroke::new(1.0, Colors::ACCENT_GREEN.linear_multiply(0.4)))
                                .rounding(egui::Rounding::same(6.0))
                                .inner_margin(egui::Margin::same(12.0))
                                .show(ui, |ui| {
                                    ui.label(RichText::new(format!(" Found {} cookie(s)", self.found_cookies.len())).color(Colors::ACCENT_GREEN).strong());
                                    
                                    let cookies: Vec<_> = self.found_cookies.iter()
                                        .map(|fc| (fc.source.clone(), fc.cookie.clone()))
                                        .collect();
                                    
                                    for (source, cookie) in cookies {
                                        ui.add_space(8.0);
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(&source).color(Colors::TEXT_PRIMARY).size(12.0));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if ui.add(theme::primary_button("Use")).clicked() {
                                                    self.action = Action::UseCookie(cookie);
                                                }
                                            });
                                        });
                                    }
                                });
                        }
                    });
                
                ui.add_space(16.0);
                
                // Manual steps card
                egui::Frame::none()
                    .fill(Colors::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                    .rounding(egui::Rounding::same(10.0))
                    .inner_margin(egui::Margin::same(16.0))
                    .show(ui, |ui| {
                        ui.label(RichText::new("📋 Manual Steps").color(Colors::TEXT_PRIMARY).size(15.0).strong());
                        ui.add_space(12.0);
                        
                        let steps = [
                            "1. Open Chrome/Edge and go to roblox.com",
                            "2. Log into your Roblox account",
                            "3. Press F12 to open Developer Tools",
                            "4. Click the 'Application' tab",
                            "5. Expand 'Cookies' → 'roblox.com'",
                            "6. Find '.ROBLOSECURITY' and copy the value",
                        ];
                        
                        for step in steps {
                            ui.label(RichText::new(step).color(Colors::TEXT_SECONDARY).size(12.0));
                            ui.add_space(4.0);
                        }
                    });
            });
            
            ui.add_space(24.0);
            ui.separator();
            ui.add_space(24.0);
            
            ui.vertical(|ui| {
                ui.label(RichText::new("PASTE COOKIE").size(12.0).color(Colors::TEXT_MUTED));
                ui.add_space(12.0);
                
                egui::Frame::none()
                    .fill(Color32::WHITE)
                    .stroke(egui::Stroke::new(2.0, Colors::BORDER_ACCENT))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), 120.0],
                            egui::TextEdit::multiline(&mut self.import_cookie)
                                .hint_text("Paste your .ROBLOSECURITY cookie here...")
                                .text_color(Color32::from_rgb(20, 20, 30))
                                .frame(false)
                                .font(egui::TextStyle::Monospace)
                        );
                    });
                
                ui.add_space(16.0);
                
                ui.horizontal(|ui| {
                    let can_import = !self.import_cookie.trim().is_empty();
                    ui.add_enabled_ui(can_import, |ui| {
                        if ui.add_sized([160.0, 44.0], theme::primary_button("Import & Verify")).clicked() {
                            self.action = Action::ImportCookie;
                        }
                    });
                    
                    ui.add_space(8.0);
                    
                    if ui.add_sized([80.0, 44.0], theme::secondary_button("Clear")).clicked() {
                        self.import_cookie.clear();
                    }
                });
                
                ui.add_space(20.0);
                
                // Warning
                egui::Frame::none()
                    .fill(Colors::ACCENT_YELLOW.linear_multiply(0.1))
                    .stroke(egui::Stroke::new(1.0, Colors::ACCENT_YELLOW.linear_multiply(0.3)))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("⚠").size(16.0).color(Colors::ACCENT_YELLOW));
                            ui.add_space(8.0);
                            ui.label(RichText::new("Keep your cookie secret! Anyone with it can access your account.").color(Colors::TEXT_SECONDARY).size(11.0));
                        });
                    });
            });
        });
    }
}
