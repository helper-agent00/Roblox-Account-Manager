// Account list UI

use eframe::egui::{self, RichText, Color32};
use crate::account::AccountStatus;
use crate::games::POPULAR_GAMES;
use crate::theme::{self, Colors};
use super::{Action, NexusApp, Tab};

impl NexusApp {
    pub fn render_accounts_tab(&mut self, ui: &mut egui::Ui) {
        let available_height = ui.available_height();
        
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(360.0, available_height),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                // Header with batch controls
                ui.horizontal(|ui| {
                    ui.label(RichText::new("YOUR ACCOUNTS").size(13.0).color(Colors::TEXT_MUTED).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Batch launch button
                        if !self.batch_selected.is_empty() {
                            if ui.add(egui::Button::new(
                                RichText::new(format!("🚀 Launch {} ", self.batch_selected.len()))
                                    .color(Colors::TEXT_PRIMARY).size(11.0)
                            ).fill(Colors::ACCENT_GREEN).rounding(egui::Rounding::same(4.0))).clicked() {
                                self.action = Action::BatchLaunch;
                            }
                            ui.add_space(4.0);
                        }
                        
                        // Fetch avatars button
                        if ui.add(egui::Button::new(
                            RichText::new("🖼").size(12.0)
                        ).fill(Colors::BG_LIGHT).rounding(egui::Rounding::same(4.0)).min_size(egui::vec2(24.0, 20.0)))
                        .on_hover_text("Fetch account avatars")
                        .clicked() {
                            self.action = Action::FetchAvatars;
                        }
                        
                        ui.add_space(4.0);
                        
                        // Refresh presence button
                        if ui.add(egui::Button::new(
                            RichText::new("🔄").size(12.0)
                        ).fill(Colors::BG_LIGHT).rounding(egui::Rounding::same(4.0)).min_size(egui::vec2(24.0, 20.0)))
                        .on_hover_text("Refresh online status")
                        .clicked() {
                            self.action = Action::RefreshPresence;
                        }
                        
                        ui.add_space(4.0);
                        ui.label(RichText::new(format!("{}", self.data.accounts.len())).size(13.0).color(Colors::ACCENT_BLUE));
                    });
                });
                
                ui.add_space(8.0);
                
                // Batch selection hint
                if self.data.accounts.len() > 1 {
                    ui.label(RichText::new("☐ Check boxes for batch launch").color(Colors::TEXT_MUTED).size(10.0));
                }
                
                ui.add_space(8.0);
                
