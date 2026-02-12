use eframe::egui::{self, RichText};
use crate::api::server_browser::SortColumn;
use crate::theme::Colors;
use super::{Action, NexusApp, Tab};

impl NexusApp {
    pub fn render_servers_tab(&mut self, ui: &mut egui::Ui) {
        let game_name = self.server_browser.current_game_name.clone()
            .or_else(|| if !self.selected_game_name.is_empty() { Some(self.selected_game_name.clone()) } else { None });
        
        ui.horizontal(|ui| {
            ui.label(RichText::new("🌐 SERVER BROWSER").size(13.0).color(Colors::TEXT_MUTED).strong());
            
            ui.add_space(20.0);
            
            if let Some(ref name) = game_name {
                ui.label(RichText::new("🎮").size(13.0));
                ui.label(RichText::new(name).color(Colors::ACCENT_BLUE).strong());
                ui.label(RichText::new(format!("({})", self.place_id)).color(Colors::TEXT_MUTED).size(11.0));
            } else if !self.place_id.is_empty() {
                ui.label(RichText::new("Place ID:").color(Colors::TEXT_SECONDARY));
                ui.label(RichText::new(&self.place_id).color(Colors::ACCENT_BLUE).strong());
            } else {
                ui.label(RichText::new("⚠ Select a game first").color(Colors::ACCENT_YELLOW));
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add(egui::Button::new("🎮 Games").fill(Colors::BG_LIGHT)).clicked() {
                    self.tab = Tab::Games;
                }
                
                let refresh_text = if self.server_browser.show_vip_servers { "🔄 Refresh VIP" } else { "🔄 Refresh" };
                if ui.add(egui::Button::new(refresh_text).fill(Colors::ACCENT_BLUE)).clicked() {
                    if self.server_browser.show_vip_servers {
                        self.action = Action::FetchVipServers;
                    } else {
                        self.action = Action::FetchServers;
                    }
                }
            });
        });
        
        ui.add_space(12.0);
        
