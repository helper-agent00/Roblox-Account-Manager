// Dark theme styling

use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};

// Color palette for the dark theme
#[allow(dead_code)]
pub struct Colors;

#[allow(dead_code)]
impl Colors {
    // Background colors
    pub const BG_DARK: Color32 = Color32::from_rgb(12, 12, 16);
    pub const BG_MEDIUM: Color32 = Color32::from_rgb(18, 18, 24);
    pub const BG_LIGHT: Color32 = Color32::from_rgb(28, 28, 36);
    pub const BG_CARD: Color32 = Color32::from_rgb(22, 22, 30);
    pub const BG_CARD_HOVER: Color32 = Color32::from_rgb(32, 32, 42);
    pub const BG_CARD_SELECTED: Color32 = Color32::from_rgb(35, 40, 60);
    
    // Border colors
    pub const BORDER_DARK: Color32 = Color32::from_rgb(40, 40, 50);
    pub const BORDER_LIGHT: Color32 = Color32::from_rgb(55, 55, 70);
    pub const BORDER_ACCENT: Color32 = Color32::from_rgb(80, 120, 200);
    pub const BORDER_GLOW: Color32 = Color32::from_rgb(100, 140, 220);
    
    // Text colors
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(240, 240, 245);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(160, 160, 175);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(100, 100, 115);
    
    // Accent colors
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(70, 130, 220);
    pub const ACCENT_GREEN: Color32 = Color32::from_rgb(80, 200, 120);
    pub const ACCENT_RED: Color32 = Color32::from_rgb(220, 80, 80);
    pub const ACCENT_YELLOW: Color32 = Color32::from_rgb(230, 180, 80);
    pub const ACCENT_PURPLE: Color32 = Color32::from_rgb(140, 100, 220);
    
    // Button colors
    pub const BTN_PRIMARY: Color32 = Color32::from_rgb(60, 100, 180);
    pub const BTN_PRIMARY_HOVER: Color32 = Color32::from_rgb(70, 120, 200);
    pub const BTN_SECONDARY: Color32 = Color32::from_rgb(45, 45, 55);
    pub const BTN_SECONDARY_HOVER: Color32 = Color32::from_rgb(55, 55, 70);
    pub const BTN_DANGER: Color32 = Color32::from_rgb(160, 50, 50);
    pub const BTN_DANGER_HOVER: Color32 = Color32::from_rgb(180, 60, 60);
    
    // Status colors
    pub const STATUS_VALID: Color32 = Color32::from_rgb(80, 200, 120);
    pub const STATUS_INVALID: Color32 = Color32::from_rgb(220, 80, 80);
    pub const STATUS_WARNING: Color32 = Color32::from_rgb(230, 180, 80);
    pub const STATUS_NEUTRAL: Color32 = Color32::from_rgb(120, 120, 130);
}

// Apply the custom dark theme
pub fn setup_dark_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Spacing
    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    style.spacing.button_padding = Vec2::new(16.0, 8.0);
    style.spacing.window_margin = egui::Margin::same(16.0);
    
    // Visuals
    let visuals = &mut style.visuals;
    
    // Dark mode
    visuals.dark_mode = true;
    
    // Window
    visuals.window_fill = Colors::BG_DARK;
    visuals.window_stroke = Stroke::new(1.0, Colors::BORDER_DARK);
    visuals.window_rounding = Rounding::same(12.0);
    
    // Panel
    visuals.panel_fill = Colors::BG_DARK;
    
    // Widgets
    visuals.widgets.noninteractive.bg_fill = Colors::BG_MEDIUM;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Colors::BORDER_DARK);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Colors::TEXT_SECONDARY);
    visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
    
    visuals.widgets.inactive.bg_fill = Colors::BTN_SECONDARY;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Colors::BORDER_DARK);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Colors::TEXT_PRIMARY);
    visuals.widgets.inactive.rounding = Rounding::same(8.0);
    
    visuals.widgets.hovered.bg_fill = Colors::BTN_SECONDARY_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, Colors::BORDER_LIGHT);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Colors::TEXT_PRIMARY);
    visuals.widgets.hovered.rounding = Rounding::same(8.0);
    
    visuals.widgets.active.bg_fill = Colors::BTN_PRIMARY;
    visuals.widgets.active.bg_stroke = Stroke::new(2.0, Colors::BORDER_ACCENT);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Colors::TEXT_PRIMARY);
    visuals.widgets.active.rounding = Rounding::same(8.0);
    
    visuals.widgets.open.bg_fill = Colors::BG_LIGHT;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, Colors::BORDER_LIGHT);
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, Colors::TEXT_PRIMARY);
    visuals.widgets.open.rounding = Rounding::same(8.0);
    
    // Selection
    visuals.selection.bg_fill = Colors::ACCENT_BLUE.linear_multiply(0.3);
    visuals.selection.stroke = Stroke::new(1.0, Colors::ACCENT_BLUE);
    
    // Extreme background
    visuals.extreme_bg_color = Colors::BG_DARK;
    visuals.faint_bg_color = Colors::BG_MEDIUM;
    
    // Hyperlinks
    visuals.hyperlink_color = Colors::ACCENT_BLUE;
    
    // Text cursor
    visuals.text_cursor.stroke = Stroke::new(2.0, Colors::TEXT_PRIMARY);
    
    ctx.set_style(style);
}

