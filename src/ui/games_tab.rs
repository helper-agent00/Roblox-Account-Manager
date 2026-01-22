// Games selection tab

use eframe::egui::{self, RichText, Color32};
use crate::games::{POPULAR_GAMES, GameCategory};
use crate::theme::{self, Colors};
use super::{Action, NexusApp, Tab};

impl NexusApp {
    pub fn render_games_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("SELECT A GAME").size(13.0).color(Colors::TEXT_MUTED).strong());
            
            ui.add_space(20.0);
            
            // Search box
            ui.label(RichText::new("Search:").color(Colors::TEXT_SECONDARY));
            egui::Frame::none()
                .fill(Color32::WHITE)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(4.0))
                .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                .show(ui, |ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.game_search)
                        .desired_width(200.0)
                        .hint_text("Type to search...")
                        .text_color(Color32::from_rgb(20, 20, 30))
                        .frame(false));
                });
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add(theme::secondary_button("Done")).clicked() {
                    self.tab = Tab::Accounts;
                }
            });
        });
        
        ui.add_space(16.0);
        
        // Wrap in scroll area for better UX
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            // Current selection with large thumbnail
            if !self.place_id.is_empty() {
                let current_game = POPULAR_GAMES.iter()
                    .find(|g| g.place_id == self.place_id);
                
                // Also check user games
                let user_game = self.data.user_games.iter()
                    .find(|g| g.place_id == self.place_id);
                
                let game_name = current_game
                    .map(|g| g.name.to_string())
                    .or_else(|| user_game.map(|g| g.name.clone()))
                    .or_else(|| self.data.recent_games.iter().find(|g| g.place_id == self.place_id).map(|g| g.name.clone()))
                    .unwrap_or_else(|| "Custom ID".to_string());
                
                // Get icon URL for selected game
                let selected_icon_url = current_game
                    .and_then(|g| g.universe_id)
                    .or_else(|| user_game.and_then(|g| g.universe_id))
                    .and_then(|uid| self.game_icons.get(&uid).cloned());
                
                egui::Frame::none()
                    .fill(Colors::BG_CARD)
                    .stroke(egui::Stroke::new(2.0, Colors::BORDER_ACCENT))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Large game thumbnail
                            let thumb_size = egui::vec2(64.0, 64.0);
                            if let Some(url) = &selected_icon_url {
                                ui.add(egui::Image::from_uri(url)
                                    .fit_to_exact_size(thumb_size)
                                    .rounding(egui::Rounding::same(8.0)));
                            } else if let Some(game) = current_game {
                                // Fallback colored square
                                let (rect, _) = ui.allocate_exact_size(thumb_size, egui::Sense::hover());
                                ui.painter().rect_filled(rect, egui::Rounding::same(8.0), game.color);
                                ui.painter().text(
                                    rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    game.icon_letter,
                                    egui::FontId::proportional(32.0),
                                    Color32::WHITE
                                );
                            } else {
                                // Generic placeholder
                                let (rect, _) = ui.allocate_exact_size(thumb_size, egui::Sense::hover());
                                ui.painter().rect_filled(rect, egui::Rounding::same(8.0), Colors::BG_LIGHT);
                                ui.painter().text(
                                    rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    "?",
                                    egui::FontId::proportional(32.0),
                                    Colors::TEXT_MUTED
                                );
                            }
                            
                            ui.add_space(12.0);
                            
                            ui.vertical(|ui| {
                                ui.label(RichText::new("SELECTED GAME").size(10.0).color(Colors::TEXT_MUTED));
                                ui.label(RichText::new(game_name).size(18.0).color(Colors::TEXT_PRIMARY).strong());
                                ui.label(RichText::new(format!("Place ID: {}", self.place_id)).size(11.0).color(Colors::ACCENT_BLUE));
                            });
                        });
                    });
                
                ui.add_space(12.0);
            }
            
            // User Games section (at the top)
            ui.horizontal(|ui| {
                ui.label(RichText::new("YOUR GAMES").size(12.0).color(Colors::ACCENT_PURPLE));
                ui.add_space(8.0);
                
                let add_btn = egui::Button::new(RichText::new("+ Add Game").size(11.0).color(Colors::TEXT_PRIMARY))
                    .fill(Colors::BG_LIGHT)
                    .stroke(egui::Stroke::new(1.0, Colors::ACCENT_PURPLE.linear_multiply(0.5)))
                    .rounding(egui::Rounding::same(4.0));
                
                if ui.add(add_btn).clicked() {
                    self.add_user_game_show = !self.add_user_game_show;
                }
            });
            ui.add_space(8.0);
            
            // Add game form (expandable)
            if self.add_user_game_show {
                egui::Frame::none()
                    .fill(Colors::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Colors::ACCENT_PURPLE.linear_multiply(0.5)))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Place ID:").color(Colors::TEXT_SECONDARY).size(12.0));
                            ui.add_space(4.0);
                            egui::Frame::none()
                                .fill(Color32::WHITE)
                                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                                .rounding(egui::Rounding::same(4.0))
                                .inner_margin(egui::Margin::symmetric(6.0, 4.0))
                                .show(ui, |ui| {
                                    ui.add(egui::TextEdit::singleline(&mut self.add_user_game_place_id)
                                        .desired_width(120.0)
                                        .hint_text("e.g. 286090429")
                                        .text_color(Color32::from_rgb(20, 20, 30))
                                        .frame(false));
                                });
                            
                            ui.add_space(12.0);
                            
                            ui.label(RichText::new("Name:").color(Colors::TEXT_SECONDARY).size(12.0));
                            ui.add_space(4.0);
                            egui::Frame::none()
                                .fill(Color32::WHITE)
                                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                                .rounding(egui::Rounding::same(4.0))
                                .inner_margin(egui::Margin::symmetric(6.0, 4.0))
                                .show(ui, |ui| {
                                    ui.add(egui::TextEdit::singleline(&mut self.add_user_game_name)
                                        .desired_width(150.0)
                                        .hint_text("Game name")
                                        .text_color(Color32::from_rgb(20, 20, 30))
                                        .frame(false));
                                });
                            
                            ui.add_space(12.0);
                            
                            let can_add = !self.add_user_game_place_id.is_empty() && !self.add_user_game_name.is_empty();
                            let add_btn = egui::Button::new(RichText::new("+ Add").size(12.0).color(if can_add { Colors::TEXT_PRIMARY } else { Colors::TEXT_MUTED }))
                                .fill(if can_add { Colors::ACCENT_GREEN.linear_multiply(0.3) } else { Colors::BG_LIGHT })
                                .stroke(egui::Stroke::new(1.0, if can_add { Colors::ACCENT_GREEN } else { Colors::BORDER_DARK }))
                                .rounding(egui::Rounding::same(4.0));
                            
                            if ui.add(add_btn).clicked() && can_add {
                                self.action = Action::AddUserGame(
                                    self.add_user_game_place_id.clone(),
                                    self.add_user_game_name.clone()
                                );
                            }
                        });
                    });
                ui.add_space(8.0);
            }
            
            // Display user games
            if !self.data.user_games.is_empty() {
                egui::Frame::none()
                    .fill(Colors::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Colors::ACCENT_PURPLE.linear_multiply(0.3)))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        // Clone user games to avoid borrow issues
                        let user_games: Vec<_> = self.data.user_games.iter()
                            .map(|g| (g.place_id.clone(), g.name.clone(), g.universe_id))
                            .collect();
                        
                        ui.horizontal_wrapped(|ui| {
                            for (place_id, name, universe_id) in user_games {
                                self.render_user_game_button(ui, &place_id, &name, universe_id);
                            }
                        });
                    });
            }
            
            ui.add_space(16.0);
            
            // Recent Games section
            if !self.data.recent_games.is_empty() {
                ui.label(RichText::new("📋 RECENTLY PLAYED").size(12.0).color(Colors::ACCENT_GREEN));
                ui.add_space(8.0);
                
                egui::Frame::none()
                    .fill(Colors::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Colors::ACCENT_GREEN.linear_multiply(0.3)))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        // Clone recent games to avoid borrow issues
                        let recent_games: Vec<_> = self.data.recent_games.iter()
                            .map(|g| (g.place_id.clone(), g.name.clone(), g.play_count, g.last_played.clone()))
                            .collect();
                        
                        ui.horizontal_wrapped(|ui| {
                            for (place_id, name, play_count, last_played) in recent_games {
                                let is_selected = self.place_id == place_id;
                                
                                let btn_text = format!("{} ({}x)", name, play_count);
                                let btn = egui::Button::new(RichText::new(&btn_text).size(11.0).color(Colors::TEXT_PRIMARY))
                                    .fill(if is_selected { Colors::BG_CARD_SELECTED } else { Colors::BG_LIGHT })
                                    .stroke(egui::Stroke::new(
                                        if is_selected { 2.0 } else { 1.0 },
                                        if is_selected { Colors::BORDER_ACCENT } else { Colors::BORDER_DARK }
                                    ))
                                    .rounding(egui::Rounding::same(6.0))
                                    .min_size(egui::vec2(0.0, 28.0));
                                
                                let response = ui.add(btn);
                                if response.clicked() {
                                    self.action = Action::SelectGame(place_id.clone(), name.clone());
                                }
                                response.on_hover_text(format!("Last played: {}", last_played));
                                
                                ui.add_space(4.0);
                            }
                        });
                    });
                
                ui.add_space(16.0);
            }
            
            // Favorites section
            let favorite_games: Vec<_> = POPULAR_GAMES.iter()
                .filter(|g| self.data.favorite_games.contains(&g.place_id.to_string()))
                .collect();
            
            if !favorite_games.is_empty() {
                ui.label(RichText::new("★ FAVORITES").size(12.0).color(Colors::ACCENT_YELLOW));
                ui.add_space(8.0);
                
                egui::Frame::none()
                    .fill(Colors::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Colors::ACCENT_YELLOW.linear_multiply(0.3)))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            for game in &favorite_games {
                                self.render_game_button(ui, game.name, game.place_id, true);
                            }
                        });
                    });
                
                ui.add_space(16.0);
            }
            
            // Categories
            let categories = [
                (GameCategory::Fighting, "⚔️ FIGHTING"),
                (GameCategory::Sports, "⚽ SPORTS"),
                (GameCategory::Roleplay, "🏠 ROLEPLAY"),
                (GameCategory::Simulator, "📊 SIMULATOR"),
                (GameCategory::Horror, "👻 HORROR"),
                (GameCategory::Obby, "🏃 OBBY"),
                (GameCategory::Classic, "🎮 CLASSIC"),
            ];
            
            for (category, label) in categories {
                let games_in_cat: Vec<_> = POPULAR_GAMES.iter()
                    .filter(|g| g.category == category)
                    .filter(|g| {
                        if self.game_search.is_empty() {
                            true
                        } else {
                            g.name.to_lowercase().contains(&self.game_search.to_lowercase())
                        }
                    })
                    .collect();
                
                if games_in_cat.is_empty() {
                    continue;
                }
                
                ui.label(RichText::new(label).size(12.0).color(Colors::ACCENT_BLUE));
                ui.add_space(8.0);
                
                egui::Frame::none()
                    .fill(Colors::BG_CARD)
                    .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            for game in games_in_cat {
                                self.render_game_button(ui, game.name, game.place_id, false);
                            }
                        });
                    });
                
                ui.add_space(12.0);
            }
        });
    }
    
    fn render_game_button(&mut self, ui: &mut egui::Ui, name: &str, place_id: &str, is_favorite_section: bool) {
        let is_selected = self.place_id == place_id;
        let is_favorite = self.data.favorite_games.contains(&place_id.to_string());
        
        // Get game info for fallback icon
        let game_info = crate::games::POPULAR_GAMES.iter().find(|g| g.place_id == place_id);
        
        // Try to get universe ID and icon URL
        let universe_id = game_info.and_then(|g| g.universe_id);
        let icon_url = universe_id.and_then(|uid| self.game_icons.get(&uid).cloned());
        
        // Game button with icon - render inline without extra horizontal wrapper
        let btn_fill = if is_selected { Colors::BG_CARD_SELECTED } else { Colors::BG_LIGHT };
        let btn_stroke = egui::Stroke::new(
            if is_selected { 2.0 } else { 1.0 },
            if is_selected { Colors::BORDER_ACCENT } else { Colors::BORDER_DARK }
        );
        
        // Create a frame for the game button with icon
        egui::Frame::none()
            .fill(btn_fill)
            .stroke(btn_stroke)
            .rounding(egui::Rounding::same(6.0))
            .inner_margin(egui::Margin::symmetric(6.0, 4.0))
            .show(ui, |ui| {
                ui.set_height(28.0);  // Fixed height for alignment
                ui.horizontal_centered(|ui| {
                    // Star button (only outside favorites section) - draw it manually
                    if !is_favorite_section {
                        let star_char = if is_favorite { '*' } else { 'o' };
                        let star_color = if is_favorite { Colors::ACCENT_YELLOW } else { Colors::TEXT_MUTED };
                        
                        // Draw star as a clickable area
                        let (rect, star_response) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::click());
                        let hover_color = if star_response.hovered() { Colors::ACCENT_YELLOW } else { star_color };
                        
                        // Draw a star shape or filled/empty circle
                        let center = rect.center();
                        if is_favorite {
                            // Draw filled star (simplified as filled circle with rays)
                            ui.painter().circle_filled(center, 6.0, hover_color);
                        } else {
                            // Draw empty star (circle outline)
                            ui.painter().circle_stroke(center, 5.0, egui::Stroke::new(1.5, hover_color));
                        }
                        
                        if star_response.clicked() {
                            self.action = Action::ToggleFavoriteGame(place_id.to_string());
                            return; // Don't process further clicks
                        }
                        star_response.on_hover_text(if is_favorite { "Remove from favorites" } else { "Add to favorites" });
                    }
                    
                    // Show game icon
                    let icon_size = egui::vec2(20.0, 20.0);
                    
                    if let Some(url) = &icon_url {
                        // Load actual game icon from Roblox
                        ui.add(egui::Image::from_uri(url)
                            .fit_to_exact_size(icon_size)
                            .rounding(egui::Rounding::same(3.0)));
                    } else if let Some(game) = game_info {
                        // Fallback: show colored square with letter
                        let (rect, _) = ui.allocate_exact_size(icon_size, egui::Sense::hover());
                        ui.painter().rect_filled(rect, egui::Rounding::same(3.0), game.color);
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            game.icon_letter,
                            egui::FontId::proportional(10.0),
                            Color32::WHITE
                        );
                    }
                    
                    ui.add_space(4.0);
                    
                    // Game name - clickable to select game
                    let name_response = ui.add(
                        egui::Label::new(RichText::new(name).size(11.0).color(Colors::TEXT_PRIMARY))
                            .sense(egui::Sense::click())
                    );
                    if name_response.clicked() {
                        self.action = Action::SelectGame(place_id.to_string(), name.to_string());
                    }
                });
            });
    }
    
    fn render_user_game_button(&mut self, ui: &mut egui::Ui, place_id: &str, name: &str, universe_id: Option<u64>) {
        let is_selected = self.place_id == place_id;
        
        // Get icon URL if available
        let icon_url = universe_id.and_then(|uid| self.game_icons.get(&uid).cloned());
        
        let btn_fill = if is_selected { Colors::BG_CARD_SELECTED } else { Colors::BG_LIGHT };
        let btn_stroke = egui::Stroke::new(
            if is_selected { 2.0 } else { 1.0 },
            if is_selected { Colors::BORDER_ACCENT } else { Colors::BORDER_DARK }
        );
        
        egui::Frame::none()
            .fill(btn_fill)
            .stroke(btn_stroke)
            .rounding(egui::Rounding::same(6.0))
            .inner_margin(egui::Margin::symmetric(6.0, 4.0))
            .show(ui, |ui| {
                ui.set_height(28.0);
                ui.horizontal_centered(|ui| {
                    // Remove button - draw an X
                    let (rect, remove_response) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::click());
                    let color = if remove_response.hovered() { Colors::ACCENT_RED } else { Colors::TEXT_MUTED };
                    let painter = ui.painter();
                    let center = rect.center();
                    let size = 4.0;
                    painter.line_segment(
                        [center + egui::vec2(-size, -size), center + egui::vec2(size, size)],
                        egui::Stroke::new(2.0, color)
                    );
                    painter.line_segment(
                        [center + egui::vec2(size, -size), center + egui::vec2(-size, size)],
                        egui::Stroke::new(2.0, color)
                    );
                    if remove_response.clicked() {
                        self.action = Action::RemoveUserGame(place_id.to_string());
                        return;  // Don't process any more clicks
                    }
                    remove_response.on_hover_text("Remove from list");
                    
                    ui.add_space(4.0);
                    
                    // Show game icon
                    let icon_size = egui::vec2(20.0, 20.0);
                    
                    if let Some(url) = &icon_url {
                        ui.add(egui::Image::from_uri(url)
                            .fit_to_exact_size(icon_size)
                            .rounding(egui::Rounding::same(3.0)));
                    } else {
                        // Generic placeholder for user games
                        let (rect, _) = ui.allocate_exact_size(icon_size, egui::Sense::hover());
                        ui.painter().rect_filled(rect, egui::Rounding::same(3.0), Colors::ACCENT_PURPLE.linear_multiply(0.5));
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            name.chars().next().unwrap_or('?'),
                            egui::FontId::proportional(10.0),
                            Color32::WHITE
                        );
                    }
                    
                    ui.add_space(4.0);
                    
                    // Game name - clickable to select
                    let name_response = ui.add(
                        egui::Label::new(RichText::new(name).size(11.0).color(Colors::TEXT_PRIMARY))
                            .sense(egui::Sense::click())
                    );
                    if name_response.clicked() {
                        self.action = Action::SelectGame(place_id.to_string(), name.to_string());
                    }
                });
            });
    }
}