        egui::Frame::none()
            .fill(Colors::BG_CARD)
            .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
            .rounding(egui::Rounding::same(8.0))
            .inner_margin(egui::Margin::same(12.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let public_active = !self.server_browser.show_vip_servers;
                    let public_btn = egui::Button::new(
                        RichText::new("🌐 Public").color(if public_active { Colors::TEXT_PRIMARY } else { Colors::TEXT_MUTED })
                    )
                    .fill(if public_active { Colors::ACCENT_BLUE } else { Colors::BG_LIGHT })
                    .rounding(egui::Rounding::same(6.0));
                    
                    if ui.add(public_btn).clicked() {
                        self.server_browser.show_vip_servers = false;
                        self.server_browser.selected_server = None;
                    }
                    
                    ui.add_space(4.0);
                    
                    let vip_active = self.server_browser.show_vip_servers;
                    let vip_btn = egui::Button::new(
                        RichText::new("⭐ VIP").color(if vip_active { Colors::TEXT_PRIMARY } else { Colors::TEXT_MUTED })
                    )
                    .fill(if vip_active { egui::Color32::from_rgb(255, 193, 7) } else { Colors::BG_LIGHT })
                    .rounding(egui::Rounding::same(6.0));
                    
                    if ui.add(vip_btn).clicked() {
                        self.server_browser.show_vip_servers = true;
                        self.server_browser.selected_server = None;
                        if self.server_browser.vip_servers.is_empty() && !self.place_id.is_empty() {
                            self.action = Action::FetchVipServers;
                        }
                    }
                    
                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(8.0);
                    
                    ui.label(RichText::new("🔒").size(14.0));
                    ui.add_space(4.0);
                    
                    let input_width = (ui.available_width() - 80.0).max(120.0);
                    egui::Frame::none()
                        .fill(Colors::BG_DARK)
                        .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                        .rounding(egui::Rounding::same(4.0))
                        .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                        .show(ui, |ui| {
                            ui.add_sized(
                                [input_width, 20.0],
                                egui::TextEdit::singleline(&mut self.server_browser.private_server_input)
                                    .hint_text("Private server link...")
                                    .text_color(Colors::TEXT_PRIMARY)
                                    .frame(false)
                            );
                        });
                    
                    ui.add_space(4.0);
                    
                    if let Some(idx) = self.selected {
                        let enabled = !self.server_browser.private_server_input.trim().is_empty();
                        ui.add_enabled_ui(enabled, |ui| {
                            if ui.add(egui::Button::new("Join").fill(Colors::ACCENT_GREEN).min_size(egui::vec2(50.0, 24.0))).clicked() {
                                self.action = Action::JoinPrivateServerLink(idx);
                            }
                        });
                    }
                });
            });
        
        ui.add_space(12.0);
        
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            if self.server_browser.loading {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(RichText::new("Loading servers...").color(Colors::TEXT_SECONDARY));
                });
                return;
            }
            
            if let Some(ref error) = self.server_browser.error {
                egui::Frame::none()
                    .fill(Colors::ACCENT_RED.linear_multiply(0.1))
                    .stroke(egui::Stroke::new(1.0, Colors::ACCENT_RED))
                    .rounding(egui::Rounding::same(6.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.label(RichText::new(format!("❌ {}", error)).color(Colors::ACCENT_RED));
                    });
                return;
            }
            
            let servers = self.server_browser.active_servers();
            
            if servers.is_empty() {
                egui::Frame::none()
                    .fill(Colors::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(20.0))
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            let msg = if self.server_browser.show_vip_servers {
                                "No VIP servers found"
                            } else {
                                "No servers loaded"
                            };
                            ui.label(RichText::new(msg).size(16.0).color(Colors::TEXT_MUTED));
                            ui.add_space(8.0);
                            if !self.place_id.is_empty() {
                                let hint = if self.server_browser.show_vip_servers {
                                    "Select account & click Refresh VIP"
                                } else {
                                    "Click 'Refresh' to load servers"
                                };
                                ui.label(RichText::new(hint).color(Colors::TEXT_SECONDARY));
                            } else {
                                ui.label(RichText::new("Select a game from the Games tab first").color(Colors::TEXT_SECONDARY));
                            }
                        });
                    });
                return;
            }
            
            let total = servers.len();
            let total_players: u32 = servers.iter().map(|s| s.playing).sum();
            let server_type_icon = if self.server_browser.show_vip_servers { "⭐" } else { "🌐" };
            
            egui::Frame::none()
                .fill(Colors::ACCENT_BLUE.linear_multiply(0.1))
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_ACCENT))
                .rounding(egui::Rounding::same(6.0))
                .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{} {} servers", server_type_icon, total)).color(Colors::ACCENT_BLUE));
                        ui.add_space(20.0);
                        ui.label(RichText::new(format!("👥 {} players", total_players)).color(Colors::TEXT_SECONDARY));
                        
                        if let Some(idx) = self.selected {
                            ui.add_space(20.0);
                            
                            if self.server_browser.selected_server.is_some() {
                                if ui.add(egui::Button::new("🎯 Join Selected").fill(Colors::ACCENT_GREEN)).clicked() {
                                    self.action = Action::JoinSelectedServer(idx);
                                }
                                ui.add_space(8.0);
                            }
                            
                            if !self.server_browser.show_vip_servers {
                                if ui.add(egui::Button::new("🎲 Random").fill(Colors::ACCENT_BLUE)).clicked() {
                                    self.action = Action::JoinRandomServer(idx);
                                }
                            }
                        }
                    });
                });
            
            ui.add_space(12.0);
            
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                
                let id_label = if self.server_browser.show_vip_servers { "Server Name" } else { "Server ID" };
                ui.label(RichText::new(id_label).size(11.0).color(Colors::TEXT_MUTED).strong());
                ui.add_space(100.0);
                
                let players_arrow = if self.server_browser.sort_column == SortColumn::Players {
                    self.server_browser.sort_direction.arrow()
                } else { "" };
                if ui.add(egui::Button::new(RichText::new(format!("Players {}", players_arrow)).size(11.0).color(Colors::TEXT_MUTED).strong())
                    .fill(egui::Color32::TRANSPARENT)
                    .stroke(egui::Stroke::NONE))
                    .clicked() 
                {
                    self.server_browser.toggle_sort(SortColumn::Players);
                }
                
                ui.add_space(60.0);
                
                let fill_arrow = if self.server_browser.sort_column == SortColumn::Fill {
                    self.server_browser.sort_direction.arrow()
                } else { "" };
                if ui.add(egui::Button::new(RichText::new(format!("Fill {}", fill_arrow)).size(11.0).color(Colors::TEXT_MUTED).strong())
                    .fill(egui::Color32::TRANSPARENT)
                    .stroke(egui::Stroke::NONE))
                    .clicked() 
                {
                    self.server_browser.toggle_sort(SortColumn::Fill);
                }
            });
            
            ui.add_space(8.0);
            
            let sorted_indices = self.server_browser.get_sorted_indices();
            let servers_data: Vec<_> = {
                let servers = self.server_browser.active_servers();
                sorted_indices.iter().map(|&i| {
                    let s = &servers[i];
                    (i, s.display_name(), s.playing, s.max_players, s.ping, s.fill_percent(), s.is_full(), s.is_vip(), s.access_code.clone(), s.id.clone())
                }).collect()
            };
            
            for (idx, display_name, playing, max_players, ping, fill_pct, is_full, is_vip, access_code, server_id) in servers_data {
                let is_selected = self.server_browser.selected_server == Some(idx);
                
                let frame_fill = if is_selected { 
                    Colors::BG_CARD_SELECTED 
                } else if is_vip {
                    egui::Color32::from_rgb(255, 193, 7).linear_multiply(0.05)
                } else { 
                    Colors::BG_CARD 
                };
                
                egui::Frame::none()
                    .fill(frame_fill)
                    .stroke(egui::Stroke::new(
                        if is_selected { 2.0 } else { 1.0 },
                        if is_selected { Colors::BORDER_ACCENT } 
                        else if is_vip { egui::Color32::from_rgb(255, 193, 7).linear_multiply(0.3) }
                        else { Colors::BORDER_DARK }
                    ))
                    .rounding(egui::Rounding::same(6.0))
                    .inner_margin(egui::Margin::same(10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if is_vip {
                                ui.label(RichText::new("⭐").size(12.0));
                            }
                            
                            let id_response = ui.add(
                                egui::Label::new(RichText::new(&display_name).size(12.0).color(Colors::TEXT_PRIMARY))
                                    .sense(egui::Sense::click())
                            );
                            if id_response.clicked() {
                                self.server_browser.selected_server = Some(idx);
                            }
                            if is_vip {
                                id_response.on_hover_text(format!("VIP Server\nCode: {}", access_code.as_deref().unwrap_or("N/A")));
                            } else {
                                id_response.on_hover_text(&server_id);
                            }
                            
                            ui.add_space(40.0);
                            
                            let player_color = if is_full {
                                Colors::ACCENT_RED
                            } else if fill_pct > 80.0 {
                                Colors::ACCENT_YELLOW
                            } else {
                                Colors::ACCENT_GREEN
                            };
                            
                            ui.label(RichText::new(format!("{}/{}", playing, max_players)).color(player_color));
                            
                            ui.add_space(60.0);
                            
                            let bar_width = 80.0;
                            let (rect, _) = ui.allocate_exact_size(egui::vec2(bar_width, 12.0), egui::Sense::hover());
                            
                            let painter = ui.painter_at(rect);
                            painter.rect_filled(rect, 3.0, Colors::BG_DARK);
                            
                            let fill_rect = egui::Rect::from_min_size(
                                rect.min,
                                egui::vec2(bar_width * (fill_pct / 100.0), rect.height())
                            );
                            painter.rect_filled(fill_rect, 3.0, player_color);
                            
                            if !is_vip {
                                ui.add_space(20.0);
                                // Ping
                                let ping_str = ping.map(|p| format!("{}ms", p)).unwrap_or_else(|| "?".to_string());
                                let ping_color = ping.map(|p| {
                                    if p < 100 { Colors::ACCENT_GREEN }
                                    else if p < 200 { Colors::ACCENT_YELLOW }
                                    else { Colors::ACCENT_RED }
                                }).unwrap_or(Colors::TEXT_MUTED);
                                ui.label(RichText::new(ping_str).color(ping_color));
                            }
                            
                            ui.add_space(40.0);
                            
                            if let Some(acc_idx) = self.selected {
                                if !is_full {
                                    let btn_color = if is_vip { egui::Color32::from_rgb(255, 193, 7) } else { Colors::ACCENT_GREEN };
                                    if ui.add(egui::Button::new("Join").fill(btn_color)).clicked() {
                                        self.server_browser.selected_server = Some(idx);
                                        self.action = Action::JoinSelectedServer(acc_idx);
                                    }
                                } else {
                                    ui.add_enabled(false, egui::Button::new("Full").fill(Colors::BG_MEDIUM));
                                }
                            }
                        });
                    });
                
                ui.add_space(4.0);
            }
            
            let has_more = self.server_browser.has_more();
            if has_more {
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width() / 2.0 - 60.0);
                    let load_action = if self.server_browser.show_vip_servers {
                        Action::FetchMoreVipServers
                    } else {
                        Action::FetchMoreServers
                    };
                    if ui.add(egui::Button::new("Load More").fill(Colors::BG_LIGHT).min_size(egui::vec2(120.0, 32.0))).clicked() {
                        self.action = load_action;
                    }
                });
            }
        });
    }
    
    pub fn render_vip_access_code_modal(&mut self, ctx: &egui::Context) {
        let modal_width = 400.0;
        let modal_height = 180.0;
        
        egui::Area::new(egui::Id::new("vip_access_code_modal"))
            .fixed_pos(egui::pos2(
                (ctx.screen_rect().width() - modal_width) / 2.0,
                (ctx.screen_rect().height() - modal_height) / 2.0,
            ))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(Colors::BG_DARK)
                    .stroke(egui::Stroke::new(2.0, Colors::BORDER_ACCENT))
                    .rounding(egui::Rounding::same(12.0))
                    .inner_margin(egui::Margin::same(20.0))
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(modal_width, modal_height));
                        
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("⭐ VIP Server Access Code").size(18.0).color(egui::Color32::from_rgb(255, 193, 7)).strong());
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.add(egui::Button::new(RichText::new("X").size(14.0).color(Colors::TEXT_MUTED))
                                    .fill(Colors::BG_LIGHT)
                                    .min_size(egui::vec2(24.0, 24.0)))
                                    .clicked() 
                                {
                                    self.server_browser.vip_access_code_show = false;
                                }
                            });
                        });
                        
                        ui.add_space(12.0);
                        
                        ui.label(RichText::new("This VIP server requires an access code to join.").color(Colors::TEXT_SECONDARY));
                        ui.label(RichText::new("Get the code from the server owner.").color(Colors::TEXT_MUTED).size(12.0));
                        
                        ui.add_space(12.0);
                        
                        egui::Frame::none()
                            .fill(Colors::BG_MEDIUM)
                            .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                            .rounding(egui::Rounding::same(6.0))
                            .inner_margin(egui::Margin::same(8.0))
                            .show(ui, |ui| {
                                ui.add_sized(
                                    [modal_width - 56.0, 24.0],
                                    egui::TextEdit::singleline(&mut self.server_browser.vip_access_code_input)
                                        .hint_text("Enter access code...")
                                        .text_color(Colors::TEXT_PRIMARY)
                                        .frame(false)
                                );
                            });
                        
                        ui.add_space(16.0);
                        
                        ui.horizontal(|ui| {
                            if ui.add(egui::Button::new("Cancel")
                                .fill(Colors::BG_LIGHT)
                                .min_size(egui::vec2(80.0, 32.0)))
                                .clicked() 
                            {
                                self.server_browser.vip_access_code_show = false;
                            }
                            
                            ui.add_space(8.0);
                            
                            let can_join = !self.server_browser.vip_access_code_input.trim().is_empty();
                            ui.add_enabled_ui(can_join, |ui| {
                                if ui.add(egui::Button::new("Join Server")
                                    .fill(egui::Color32::from_rgb(255, 193, 7))
                                    .min_size(egui::vec2(100.0, 32.0)))
                                    .clicked() 
                                {
                                    self.action = Action::JoinVipWithAccessCode;
                                }
                            });
                        });
                    });
            });
    }
}