// Draw a status indicator circle
pub fn draw_status_circle(ui: &mut egui::Ui, color: Color32, size: f32) {
    let (rect, _response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());
    
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let center = rect.center();
        let radius = size / 2.0 - 1.0;
        
        // Outer glow
        painter.circle_filled(center, radius + 2.0, color.linear_multiply(0.2));
        
        // Main circle
        painter.circle_filled(center, radius, color);
        
        // Inner highlight
        painter.circle_filled(
            center - Vec2::new(radius * 0.25, radius * 0.25),
            radius * 0.3,
            Color32::from_white_alpha(60),
        );
    }
}

// Create a styled card frame with double border effect
#[allow(dead_code)]
pub fn card_frame(selected: bool) -> egui::Frame {
    egui::Frame::none()
        .fill(if selected { Colors::BG_CARD_SELECTED } else { Colors::BG_CARD })
        .stroke(Stroke::new(
            if selected { 2.0 } else { 1.0 },
            if selected { Colors::BORDER_ACCENT } else { Colors::BORDER_DARK }
        ))
        .rounding(Rounding::same(10.0))
        .inner_margin(egui::Margin::same(14.0))
        .outer_margin(egui::Margin::same(2.0))
}

// Create a double-bordered panel frame
#[allow(dead_code)]
pub fn double_border_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Colors::BG_MEDIUM)
        .stroke(Stroke::new(2.0, Colors::BORDER_DARK))
        .rounding(Rounding::same(12.0))
        .inner_margin(egui::Margin::same(4.0))
        .outer_margin(egui::Margin::same(2.0))
}

// Create a styled group frame
#[allow(dead_code)]
pub fn group_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Colors::BG_LIGHT)
        .stroke(Stroke::new(1.0, Colors::BORDER_DARK))
        .rounding(Rounding::same(8.0))
        .inner_margin(egui::Margin::same(12.0))
}

// Create a header frame
#[allow(dead_code)]
pub fn header_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Colors::BG_MEDIUM)
        .stroke(Stroke::new(1.0, Colors::BORDER_DARK))
        .rounding(Rounding {
            nw: 12.0,
            ne: 12.0,
            sw: 0.0,
            se: 0.0,
        })
        .inner_margin(egui::Margin::symmetric(16.0, 12.0))
}

// Styled primary button
pub fn primary_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(
        egui::RichText::new(text).color(Colors::TEXT_PRIMARY)
    )
    .fill(Colors::BTN_PRIMARY)
    .stroke(Stroke::new(1.0, Colors::BORDER_ACCENT))
    .rounding(Rounding::same(8.0))
}

// Styled secondary button
pub fn secondary_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(
        egui::RichText::new(text).color(Colors::TEXT_PRIMARY)
    )
    .fill(Colors::BTN_SECONDARY)
    .stroke(Stroke::new(1.0, Colors::BORDER_DARK))
    .rounding(Rounding::same(8.0))
}

// Styled danger button
pub fn danger_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(
        egui::RichText::new(text).color(Colors::TEXT_PRIMARY)
    )
    .fill(Colors::BTN_DANGER)
    .stroke(Stroke::new(1.0, Colors::ACCENT_RED.linear_multiply(0.5)))
    .rounding(Rounding::same(8.0))
}
