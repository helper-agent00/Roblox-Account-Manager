// Account utilities tab

use eframe::egui::{self, RichText, Color32};
use crate::theme::Colors;
use super::{Action, NexusApp, Tab};

impl NexusApp {
    pub fn render_account_utils_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("🔧 ACCOUNT UTILITIES").size(13.0).color(Colors::TEXT_MUTED).strong());
            
            ui.add_space(20.0);
            
            // Show selected account
            if let Some(idx) = self.selected {
                if let Some(account) = self.data.accounts.get(idx) {
                    ui.label(RichText::new("Selected:").color(Colors::TEXT_SECONDARY));
                    ui.label(RichText::new(&account.username).color(Colors::ACCENT_BLUE).strong());
                    
                    if let Some(ref display_name) = account.display_name {
                        ui.label(RichText::new(format!("({})", display_name)).color(Colors::TEXT_MUTED));
                    }
                }
            } else {
                ui.label(RichText::new("⚠ Select an account first").color(Colors::ACCENT_YELLOW));
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add(egui::Button::new("👥 Accounts").fill(Colors::BG_LIGHT)).clicked() {
                    self.tab = Tab::Accounts;
                }
            });
        });
        
        ui.add_space(16.0);
        
        if self.selected.is_none() {
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(20.0))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("No account selected").size(16.0).color(Colors::TEXT_MUTED));
                        ui.add_space(8.0);
                        ui.label(RichText::new("Go to Accounts tab and select an account first").color(Colors::TEXT_SECONDARY));
                    });
                });
            return;
        }
        
        let idx = self.selected.unwrap();
        
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            // Session Management
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.label(RichText::new("🔐 Session Management").size(14.0).color(Colors::TEXT_PRIMARY).strong());
                    ui.add_space(12.0);
                    
                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new("🚪 Logout Other Sessions")
                            .fill(Colors::ACCENT_RED.linear_multiply(0.8))
                            .min_size(egui::vec2(180.0, 32.0))).clicked() 
                        {
                            self.action = Action::LogoutOtherSessions(idx);
                        }
                        
                        ui.label(RichText::new("Signs out of all other devices").color(Colors::TEXT_MUTED));
                    });
                });
            
            ui.add_space(12.0);
            
            // Change Password
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.label(RichText::new("🔑 Change Password").size(14.0).color(Colors::TEXT_PRIMARY).strong());
                    ui.add_space(12.0);
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("New Password:").color(Colors::TEXT_SECONDARY));
                        
                        egui::Frame::none()
                            .fill(Color32::WHITE)
                            .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                            .rounding(egui::Rounding::same(4.0))
                            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                            .show(ui, |ui| {
                                ui.add(egui::TextEdit::singleline(&mut self.util_new_password)
                                    .password(true)
                                    .desired_width(200.0)
                                    .hint_text("Enter new password")
                                    .text_color(Color32::from_rgb(20, 20, 30))
                                    .frame(false));
                            });
                        
                        if ui.add(egui::Button::new("Change")
                            .fill(Colors::ACCENT_BLUE)
                            .min_size(egui::vec2(80.0, 28.0))).clicked() 
                        {
                            self.action = Action::ChangePassword(idx);
                        }
                    });
                    
                    ui.add_space(4.0);
                    ui.label(RichText::new("⚠ Make sure to save your new password!").size(11.0).color(Colors::ACCENT_YELLOW));
                });
            
            ui.add_space(12.0);
            
            // Change Display Name
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.label(RichText::new("📝 Change Display Name").size(14.0).color(Colors::TEXT_PRIMARY).strong());
                    ui.add_space(12.0);
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("New Display Name:").color(Colors::TEXT_SECONDARY));
                        
                        egui::Frame::none()
                            .fill(Color32::WHITE)
                            .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                            .rounding(egui::Rounding::same(4.0))
                            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                            .show(ui, |ui| {
                                ui.add(egui::TextEdit::singleline(&mut self.util_new_display_name)
                                    .desired_width(200.0)
                                    .hint_text("Enter new display name")
                                    .text_color(Color32::from_rgb(20, 20, 30))
                                    .frame(false));
                            });
                        
                        if ui.add(egui::Button::new("Change")
                            .fill(Colors::ACCENT_BLUE)
                            .min_size(egui::vec2(80.0, 28.0))).clicked() 
                        {
                            self.action = Action::SetDisplayName(idx);
                        }
                    });
                    
                    ui.add_space(4.0);
                    ui.label(RichText::new("Display names can only be changed once per week").size(11.0).color(Colors::TEXT_MUTED));
                });
            
            ui.add_space(12.0);
            
            // User Actions
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.label(RichText::new("👥 User Actions").size(14.0).color(Colors::TEXT_PRIMARY).strong());
                    ui.add_space(12.0);
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Target Username:").color(Colors::TEXT_SECONDARY));
                        
                        egui::Frame::none()
                            .fill(Color32::WHITE)
                            .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                            .rounding(egui::Rounding::same(4.0))
                            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                            .show(ui, |ui| {
                                ui.add(egui::TextEdit::singleline(&mut self.util_target_user)
                                    .desired_width(200.0)
                                    .hint_text("Enter username")
                                    .text_color(Color32::from_rgb(20, 20, 30))
                                    .frame(false));
                            });
                    });
                    
                    ui.add_space(8.0);
                    
                    ui.horizontal(|ui| {
                        // Block button
                        if ui.add(egui::Button::new("🚫 Block")
                            .fill(Colors::ACCENT_RED.linear_multiply(0.8))
                            .min_size(egui::vec2(100.0, 28.0))).clicked() 
                        {
                            self.do_user_action(idx, "block");
                        }
                        
                        // Unblock button
                        if ui.add(egui::Button::new(" Unblock")
                            .fill(Colors::ACCENT_GREEN.linear_multiply(0.8))
                            .min_size(egui::vec2(100.0, 28.0))).clicked() 
                        {
                            self.do_user_action(idx, "unblock");
                        }
                        
                        // Friend request button
                        if ui.add(egui::Button::new("+ Send Friend Request")
                            .fill(Colors::ACCENT_BLUE)
                            .min_size(egui::vec2(150.0, 28.0))).clicked() 
                        {
                            self.do_user_action(idx, "friend");
                        }
                    });
                });
            
            ui.add_space(12.0);
            
            // Account Organization (Groups and Notes)
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.label(RichText::new("Account Organization").size(14.0).color(Colors::TEXT_PRIMARY).strong());
                    ui.add_space(12.0);
                    
                    // Group selection
                    let current_group = self.data.accounts.get(idx)
                        .map(|a| a.group.clone())
                        .unwrap_or_default();
                    
                    // Get all existing groups for suggestions (clone to avoid borrow issues)
                    let existing_groups: Vec<String> = self.data.accounts.iter()
                        .map(|a| a.group.clone())
                        .filter(|g| !g.is_empty())
                        .collect::<std::collections::HashSet<_>>()
                        .into_iter()
                        .collect();
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Group:").color(Colors::TEXT_SECONDARY));
                        
                        egui::Frame::none()
                            .fill(Color32::WHITE)
                            .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                            .rounding(egui::Rounding::same(4.0))
                            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                            .show(ui, |ui| {
                                let mut group = current_group.clone();
                                let response = ui.add(egui::TextEdit::singleline(&mut group)
                                    .desired_width(150.0)
                                    .hint_text("No group")
                                    .text_color(Color32::from_rgb(20, 20, 30))
                                    .frame(false));
                                
                                if response.changed() {
                                    if let Some(account) = self.data.accounts.get_mut(idx) {
                                        account.group = group;
                                        self.data.save();
                                    }
                                }
                            });
                    });
                    
                    // Quick group buttons on separate row
                    if !existing_groups.is_empty() {
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Quick:").color(Colors::TEXT_SECONDARY).size(11.0));
                            ui.add_space(8.0);
                            
                            // Define some nice colors for group buttons
                            let group_colors = [
                                Colors::ACCENT_BLUE,
                                Colors::ACCENT_GREEN,
                                Color32::from_rgb(156, 89, 182),  // Purple
                                Colors::ACCENT_YELLOW,
                                Color32::from_rgb(230, 126, 34),  // Orange
                                Color32::from_rgb(52, 152, 219),  // Light blue
                            ];
                            
                            for group in existing_groups.iter() {
                                if group != &current_group {
                                    // Use a stable hash of the group name for consistent color
                                    let hash: usize = group.bytes().fold(0usize, |acc, b| acc.wrapping_add(b as usize).wrapping_mul(31));
                                    let color = group_colors[hash % group_colors.len()];
                                    if ui.add(egui::Button::new(RichText::new(group).color(Colors::TEXT_PRIMARY).size(11.0))
                                        .fill(color.linear_multiply(0.3))
                                        .stroke(egui::Stroke::new(1.0, color.linear_multiply(0.6)))
                                        .rounding(egui::Rounding::same(4.0))
                                        .min_size(egui::vec2(60.0, 24.0))).clicked() 
                                    {
                                        if let Some(account) = self.data.accounts.get_mut(idx) {
                                            account.group = group.clone();
                                            self.data.save();
                                        }
                                    }
                                    ui.add_space(8.0);
                                }
                            }
                        });
                    }
                    
                    ui.add_space(8.0);
                    
                    // Notes
                    ui.label(RichText::new("Notes:").color(Colors::TEXT_SECONDARY));
                    
                    let current_notes = self.data.accounts.get(idx)
                        .map(|a| a.notes.clone())
                        .unwrap_or_default();
                    
                    egui::Frame::none()
                        .fill(Color32::WHITE)
                        .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                        .rounding(egui::Rounding::same(4.0))
                        .inner_margin(egui::Margin::same(8.0))
                        .show(ui, |ui| {
                            let mut notes = current_notes.clone();
                            let response = ui.add(egui::TextEdit::multiline(&mut notes)
                                .desired_width(ui.available_width() - 16.0)
                                .desired_rows(3)
                                .hint_text("Add notes about this account...")
                                .text_color(Color32::from_rgb(20, 20, 30))
                                .frame(false));
                            
                            if response.changed() {
                                if let Some(account) = self.data.accounts.get_mut(idx) {
                                    account.notes = notes;
                                    self.data.save();
                                }
                            }
                        });
                });
            
            ui.add_space(12.0);
            
            // Quick Actions for all accounts
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .stroke(egui::Stroke::new(1.0, Colors::BORDER_DARK))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.label(RichText::new("⚡ Quick Actions").size(14.0).color(Colors::TEXT_PRIMARY).strong());
                    ui.add_space(12.0);
                    
                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new("📋 Copy Cookie")
                            .fill(Colors::ACCENT_BLUE.linear_multiply(0.7))
                            .stroke(egui::Stroke::new(1.0, Colors::ACCENT_BLUE))
                            .min_size(egui::vec2(120.0, 28.0))).clicked() 
                        {
                            if let Some(cookie) = self.data.accounts.get(idx).and_then(|a| a.cookie.clone()) {
                                ui.output_mut(|o| o.copied_text = cookie);
                                self.set_status(" Cookie copied to clipboard", false);
                            }
                        }
                        
                        if ui.add(egui::Button::new("🆔 Copy User ID")
                            .fill(Colors::ACCENT_BLUE.linear_multiply(0.7))
                            .stroke(egui::Stroke::new(1.0, Colors::ACCENT_BLUE))
                            .min_size(egui::vec2(120.0, 28.0))).clicked() 
                        {
                            if let Some(user_id) = self.data.accounts.get(idx).and_then(|a| a.user_id) {
                                ui.output_mut(|o| o.copied_text = user_id.to_string());
                                self.set_status(" User ID copied to clipboard", false);
                            }
                        }
                        
                        if ui.add(egui::Button::new("🔗 Open Profile")
                            .fill(Colors::ACCENT_BLUE.linear_multiply(0.7))
                            .stroke(egui::Stroke::new(1.0, Colors::ACCENT_BLUE))
                            .min_size(egui::vec2(120.0, 28.0))).clicked() 
                        {
                            if let Some(user_id) = self.data.accounts.get(idx).and_then(|a| a.user_id) {
                                let url = format!("https://www.roblox.com/users/{}/profile", user_id);
                                #[cfg(windows)]
                                {
                                    use std::os::windows::process::CommandExt;
                                    const CREATE_NO_WINDOW: u32 = 0x08000000;
                                    let _ = std::process::Command::new("cmd")
                                        .args(["/C", "start", "", &url])
                                        .creation_flags(CREATE_NO_WINDOW)
                                        .spawn();
                                }
                                self.set_status(" Opened profile in browser", false);
                            }
                        }
                    });
                });
        });
    }
    
    fn do_user_action(&mut self, account_idx: usize, action_type: &str) {
        let target = self.util_target_user.trim().to_string();
        
        if target.is_empty() {
            self.set_status("Enter a target username", true);
            return;
        }
        
        // Queue the action - user lookup will happen in process_action
        match action_type {
            "block" => self.action = Action::BlockUserByName(account_idx, target),
            "unblock" => self.action = Action::UnblockUserByName(account_idx, target),
            "friend" => self.action = Action::SendFriendRequestByName(account_idx, target),
            _ => {}
        }
    }
}
