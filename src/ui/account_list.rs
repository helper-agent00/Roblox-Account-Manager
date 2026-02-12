use eframe::egui::{self, RichText};
use crate::account::AccountStatus;
use crate::games::POPULAR_GAMES;
use crate::theme::{self, Colors};
use super::{Action, AccountSort, NexusApp, Tab};

impl NexusApp {
    pub fn render_accounts_tab(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        let available_height = ui.available_height();
        let list_width = (available_width * 0.45).min(420.0).max(340.0);
        
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            
            ui.allocate_ui_with_layout(
                egui::vec2(list_width, available_height),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("ACCOUNTS").size(11.0).color(Colors::TEXT_MUTED).strong());
                    ui.add_space(4.0);
                    theme::label_badge(ui,
                        &format!("{}", self.data.accounts.len()),
                        Colors::ACCENT_BLUE,
                    );
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if !self.batch_selected.is_empty() {
                            if ui.add(theme::success_button(
                                &format!("▶ Launch {}", self.batch_selected.len())
                            ).min_size(egui::vec2(0.0, 24.0))).clicked() {
                                self.action = Action::BatchLaunch;
                            }
                            ui.add_space(4.0);
                        }
                        
                        if ui.add(theme::icon_button("🖼"))
                            .on_hover_text("Fetch avatars")
                            .clicked() {
                            self.action = Action::FetchAvatars;
                        }
                        
                        if ui.add(theme::icon_button("🔄"))
                            .on_hover_text("Refresh presence")
                            .clicked() {
                            self.action = Action::RefreshPresence;
                        }
                        
                        let sort_label = format!("↕ {}", self.account_sort.label());
                        if ui.add(egui::Button::new(
                            RichText::new(&sort_label).size(10.0).color(Colors::TEXT_MUTED)
                        ).fill(Colors::BG_LIGHT).rounding(egui::Rounding::same(4.0))).clicked() {
                            self.account_sort = self.account_sort.next();
                        }
                    });
                });
                
                ui.add_space(6.0);
                
                theme::input_frame().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("🔍").size(12.0).color(Colors::TEXT_MUTED));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.account_search)
                                .desired_width(ui.available_width())
                                .hint_text(RichText::new("Search accounts...").color(Colors::TEXT_MUTED))
                                .text_color(Colors::TEXT_PRIMARY)
                                .frame(false)
                        );
                    });
                });
                
                ui.add_space(6.0);
                
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
                            .stroke(egui::Stroke::new(0.5, Colors::BORDER_SUBTLE))
                            .rounding(egui::Rounding::same(10.0))
                            .inner_margin(egui::Margin::same(32.0))
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(16.0);
                                    ui.label(RichText::new("📋").size(32.0));
                                    ui.add_space(8.0);
                                    ui.label(RichText::new("No accounts yet").color(Colors::TEXT_SECONDARY).size(16.0));
                                    ui.add_space(4.0);
                                    ui.label(RichText::new("Click 'Add Account' to get started").color(Colors::TEXT_MUTED).size(12.0));
                                    ui.add_space(16.0);
                                });
                            });
                    } else {
                        let search_lower = self.account_search.to_lowercase();
                        let filtered: Vec<_> = accounts.into_iter()
                            .filter(|(_, username, display_name, _, _, _, _, _, _, group, _, _, _)| {
                                if search_lower.is_empty() { return true; }
                                username.to_lowercase().contains(&search_lower)
                                    || display_name.as_ref().map(|d| d.to_lowercase().contains(&search_lower)).unwrap_or(false)
                                    || group.to_lowercase().contains(&search_lower)
                            })
                            .collect();
                        
                        let mut sorted = filtered;
                        match self.account_sort {
                            AccountSort::Default => {},
                            AccountSort::NameAsc => sorted.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase())),
                            AccountSort::NameDesc => sorted.sort_by(|a, b| b.1.to_lowercase().cmp(&a.1.to_lowercase())),
                            AccountSort::StatusFirst => sorted.sort_by(|a, b| {
                                let status_order = |s: &AccountStatus| match s {
                                    AccountStatus::Valid => 0,
                                    AccountStatus::NotVerified => 1,
                                    AccountStatus::Requires2FA => 2,
                                    AccountStatus::Invalid => 3,
                                };
                                status_order(&a.3).cmp(&status_order(&b.3))
                            }),
                            AccountSort::RobuxHigh => sorted.sort_by(|a, b| b.5.unwrap_or(0).cmp(&a.5.unwrap_or(0))),
                            AccountSort::GroupAlpha => sorted.sort_by(|a, b| a.9.cmp(&b.9)),
                        }
                        
                        let use_groups = self.data.accounts.iter().any(|a| !a.group.is_empty());
                        let mut grouped: std::collections::BTreeMap<String, Vec<_>> = std::collections::BTreeMap::new();
                        
                        if use_groups && self.account_sort == AccountSort::GroupAlpha {
                            for item in sorted {
                                let group_name = if item.9.is_empty() { "Ungrouped".to_string() } else { item.9.clone() };
                                grouped.entry(group_name).or_default().push(item);
                            }
                        } else {
                            grouped.insert(String::new(), sorted);
                        }
                        
                        for (group_name, group_accounts) in grouped {
                            if !group_name.is_empty() {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("▸").size(10.0).color(Colors::TEXT_MUTED));
                                    ui.label(RichText::new(&group_name).size(10.5).color(Colors::TEXT_MUTED).strong());
                                    ui.label(RichText::new(format!("({})", group_accounts.len())).size(9.5).color(Colors::TEXT_MUTED));
                                });
                                ui.add_space(3.0);
                            }
                        
                        for (idx, username, display_name, status, _last_login, robux, friends, is_premium, presence, _group, _user_id, avatar_url, collectibles) in group_accounts {
                            let is_selected = self.selected == Some(idx);
                            let is_batch_selected = self.batch_selected.contains(&idx);
                            
                            ui.horizontal(|ui| {
                                let mut batch_checked = is_batch_selected;
                                if ui.checkbox(&mut batch_checked, "").changed() {
                                    if batch_checked {
                                        self.batch_selected.insert(idx);
                                    } else {
                                        self.batch_selected.remove(&idx);
                                    }
                                }
                                
                                let card_fill = if is_selected {
                                    Colors::BG_CARD_SELECTED
                                } else if is_batch_selected {
                                    Colors::BG_CARD_SELECTED.linear_multiply(0.6)
                                } else {
                                    Colors::BG_CARD
                                };
                                let card_stroke = if is_selected {
                                    egui::Stroke::new(1.5, Colors::BORDER_ACCENT)
                                } else if is_batch_selected {
                                    egui::Stroke::new(1.0, Colors::ACCENT_GREEN.linear_multiply(0.4))
                                } else {
                                    egui::Stroke::new(0.5, Colors::BORDER_SUBTLE)
                                };
                                
                                let response = egui::Frame::none()
                                    .fill(card_fill)
                                    .stroke(card_stroke)
                                    .rounding(egui::Rounding::same(8.0))
                                    .inner_margin(egui::Margin::same(10.0))
                                    .show(ui, |ui| {
                                        ui.set_width(ui.available_width() - 16.0);
                                        ui.horizontal(|ui| {
                                            let mut showed_avatar = false;
                                            if let Some(ref url) = avatar_url {
                                                if !url.is_empty() {
                                                    let size = egui::vec2(36.0, 36.0);
                                                    ui.add(egui::Image::from_uri(url)
                                                        .fit_to_exact_size(size)
                                                        .rounding(egui::Rounding::same(6.0)));
                                                    showed_avatar = true;
                                                }
                                            }
                                            if !showed_avatar {
                                                let (indicator_color, _) = if let Some(ref p) = presence {
                                                    (p.presence_type.color(), p.presence_type.label())
                                                } else {
                                                    (status.color(), status.label())
                                                };
                                                theme::draw_status_circle(ui, indicator_color, 36.0);
                                            }
                                            ui.add_space(8.0);
                                            
                                            let (indicator_color, indicator_text) = if let Some(ref p) = presence {
                                                (p.presence_type.color(), p.presence_type.label())
                                            } else {
                                                (status.color(), status.label())
                                            };
                                            
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(RichText::new(&username).color(Colors::TEXT_PRIMARY).size(13.0).strong());
                                                    if is_premium == Some(true) {
                                                        ui.label(RichText::new("⭐").size(10.0));
                                                    }
                                                });
                                                
                                                if let Some(dn) = &display_name {
                                                    if dn != &username {
                                                        ui.label(RichText::new(dn).color(Colors::TEXT_MUTED).size(10.0));
                                                    }
                                                }
                                                
                                                ui.horizontal(|ui| {
                                                    if let Some(ref p) = presence {
                                                        use crate::account::UserPresenceType;
                                                        if p.presence_type != UserPresenceType::Offline {
                                                            theme::draw_status_circle(ui, indicator_color, 6.0);
                                                            ui.label(RichText::new(indicator_text).color(indicator_color).size(10.0));
                                                            
                                                            let game_display = p.game_name.as_ref()
                                                                .or(p.last_location.as_ref())
                                                                .map(|s| s.as_str())
                                                                .filter(|s| !s.is_empty());
                                                            
                                                            if let Some(game) = game_display {
                                                                ui.label(RichText::new("·").color(Colors::TEXT_MUTED).size(10.0));
                                                                ui.label(RichText::new(game).color(Colors::ACCENT_BLUE).size(10.0));
                                                            }
                                                        } else {
                                                            Self::render_account_stats_inline(ui, &status, robux, friends, collectibles);
                                                        }
                                                    } else {
                                                        Self::render_account_stats_inline(ui, &status, robux, friends, collectibles);
                                                    }
                                                });
                                            });
                                        });
                                    });
                                
                                if response.response.interact(egui::Sense::click()).clicked() {
                                    self.action = Action::SelectAccount(idx);
                                }
                            });
                            
                            ui.add_space(3.0);
                        }
                        
                        ui.add_space(6.0);
                        }
                    }
                });
            });
            
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);
            
            egui::ScrollArea::vertical()
                .id_salt("account_actions_scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.vertical(|ui| {
                    if let Some(idx) = self.selected {
                        let account_info = self.data.accounts.get(idx).cloned();
                        
                        if let Some(account) = account_info {
                            theme::section_frame().show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    // Avatar or status indicator
                                    if let Some(ref url) = account.avatar_url {
                                        if !url.is_empty() {
                                            ui.add(egui::Image::from_uri(url)
                                                .fit_to_exact_size(egui::vec2(48.0, 48.0))
                                                .rounding(egui::Rounding::same(8.0)));
                                        } else {
                                            theme::draw_status_circle(ui, account.status.color(), 20.0);
                                        }
                                    } else {
                                        theme::draw_status_circle(ui, account.status.color(), 20.0);
                                    }
                                    ui.add_space(12.0);
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(&account.username).color(Colors::TEXT_PRIMARY).size(20.0).strong());
                                            if account.is_premium == Some(true) {
                                                theme::label_badge(ui, "⭐ PREMIUM", Colors::ACCENT_YELLOW);
                                            }
                                        });
                                        ui.horizontal(|ui| {
                                            theme::draw_status_circle(ui, account.status.color(), 8.0);
                                            ui.label(RichText::new(account.status.label()).color(account.status.color()).size(12.0));
                                            if let Some(ref dn) = account.display_name {
                                                if dn != &account.username {
                                                    ui.label(RichText::new("·").color(Colors::TEXT_MUTED));
                                                    ui.label(RichText::new(dn).color(Colors::TEXT_SECONDARY).size(12.0));
                                                }
                                            }
                                        });
                                    });
                                });
                                
                                if account.robux.is_some() || account.friends_count.is_some() {
                                    ui.add_space(10.0);
                                    ui.horizontal(|ui| {
                                        if let Some(robux) = account.robux {
                                            theme::stat_chip(ui, &format!("💰 R${}", robux), Colors::ACCENT_GREEN);
                                        }
                                        if let Some(friends) = account.friends_count {
                                            theme::stat_chip(ui, &format!("👥 {}", friends), Colors::ACCENT_BLUE);
                                        }
                                        if let Some(c) = account.collectibles_count {
                                            if c > 0 {
                                                theme::stat_chip(ui, &format!("💎 {}", c), Colors::ACCENT_PURPLE);
                                            }
                                        }
                                    });
                                }
                                
                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    if ui.add(egui::Button::new(
                                        RichText::new("📊 Fetch Info").color(Colors::TEXT_SECONDARY).size(11.0)
                                    ).fill(Colors::BG_LIGHT).rounding(egui::Rounding::same(4.0))).clicked() {
                                        self.action = Action::FetchAccountInfo(idx);
                                    }
                                    if let Some(last_fetch) = &account.last_info_fetch {
                                        ui.label(RichText::new(format!("Updated: {}", last_fetch)).color(Colors::TEXT_MUTED).size(9.0));
                                    }
                                });
                            });
                            
                            ui.add_space(10.0);
                            
                            theme::section_frame().show(ui, |ui| {
                                ui.label(RichText::new("LAUNCH GAME").size(10.0).color(Colors::TEXT_MUTED).strong());
                                ui.add_space(6.0);
                                
                                let current_game = POPULAR_GAMES.iter()
                                    .find(|g| g.place_id == self.place_id)
                                    .map(|g| g.name)
                                    .unwrap_or(if self.place_id.is_empty() { "No game selected" } else { "Custom ID" });
                                
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Game:").color(Colors::TEXT_SECONDARY).size(12.0));
                                    ui.label(RichText::new(current_game).color(Colors::ACCENT_BLUE).size(12.0));
                                });
                                
                                ui.add_space(6.0);
                                
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Place ID:").color(Colors::TEXT_SECONDARY).size(12.0));
                                    theme::input_frame().show(ui, |ui| {
                                        ui.add(egui::TextEdit::singleline(&mut self.place_id)
                                            .desired_width(120.0)
                                            .hint_text(RichText::new("Enter ID").color(Colors::TEXT_MUTED))
                                            .text_color(Colors::TEXT_PRIMARY)
                                            .frame(false));
                                    });
                                    if ui.add(theme::secondary_button("Browse")).clicked() {
                                        self.tab = Tab::Games;
                                    }
                                });
                            });
                            
                            ui.add_space(10.0);
                            
                            theme::section_frame().show(ui, |ui| {
                                ui.label(RichText::new("ACTIONS").size(10.0).color(Colors::TEXT_MUTED).strong());
                                ui.add_space(8.0);
                                
                                ui.horizontal(|ui| {
                                    let mut multi_enabled = self.data.multi_instance_enabled;
                                    if ui.checkbox(&mut multi_enabled, "").changed() {
                                        self.action = Action::ToggleMultiInstance;
                                    }
                                    ui.label(RichText::new("Multi-Instance").color(Colors::TEXT_SECONDARY).size(12.0));
                                    if multi_enabled {
                                        theme::label_badge(ui, "ON", Colors::ACCENT_GREEN);
                                    }
                                });
                                
                                ui.add_space(10.0);
                                
                                let can_launch = account.status == AccountStatus::Valid && account.cookie.is_some();
                                ui.add_enabled_ui(can_launch, |ui| {
                                    if ui.add_sized([ui.available_width(), 40.0], theme::primary_button("▶  LAUNCH ROBLOX")).clicked() {
                                        self.action = Action::LaunchAccount(idx);
                                    }
                                });
                                
                                if !can_launch {
                                    ui.label(RichText::new("Verify account to enable launch").color(Colors::TEXT_MUTED).size(10.0));
                                }
                                
                                ui.add_space(8.0);
                                
                                if ui.add_sized([ui.available_width(), 32.0], theme::secondary_button("🔄  Verify Account")).clicked() {
                                    self.action = Action::VerifyAccount(idx);
                                }
                                
                                ui.add_space(6.0);
                                
                                if account.cookie.is_some() {
                                    ui.horizontal(|ui| {
                                        if ui.add(egui::Button::new(
                                            RichText::new("🍪 Cookie").color(Colors::TEXT_SECONDARY).size(11.0)
                                        ).fill(Colors::BG_LIGHT).stroke(egui::Stroke::new(0.5, Colors::BORDER_DARK))
                                            .rounding(egui::Rounding::same(5.0))).clicked() {
                                            self.action = Action::ShowCookieModal(idx);
                                        }
                                        
                                        if ui.add(egui::Button::new(
                                            RichText::new("👥 Follow User").color(Colors::ACCENT_BLUE).size(11.0)
                                        ).fill(Colors::ACCENT_BLUE.linear_multiply(0.1))
                                            .stroke(egui::Stroke::new(0.5, Colors::ACCENT_BLUE.linear_multiply(0.3)))
                                            .rounding(egui::Rounding::same(5.0)))
                                        .on_hover_text("Join a friend's game")
                                        .clicked() {
                                            self.follow_user_account_idx = Some(idx);
                                            self.follow_user_show = true;
                                            self.follow_user_target.clear();
                                        }
                                    });
                                }
                                
                                if account.status == AccountStatus::Requires2FA || account.status == AccountStatus::Invalid || account.status == AccountStatus::NotVerified {
                                    ui.add_space(6.0);
                                    if ui.add_sized([ui.available_width(), 32.0], theme::secondary_button("🍪 Import Cookie")).clicked() {
                                        self.tab = Tab::ImportCookie;
                                        self.import_cookie.clear();
                                    }
                                }
                            });
                            
                            ui.add_space(10.0);
                            
                            egui::Frame::none()
                                .fill(Colors::ACCENT_RED.linear_multiply(0.06))
                                .stroke(egui::Stroke::new(0.5, Colors::ACCENT_RED.linear_multiply(0.2)))
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::same(12.0))
                                .show(ui, |ui| {
                                    if self.delete_confirm == Some(idx) {
                                        ui.label(RichText::new("Confirm deletion?").color(Colors::ACCENT_RED).size(13.0).strong());
                                        ui.add_space(6.0);
                                        ui.horizontal(|ui| {
                                            if ui.add_sized([90.0, 30.0], theme::danger_button("Delete")).clicked() {
                                                self.action = Action::DeleteAccount(idx);
                                                self.delete_confirm = None;
                                            }
                                            if ui.add_sized([70.0, 30.0], theme::secondary_button("Cancel")).clicked() {
                                                self.delete_confirm = None;
                                            }
                                        });
                                    } else {
                                        if ui.add_sized([ui.available_width(), 30.0], theme::danger_button("🗑  Delete Account")).clicked() {
                                            self.delete_confirm = Some(idx);
                                        }
                                    }
                                });
                        }
                    } else {
                        theme::section_frame().show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(60.0);
                                ui.label(RichText::new("←").size(28.0).color(Colors::TEXT_MUTED));
                                ui.add_space(8.0);
                                ui.label(RichText::new("Select an account").color(Colors::TEXT_SECONDARY).size(16.0));
                                ui.add_space(4.0);
                                ui.label(RichText::new("Choose from the list on the left").color(Colors::TEXT_MUTED).size(12.0));
                                ui.add_space(60.0);
                            });
                        });
                    }
                });
            });
        });
    }
    
    fn render_account_stats_inline(ui: &mut egui::Ui, status: &AccountStatus, robux: Option<i64>, friends: Option<u32>, collectibles: Option<u32>) {
        theme::draw_status_circle(ui, status.color(), 6.0);
        ui.label(RichText::new(status.label()).color(status.color()).size(10.0));
        
        if let Some(r) = robux {
            ui.label(RichText::new("·").color(Colors::TEXT_MUTED).size(10.0));
            ui.label(RichText::new(format!("R${}", r)).color(Colors::ACCENT_GREEN).size(10.0));
        }
        if let Some(f) = friends {
            ui.label(RichText::new("·").color(Colors::TEXT_MUTED).size(10.0));
            ui.label(RichText::new(format!("{} friends", f)).color(Colors::TEXT_MUTED).size(10.0));
        }
        if let Some(c) = collectibles {
            if c > 0 {
                ui.label(RichText::new("·").color(Colors::TEXT_MUTED).size(10.0));
                ui.label(RichText::new(format!("{} limiteds", c)).color(Colors::ACCENT_PURPLE).size(10.0));
            }
        }
    }
}
