use eframe::egui::{self, RichText, Color32};
use crate::theme::Colors;
use super::{Action, NexusApp};

impl NexusApp {
    pub fn render_add_account_tab(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let available_width = ui.available_width();
                
                ui.vertical_centered(|ui| {
                    let card_width = 520.0_f32.min(available_width - 48.0);
                    
                    ui.add_space(32.0);
                    
                    ui.label(RichText::new("Add Account")
                        .color(Colors::TEXT_PRIMARY)
                        .size(28.0)
                        .strong());
                    ui.add_space(8.0);
                    ui.label(RichText::new("Choose how you'd like to add your Roblox account")
                        .color(Colors::TEXT_SECONDARY)
                        .size(14.0));
                    
                    ui.add_space(32.0);
                    
                    self.render_browser_login_card(ui, card_width);
                    
                    ui.add_space(16.0);
                    
                    self.render_divider(ui, card_width);
                    
                    ui.add_space(16.0);
                    
                    self.render_cookie_import_card(ui, card_width);
                    
                    ui.add_space(24.0);
                    self.render_security_notice(ui, card_width);
                    
                    ui.add_space(32.0);
                });
            });
    }
    
    fn render_browser_login_card(&mut self, ui: &mut egui::Ui, width: f32) {
        ui.allocate_ui_with_layout(
            egui::vec2(width, 0.0),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                let is_active = self.browser_login_session.is_some();
                
                egui::Frame::none()
                    .fill(if is_active { 
                        Colors::ACCENT_BLUE.linear_multiply(0.15) 
                    } else { 
                        Colors::ACCENT_BLUE.linear_multiply(0.08) 
                    })
                    .stroke(egui::Stroke::new(
                        if is_active { 2.0 } else { 1.5 }, 
                        Colors::ACCENT_BLUE.linear_multiply(if is_active { 0.8 } else { 0.4 })
                    ))
                    .rounding(egui::Rounding::same(12.0))
                    .inner_margin(egui::Margin::same(20.0))
                    .show(ui, |ui| {
                        ui.set_min_width(width - 44.0);
                        
                        ui.horizontal(|ui| {
                            egui::Frame::none()
                                .fill(Colors::ACCENT_BLUE.linear_multiply(0.2))
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::same(8.0))
                                .show(ui, |ui| {
                                    ui.label(RichText::new("🌐").size(20.0));
                                });
                            
                            ui.add_space(12.0);
                            
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Browser Login")
                                        .color(Colors::TEXT_PRIMARY)
                                        .size(17.0)
                                        .strong());
                                    ui.add_space(8.0);
                                    egui::Frame::none()
                                        .fill(Colors::ACCENT_GREEN.linear_multiply(0.2))
                                        .rounding(egui::Rounding::same(4.0))
                                        .inner_margin(egui::Margin::symmetric(6.0, 2.0))
                                        .show(ui, |ui| {
                                            ui.label(RichText::new("RECOMMENDED")
                                                .color(Colors::ACCENT_GREEN)
                                                .size(9.0)
                                                .strong());
                                        });
                                });
                                ui.label(RichText::new("Supports 2FA • Most secure • Auto-detection")
                                    .color(Colors::TEXT_MUTED)
                                    .size(12.0));
                            });
                        });
                        
                        ui.add_space(16.0);
                        
                        ui.label(RichText::new(
                            "Opens a secure browser window where you can log in to Roblox normally. \
                            Nexus will automatically detect your login and import your account.")
                            .color(Colors::TEXT_SECONDARY)
                            .size(13.0));
                        
                        ui.add_space(16.0);
                        
                        if is_active {
                            egui::Frame::none()
                                .fill(Colors::BG_DARK)
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::same(16.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.spinner();
                                        ui.add_space(12.0);
                                        ui.vertical(|ui| {
                                            ui.label(RichText::new("Waiting for login...")
                                                .color(Colors::ACCENT_BLUE)
                                                .size(14.0)
                                                .strong());
                                            ui.label(RichText::new("Complete the login in the browser window")
                                                .color(Colors::TEXT_MUTED)
                                                .size(12.0));
                                        });
                                    });
                                });
                            
                            ui.add_space(12.0);
                            
                            ui.horizontal(|ui| {
                                let cancel_btn = egui::Button::new(
                                    RichText::new("Cancel").color(Colors::TEXT_SECONDARY).size(13.0)
                                )
                                .fill(Colors::BG_LIGHT)
                                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                                .rounding(egui::Rounding::same(6.0));
                                
                                if ui.add_sized([100.0, 36.0], cancel_btn).clicked() {
                                    if let Some(ref session) = self.browser_login_session {
                                        session.cancel();
                                    }
                                    self.browser_login_session = None;
                                    self.set_status("Browser login cancelled", false);
                                }
                            });
                        } else {
                            let btn = egui::Button::new(
                                RichText::new("🔓  Open Browser Login")
                                    .size(15.0)
                                    .color(Color32::WHITE)
                                    .strong()
                            )
                            .fill(Colors::ACCENT_BLUE)
                            .rounding(egui::Rounding::same(8.0));
                            
                            if ui.add_sized([ui.available_width(), 44.0], btn).clicked() {
                                self.action = Action::StartBrowserLogin;
                            }
                        }
                    });
            }
        );
    }
    
    fn render_cookie_import_card(&mut self, ui: &mut egui::Ui, width: f32) {
        ui.allocate_ui_with_layout(
            egui::vec2(width, 0.0),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                egui::Frame::none()
                    .fill(Colors::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                    .rounding(egui::Rounding::same(12.0))
                    .inner_margin(egui::Margin::same(20.0))
                    .show(ui, |ui| {
                        ui.set_min_width(width - 44.0);
                        
                        ui.horizontal(|ui| {
                            egui::Frame::none()
                                .fill(Colors::ACCENT_YELLOW.linear_multiply(0.15))
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::same(8.0))
                                .show(ui, |ui| {
                                    ui.label(RichText::new("🍪").size(20.0));
                                });
                            
                            ui.add_space(12.0);
                            
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Import Cookie")
                                        .color(Colors::TEXT_PRIMARY)
                                        .size(17.0)
                                        .strong());
                                    ui.add_space(8.0);
                                    egui::Frame::none()
                                        .fill(Colors::TEXT_MUTED.linear_multiply(0.2))
                                        .rounding(egui::Rounding::same(4.0))
                                        .inner_margin(egui::Margin::symmetric(6.0, 2.0))
                                        .show(ui, |ui| {
                                            ui.label(RichText::new("ADVANCED")
                                                .color(Colors::TEXT_MUTED)
                                                .size(9.0)
                                                .strong());
                                        });
                                });
                                ui.label(RichText::new("Paste .ROBLOSECURITY cookie directly")
                                    .color(Colors::TEXT_MUTED)
                                    .size(12.0));
                            });
                        });
                        
                        ui.add_space(16.0);
                        
                        let find_btn = egui::Button::new(
                            RichText::new("🔍 Find Cookies on System")
                                .size(13.0)
                                .color(Colors::TEXT_PRIMARY)
                        )
                        .fill(Colors::BG_LIGHT)
                        .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                        .rounding(egui::Rounding::same(6.0));
                        
                        if ui.add_sized([ui.available_width(), 36.0], find_btn).clicked() {
                            self.action = Action::FindCookies;
                        }
                        
                        if !self.found_cookies.is_empty() {
                            ui.add_space(12.0);
                            
                            ui.label(RichText::new(format!("Found {} cookie(s):", self.found_cookies.len()))
                                .color(Colors::ACCENT_GREEN)
                                .size(12.0)
                                .strong());
                            
                            ui.add_space(8.0);
                            
                            let cookies: Vec<_> = self.found_cookies.iter()
                                .map(|c| (c.cookie.clone(), c.source.clone()))
                                .collect();
                            
                            let mut cookie_to_use: Option<String> = None;
                            
                            for (cookie, source) in &cookies {
                                egui::Frame::none()
                                    .fill(Colors::BG_DARK)
                                    .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(egui::Margin::same(10.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            let source_icon = match source.as_str() {
                                                "Windows Registry" => "🪟",
                                                "Google Chrome" => "🌐",
                                                "Microsoft Edge" => "🌐",
                                                "Mozilla Firefox" => "🦊",
                                                _ => "🍪",
                                            };
                                            ui.label(RichText::new(source_icon).size(14.0));
                                            ui.add_space(4.0);
                                            ui.label(RichText::new(source)
                                                .color(Colors::ACCENT_BLUE)
                                                .size(12.0)
                                                .strong());
                                        });
                                        
                                        ui.add_space(6.0);
                                        
                                        let preview = if cookie.len() > 60 {
                                            format!("{}...{}", &cookie[..30], &cookie[cookie.len()-20..])
                                        } else {
                                            cookie.clone()
                                        };
                                        
                                        ui.label(RichText::new(preview)
                                            .color(Colors::TEXT_MUTED)
                                            .size(10.0)
                                            .monospace());
                                        
                                        ui.add_space(8.0);
                                        
                                        ui.horizontal(|ui| {
                                            let use_btn = egui::Button::new(
                                                RichText::new("Use This")
                                                    .size(11.0)
                                                    .color(Colors::TEXT_PRIMARY)
                                            )
                                            .fill(Colors::ACCENT_GREEN.linear_multiply(0.2))
                                            .stroke(egui::Stroke::new(1.0, Colors::ACCENT_GREEN.linear_multiply(0.5)))
                                            .rounding(egui::Rounding::same(4.0));
                                            
                                            if ui.add_sized([80.0, 28.0], use_btn).clicked() {
                                                cookie_to_use = Some(cookie.clone());
                                            }
                                            
                                            ui.add_space(6.0);
                                            
                                            let copy_btn = egui::Button::new(
                                                RichText::new("📋 Copy")
                                                    .size(11.0)
                                                    .color(Colors::TEXT_SECONDARY)
                                            )
                                            .fill(Colors::BG_LIGHT)
                                            .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                                            .rounding(egui::Rounding::same(4.0));
                                            
                                            if ui.add_sized([70.0, 28.0], copy_btn).clicked() {
                                                ui.output_mut(|o| o.copied_text = cookie.clone());
                                                self.set_status("Cookie copied to clipboard", false);
                                            }
                                        });
                                    });
                                
                                ui.add_space(6.0);
                            }
                            
                            // Handle cookie selection after the loop
                            if let Some(cookie) = cookie_to_use {
                                self.import_cookie = cookie;
                                self.set_status("Cookie loaded into input field", false);
                            }
                        }
                        
                        ui.add_space(14.0);
                        
                        ui.separator();
                        
                        ui.add_space(10.0);
                        
                        ui.label(RichText::new("Or paste cookie manually:")
                            .color(Colors::TEXT_SECONDARY)
                            .size(11.0));
                        
                        ui.add_space(6.0);
                        
                        egui::Frame::none()
                            .fill(Colors::BG_DARK)
                            .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                            .rounding(egui::Rounding::same(6.0))
                            .inner_margin(egui::Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.add_sized(
                                    [ui.available_width(), 50.0],
                                    egui::TextEdit::multiline(&mut self.import_cookie)
                                        .hint_text("Paste your .ROBLOSECURITY cookie here...")
                                        .text_color(Colors::TEXT_PRIMARY)
                                        .font(egui::TextStyle::Monospace)
                                        .frame(false)
                                );
                            });
                        
                        ui.add_space(14.0);
                        
                        let import_btn = egui::Button::new(
                            RichText::new("Import & Add Account")
                                .size(14.0)
                                .color(if self.import_cookie.is_empty() { 
                                    Colors::TEXT_MUTED 
                                } else { 
                                    Color32::WHITE
                                })
                        )
                        .fill(if self.import_cookie.is_empty() {
                            Colors::BG_LIGHT
                        } else {
                            Colors::ACCENT_GREEN
                        })
                        .rounding(egui::Rounding::same(6.0));
                        
                        if ui.add_sized([ui.available_width(), 40.0], import_btn).clicked() && !self.import_cookie.is_empty() {
                            self.action = Action::ImportCookieAsNewAccount;
                        }
                    });
            }
        );
    }
    
    fn render_divider(&self, ui: &mut egui::Ui, width: f32) {
        ui.allocate_ui_with_layout(
            egui::vec2(width, 20.0),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.horizontal(|ui| {
                    let line_width = (width - 40.0) / 2.0;
                    
                    // Left line
                    ui.add_sized([line_width, 1.0], |ui: &mut egui::Ui| {
                        let rect = ui.available_rect_before_wrap();
                        ui.painter().line_segment(
                            [rect.left_center(), rect.right_center()],
                            egui::Stroke::new(1.0, Colors::BORDER_DARK)
                        );
                        ui.allocate_rect(rect, egui::Sense::hover())
                    });
                    
                    ui.add_space(8.0);
                    ui.label(RichText::new("or").color(Colors::TEXT_MUTED).size(12.0));
                    ui.add_space(8.0);
                    
                    // Right line
                    ui.add_sized([line_width, 1.0], |ui: &mut egui::Ui| {
                        let rect = ui.available_rect_before_wrap();
                        ui.painter().line_segment(
                            [rect.left_center(), rect.right_center()],
                            egui::Stroke::new(1.0, Colors::BORDER_DARK)
                        );
                        ui.allocate_rect(rect, egui::Sense::hover())
                    });
                });
            }
        );
    }
    
    fn render_security_notice(&self, ui: &mut egui::Ui, width: f32) {
        ui.allocate_ui_with_layout(
            egui::vec2(width, 0.0),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                egui::Frame::none()
                    .fill(Colors::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::symmetric(16.0, 12.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("🔒").size(14.0));
                            ui.add_space(10.0);
                            ui.label(RichText::new(
                                "Your credentials are stored locally and encrypted. Never share your cookies with anyone.")
                                .color(Colors::TEXT_MUTED)
                                .size(12.0));
                        });
                    });
            }
        );
    }
}