                // Account cards with checkboxes
                egui::ScrollArea::vertical()
                    .id_salt("account_list_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    
                    let accounts: Vec<_> = self.data.accounts.iter().enumerate()
                        .map(|(i, a)| (i, a.username.clone(), a.display_name.clone(), a.status.clone(), 
                            a.last_login.clone(), a.robux, a.friends_count, a.is_premium, a.presence.clone(), 
                            a.group.clone(), a.user_id, a.avatar_url.clone(), a.collectibles_count))
                        .collect();
                    
                    if accounts.is_empty() {
                        egui::Frame::none()
                            .fill(Colors::BG_CARD)
                            .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(egui::Margin::same(24.0))
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(20.0);
                                    ui.label(RichText::new("No accounts yet").color(Colors::TEXT_MUTED).size(16.0));
                                    ui.add_space(8.0);
                                    ui.label(RichText::new("Go to 'Add Account' to get started").color(Colors::TEXT_MUTED).size(12.0));
                                    ui.add_space(20.0);
                                });
                            });
                    } else {
                        // Group accounts by group
                        let mut grouped: std::collections::BTreeMap<String, Vec<_>> = std::collections::BTreeMap::new();
                        for item in accounts {
                            let group_name = if item.9.is_empty() { "Ungrouped".to_string() } else { item.9.clone() };
                            grouped.entry(group_name).or_default().push(item);
                        }
                        
                        for (group_name, group_accounts) in grouped {
                            // Show group header if there are groups
                            if self.data.accounts.iter().any(|a| !a.group.is_empty()) {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(format!("📁 {}", group_name)).size(11.0).color(Colors::TEXT_MUTED).strong());
                                    ui.label(RichText::new(format!("({})", group_accounts.len())).size(10.0).color(Colors::TEXT_MUTED));
                                });
                                ui.add_space(4.0);
                            }
                        
                        for (idx, username, display_name, status, _last_login, robux, friends, is_premium, presence, _group, _user_id, avatar_url, collectibles) in group_accounts {
                            let is_selected = self.selected == Some(idx);
                            let is_batch_selected = self.batch_selected.contains(&idx);
                            
                            ui.horizontal(|ui| {
                                // Checkbox for batch selection
                                let mut batch_checked = is_batch_selected;
                                if ui.checkbox(&mut batch_checked, "").changed() {
                                    if batch_checked {
                                        self.batch_selected.insert(idx);
                                    } else {
                                        self.batch_selected.remove(&idx);
                                    }
                                }
                                
                                // Account card
                                let response = egui::Frame::none()
                                    .fill(if is_selected { Colors::BG_CARD_SELECTED } else if is_batch_selected { Colors::BG_CARD_SELECTED.linear_multiply(0.7) } else { Colors::BG_CARD })
                                    .stroke(egui::Stroke::new(
                                        if is_selected { 2.0 } else { 1.0 },
                                        if is_selected { Colors::BORDER_ACCENT } else if is_batch_selected { Colors::ACCENT_GREEN.linear_multiply(0.5) } else { Colors::BORDER_DARK }
                                    ))
                                    .rounding(egui::Rounding::same(8.0))
                                    .inner_margin(egui::Margin::same(10.0))
                                    .show(ui, |ui| {
                                        ui.set_width(290.0);
                                        ui.horizontal(|ui| {
                                            // Show avatar if URL is available, otherwise show status circle
                                            let mut showed_avatar = false;
                                            if let Some(ref url) = avatar_url {
                                                if !url.is_empty() {
                                                    // Use egui's image loading from URL
                                                    let size = egui::vec2(32.0, 32.0);
                                                    ui.add(egui::Image::from_uri(url).fit_to_exact_size(size).rounding(egui::Rounding::same(4.0)));
                                                    showed_avatar = true;
                                                }
                                            }
                                            
                                            if !showed_avatar {
                                                // Fallback: show presence/status indicator
                                                let (indicator_color, _indicator_text) = if let Some(ref p) = presence {
                                                    (p.presence_type.color(), p.presence_type.label())
                                                } else {
                                                    (status.color(), status.label())
                                                };
                                                theme::draw_status_circle(ui, indicator_color, 32.0);
                                            }
                                            ui.add_space(8.0);
                                            
                                            // Get presence info for display
                                            let (indicator_color, indicator_text) = if let Some(ref p) = presence {
                                                (p.presence_type.color(), p.presence_type.label())
                                            } else {
                                                (status.color(), status.label())
                                            };
                                            
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(RichText::new(&username).color(Colors::TEXT_PRIMARY).size(13.0).strong());
                                                    if is_premium == Some(true) {
                                                        ui.label(RichText::new("⭐").size(11.0));
                                                    }
                                                });
                                                
                                                if let Some(dn) = &display_name {
                                                    if dn != &username {
                                                        ui.label(RichText::new(dn).color(Colors::TEXT_SECONDARY).size(10.0));
                                                    }
                                                }
                                                
                                                // Presence row (if available and not offline)
                                                if let Some(ref p) = presence {
                                                    use crate::account::UserPresenceType;
                                                    if p.presence_type != UserPresenceType::Offline {
                                                        ui.horizontal(|ui| {
                                                            ui.label(RichText::new(indicator_text).color(indicator_color).size(10.0));
                                                            
                                                            // Show game name: prefer game_name (fetched), then last_location (API), then place_id
                                                            let game_display = p.game_name.as_ref()
                                                                .or(p.last_location.as_ref())
                                                                .map(|s| s.as_str())
                                                                .filter(|s| !s.is_empty());
                                                            
                                                            if let Some(game) = game_display {
                                                                ui.label(RichText::new("•").color(Colors::TEXT_MUTED).size(10.0));
                                                                ui.label(RichText::new(format!("🎮 {}", game)).color(Colors::ACCENT_BLUE).size(10.0));
                                                            } else if p.presence_type == UserPresenceType::InGame {
                                                                if let Some(pid) = p.place_id {
                                                                    ui.label(RichText::new("•").color(Colors::TEXT_MUTED).size(10.0));
                                                                    ui.label(RichText::new(format!("🎮 Game #{}", pid)).color(Colors::TEXT_MUTED).size(10.0));
                                                                }
                                                            }
                                                        });
                                                    } else {
                                                        // Show account status row for offline users
                                                        ui.horizontal(|ui| {
                                                            ui.label(RichText::new(status.label()).color(status.color()).size(10.0));
                                                            if let Some(r) = robux {
                                                                ui.label(RichText::new("•").color(Colors::TEXT_MUTED).size(10.0));
                                                                ui.label(RichText::new(format!("R${}", r)).color(Colors::ACCENT_GREEN).size(10.0));
                                                            }
                                                        });
                                                    }
                                                } else {
                                                    // Account stats row (no presence data)
                                                    ui.horizontal(|ui| {
                                                        ui.label(RichText::new(status.label()).color(status.color()).size(10.0));
                                                        
                                                        if let Some(r) = robux {
                                                            ui.label(RichText::new("•").color(Colors::TEXT_MUTED).size(10.0));
                                                            ui.label(RichText::new(format!("R${}", r)).color(Colors::ACCENT_GREEN).size(10.0));
                                                        }
                                                        
                                                        if let Some(f) = friends {
                                                            ui.label(RichText::new("•").color(Colors::TEXT_MUTED).size(10.0));
                                                            ui.label(RichText::new(format!("{} friends", f)).color(Colors::TEXT_MUTED).size(10.0));
                                                        }
                                                        
                                                        if let Some(c) = collectibles {
                                                            if c > 0 {
                                                                ui.label(RichText::new("•").color(Colors::TEXT_MUTED).size(10.0));
                                                                ui.label(RichText::new(format!("{} limiteds", c)).color(Colors::ACCENT_PURPLE).size(10.0));
                                                            }
                                                        }
                                                    });
                                                }
                                            });
                                        });
                                    });
                                
                                if response.response.interact(egui::Sense::click()).clicked() {
                                    self.action = Action::SelectAccount(idx);
                                }
                            });
                            
                            ui.add_space(4.0);
                        }
                        
                        ui.add_space(8.0);
                        }  // End of group loop
                    }
                });
            });  // End of allocate_ui_with_layout for left panel
            
            ui.add_space(16.0);
            ui.separator();
            ui.add_space(16.0);
            
            egui::ScrollArea::vertical()
                .id_salt("account_actions_scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                ui.vertical(|ui| {
                    if let Some(idx) = self.selected {
                        let account_info = self.data.accounts.get(idx).cloned();
                        
                        if let Some(account) = account_info {
                            // Account Info Card - Enhanced
                            egui::Frame::none()
                                .fill(Colors::BG_CARD)
                                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::same(16.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        theme::draw_status_circle(ui, account.status.color(), 20.0);
                                        ui.add_space(12.0);
                                        ui.vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new(&account.username).color(Colors::TEXT_PRIMARY).size(20.0).strong());
                                                if account.is_premium == Some(true) {
                                                    ui.label(RichText::new("⭐ Premium").color(Colors::ACCENT_YELLOW).size(12.0));
                                                }
                                            });
                                            ui.label(RichText::new(account.status.label()).color(account.status.color()).size(13.0));
                                        });
                                    });
                                    
                                    // Stats row
                                    if account.robux.is_some() || account.friends_count.is_some() {
                                        ui.add_space(12.0);
                                        ui.horizontal(|ui| {
                                            if let Some(robux) = account.robux {
                                                egui::Frame::none()
                                                    .fill(Colors::BG_LIGHT)
                                                    .rounding(egui::Rounding::same(4.0))
                                                    .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                                    .show(ui, |ui| {
                                                        ui.label(RichText::new(format!("💰 R${}", robux)).color(Colors::ACCENT_GREEN).size(12.0));
                                                    });
                                            }
                                            if let Some(friends) = account.friends_count {
                                                egui::Frame::none()
                                                    .fill(Colors::BG_LIGHT)
                                                    .rounding(egui::Rounding::same(4.0))
                                                    .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                                    .show(ui, |ui| {
                                                        ui.label(RichText::new(format!("👥 {} friends", friends)).color(Colors::ACCENT_BLUE).size(12.0));
                                                    });
                                            }
                                        });
                                    }
                                    
                                    // Fetch info button
                                    ui.add_space(8.0);
                                    if ui.add(egui::Button::new(
                                        RichText::new("📊 Fetch Account Info").color(Colors::TEXT_SECONDARY).size(11.0)
                                    ).fill(Colors::BG_LIGHT).rounding(egui::Rounding::same(4.0))).clicked() {
                                        self.action = Action::FetchAccountInfo(idx);
                                    }
                                    
                                    if let Some(last_fetch) = &account.last_info_fetch {
                                        ui.label(RichText::new(format!("Last updated: {}", last_fetch)).color(Colors::TEXT_MUTED).size(9.0));
                                    }
                                });
                            
                            ui.add_space(12.0);
                            
                            // Game Selection
                            egui::Frame::none()
                                .fill(Colors::BG_CARD)
                                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::same(16.0))
                                .show(ui, |ui| {
                                    ui.label(RichText::new("LAUNCH GAME").size(11.0).color(Colors::TEXT_MUTED));
                                    ui.add_space(8.0);
                                    
                                    let current_game = POPULAR_GAMES.iter()
                                        .find(|g| g.place_id == self.place_id)
                                        .map(|g| g.name)
                                        .unwrap_or(if self.place_id.is_empty() { "No game selected" } else { "Custom ID" });
                                    
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Game:").color(Colors::TEXT_SECONDARY));
                                        ui.label(RichText::new(current_game).color(Colors::ACCENT_BLUE));
                                    });
                                    
                                    ui.add_space(8.0);
                                    
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Place ID:").color(Colors::TEXT_SECONDARY));
                                        
                                        egui::Frame::none()
                                            .fill(Color32::WHITE)
                                            .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                                            .rounding(egui::Rounding::same(4.0))
                                            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                            .show(ui, |ui| {
                                                ui.add(egui::TextEdit::singleline(&mut self.place_id)
                                                    .desired_width(120.0)
                                                    .hint_text("Enter ID")
                                                    .text_color(Color32::from_rgb(20, 20, 30))
                                                    .frame(false));
                                            });
                                        
                                        if ui.add(theme::secondary_button("Browse")).clicked() {
                                            self.tab = Tab::Games;
                                        }
                                    });
                                });
                            
                            ui.add_space(12.0);
                            
                            // Action Buttons
                            egui::Frame::none()
                                .fill(Colors::BG_CARD)
                                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::same(16.0))
                                .show(ui, |ui| {
                                    ui.label(RichText::new("ACTIONS").size(11.0).color(Colors::TEXT_MUTED));
                                    ui.add_space(12.0);
                                    
                                    // Multi-instance toggle
                                    ui.horizontal(|ui| {
                                        let mut multi_enabled = self.data.multi_instance_enabled;
                                        if ui.checkbox(&mut multi_enabled, "").changed() {
                                            self.action = Action::ToggleMultiInstance;
                                        }
                                        ui.label(RichText::new("Multi-Instance Mode").color(Colors::TEXT_SECONDARY).size(12.0));
                                        if multi_enabled {
                                            ui.label(RichText::new("(ON)").color(Colors::ACCENT_GREEN).size(10.0));
                                        }
                                    });
                                    ui.label(RichText::new("Allow multiple Roblox windows").color(Colors::TEXT_MUTED).size(10.0));
                                    
                                    ui.add_space(12.0);
                                    
                                    // Launch button
                                    let can_launch = account.status == AccountStatus::Valid && account.cookie.is_some();
                                    ui.add_enabled_ui(can_launch, |ui| {
                                        if ui.add_sized([ui.available_width(), 44.0], theme::primary_button("▶ LAUNCH ROBLOX")).clicked() {
                                            self.action = Action::LaunchAccount(idx);
                                        }
                                    });
                                    
                                    if !can_launch {
                                        ui.label(RichText::new("Verify account to enable launch").color(Colors::TEXT_MUTED).size(10.0));
                                    }
                                    
                                    ui.add_space(12.0);
                                    
                                    // Verify button
                                    if ui.add_sized([ui.available_width(), 36.0], theme::secondary_button("🔄 Verify Account")).clicked() {
                                        self.action = Action::VerifyAccount(idx);
                                    }
                                    
                                    ui.add_space(8.0);
                                    
                                    // Manage Cookie button - always visible for valid accounts
                                    if account.cookie.is_some() {
                                        if ui.add_sized([ui.available_width(), 36.0], 
                                            egui::Button::new(RichText::new("🍪 Manage Cookie").color(Colors::TEXT_SECONDARY).size(12.0))
                                                .fill(Colors::BG_LIGHT)
                                                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                                                .rounding(egui::Rounding::same(6.0))
                                        ).clicked() {
                                            self.action = Action::ShowCookieModal(idx);
                                        }
                                        
                                        ui.add_space(8.0);
                                        
                                        // Follow User button
                                        if ui.add_sized([ui.available_width(), 36.0], 
                                            egui::Button::new(RichText::new("👥 Follow User").color(Colors::ACCENT_BLUE).size(12.0))
                                                .fill(Colors::ACCENT_BLUE.linear_multiply(0.15))
                                                .stroke(egui::Stroke::new(1.0, Colors::ACCENT_BLUE.linear_multiply(0.5)))
                                                .rounding(egui::Rounding::same(6.0))
                                        ).on_hover_text("Join a friend's game")
                                        .clicked() {
                                            self.follow_user_account_idx = Some(idx);
                                            self.follow_user_show = true;
                                            self.follow_user_target.clear();
                                        }
                                    }
                                    
                                    // Import cookie button (if needed)
                                    if account.status == AccountStatus::Requires2FA || account.status == AccountStatus::Invalid || account.status == AccountStatus::NotVerified {
                                        ui.add_space(8.0);
                                        if ui.add_sized([ui.available_width(), 36.0], theme::secondary_button("🍪 Import Cookie")).clicked() {
                                            self.tab = Tab::ImportCookie;
                                            self.import_cookie.clear();
                                        }
                                    }
                                });
                            
                            ui.add_space(12.0);
                            
                            // Delete Section
                            egui::Frame::none()
                                .fill(Colors::ACCENT_RED.linear_multiply(0.1))
                                .stroke(egui::Stroke::new(1.0, Colors::ACCENT_RED.linear_multiply(0.3)))
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::same(16.0))
                                .show(ui, |ui| {
                                    if self.delete_confirm == Some(idx) {
                                        ui.label(RichText::new("Are you sure?").color(Colors::ACCENT_RED).size(14.0).strong());
                                        ui.add_space(8.0);
                                        ui.horizontal(|ui| {
                                            if ui.add_sized([100.0, 32.0], theme::danger_button("Yes, Delete")).clicked() {
                                                self.action = Action::DeleteAccount(idx);
                                                self.delete_confirm = None;
                                            }
                                            if ui.add_sized([80.0, 32.0], theme::secondary_button("Cancel")).clicked() {
                                                self.delete_confirm = None;
                                            }
                                        });
                                    } else {
                                        if ui.add_sized([ui.available_width(), 34.0], theme::danger_button("🗑 Delete Account")).clicked() {
                                            self.delete_confirm = Some(idx);
                                        }
                                    }
                                });
                        }
                    } else {
                        // No account selected
                        egui::Frame::none()
                            .fill(Colors::BG_CARD)
                            .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(egui::Margin::same(32.0))
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(40.0);
                                    ui.label(RichText::new("Select an account").color(Colors::TEXT_MUTED).size(18.0));
                                    ui.add_space(8.0);
                                    ui.label(RichText::new("Choose from the list on the left").color(Colors::TEXT_MUTED).size(13.0));
                                    ui.add_space(40.0);
                                });
                            });
                    }
                });
            });
        });
    }
}
