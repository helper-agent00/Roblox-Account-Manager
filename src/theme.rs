use eframe::egui::{self, Color32, Rounding, Stroke, Vec2, Margin};

#[allow(dead_code)]
pub struct Colors;

#[allow(dead_code)]
impl Colors {
    pub const BG_DARK: Color32 = Color32::from_rgb(10, 10, 15);
    pub const BG_MEDIUM: Color32 = Color32::from_rgb(16, 16, 24);
    pub const BG_LIGHT: Color32 = Color32::from_rgb(24, 24, 35);
    pub const BG_CARD: Color32 = Color32::from_rgb(20, 20, 30);
    pub const BG_CARD_SELECTED: Color32 = Color32::from_rgb(25, 32, 55);
    pub const BG_ELEVATED: Color32 = Color32::from_rgb(30, 30, 44);
    pub const BG_INPUT: Color32 = Color32::from_rgb(14, 14, 22);

    pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(35, 35, 48);
    pub const BORDER_DARK: Color32 = Color32::from_rgb(42, 42, 56);
    pub const BORDER_LIGHT: Color32 = Color32::from_rgb(55, 55, 72);
    pub const BORDER_ACCENT: Color32 = Color32::from_rgb(70, 110, 190);

    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(235, 235, 245);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(155, 155, 175);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(95, 95, 115);
    pub const TEXT_HEADING: Color32 = Color32::from_rgb(245, 245, 255);

    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(60, 120, 220);
    pub const ACCENT_BLUE_BRIGHT: Color32 = Color32::from_rgb(80, 150, 255);
    pub const ACCENT_GREEN: Color32 = Color32::from_rgb(56, 193, 114);
    pub const ACCENT_RED: Color32 = Color32::from_rgb(225, 70, 70);
    pub const ACCENT_YELLOW: Color32 = Color32::from_rgb(240, 185, 60);
    pub const ACCENT_PURPLE: Color32 = Color32::from_rgb(130, 90, 220);

    pub const BTN_PRIMARY: Color32 = Color32::from_rgb(50, 100, 190);
    pub const BTN_SECONDARY: Color32 = Color32::from_rgb(36, 36, 50);
    pub const BTN_SECONDARY_HOVER: Color32 = Color32::from_rgb(48, 48, 66);
    pub const BTN_DANGER: Color32 = Color32::from_rgb(160, 45, 45);

    pub const NAV_BG: Color32 = Color32::from_rgb(13, 13, 20);
    pub const NAV_ACTIVE: Color32 = Color32::from_rgb(30, 40, 65);
    pub const NAV_BORDER: Color32 = Color32::from_rgb(30, 30, 42);
}

pub fn setup_dark_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.spacing.item_spacing = Vec2::new(8.0, 8.0);
    style.spacing.button_padding = Vec2::new(14.0, 7.0);
    style.spacing.window_margin = Margin::same(14.0);

    let visuals = &mut style.visuals;
    visuals.dark_mode = true;

    visuals.window_fill = Colors::BG_DARK;
    visuals.window_stroke = Stroke::new(1.0, Colors::BORDER_SUBTLE);
    visuals.window_rounding = Rounding::same(10.0);

    visuals.panel_fill = Colors::BG_DARK;

    visuals.widgets.noninteractive.bg_fill = Colors::BG_MEDIUM;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(0.5, Colors::BORDER_SUBTLE);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Colors::TEXT_SECONDARY);
    visuals.widgets.noninteractive.rounding = Rounding::same(6.0);

    visuals.widgets.inactive.bg_fill = Colors::BTN_SECONDARY;
    visuals.widgets.inactive.bg_stroke = Stroke::new(0.5, Colors::BORDER_DARK);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Colors::TEXT_PRIMARY);
    visuals.widgets.inactive.rounding = Rounding::same(6.0);

    visuals.widgets.hovered.bg_fill = Colors::BTN_SECONDARY_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Colors::BORDER_LIGHT);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Colors::TEXT_PRIMARY);
    visuals.widgets.hovered.rounding = Rounding::same(6.0);

    visuals.widgets.active.bg_fill = Colors::BTN_PRIMARY;
    visuals.widgets.active.bg_stroke = Stroke::new(1.5, Colors::BORDER_ACCENT);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Colors::TEXT_PRIMARY);
    visuals.widgets.active.rounding = Rounding::same(6.0);

    visuals.widgets.open.bg_fill = Colors::BG_ELEVATED;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, Colors::BORDER_LIGHT);
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, Colors::TEXT_PRIMARY);
    visuals.widgets.open.rounding = Rounding::same(6.0);

    visuals.selection.bg_fill = Colors::ACCENT_BLUE.linear_multiply(0.25);
    visuals.selection.stroke = Stroke::new(1.0, Colors::ACCENT_BLUE);

    visuals.extreme_bg_color = Colors::BG_INPUT;
    visuals.faint_bg_color = Colors::BG_MEDIUM;

    visuals.hyperlink_color = Colors::ACCENT_BLUE_BRIGHT;

    visuals.text_cursor.stroke = Stroke::new(2.0, Colors::ACCENT_BLUE_BRIGHT);

    ctx.set_style(style);
}

pub fn draw_status_circle(ui: &mut egui::Ui, color: Color32, size: f32) {
    let (rect, _response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let center = rect.center();
        let radius = size / 2.0 - 1.0;
        painter.circle_filled(center, radius + 3.0, color.linear_multiply(0.12));
        painter.circle_filled(center, radius + 1.5, color.linear_multiply(0.22));
        painter.circle_filled(center, radius, color);
        painter.circle_filled(
            center - Vec2::new(radius * 0.2, radius * 0.25),
            radius * 0.3,
            Color32::from_white_alpha(50),
        );
    }
}

#[allow(dead_code)]
pub fn card_frame(selected: bool) -> egui::Frame {
    egui::Frame::none()
        .fill(if selected { Colors::BG_CARD_SELECTED } else { Colors::BG_CARD })
        .stroke(Stroke::new(
            if selected { 1.5 } else { 0.5 },
            if selected { Colors::BORDER_ACCENT } else { Colors::BORDER_SUBTLE },
        ))
        .rounding(Rounding::same(8.0))
        .inner_margin(Margin::same(14.0))
        .outer_margin(Margin::same(2.0))
}

pub fn section_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Colors::BG_CARD)
        .stroke(Stroke::new(0.5, Colors::BORDER_SUBTLE))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::same(16.0))
}

pub fn input_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Colors::BG_INPUT)
        .stroke(Stroke::new(1.0, Colors::BORDER_DARK))
        .rounding(Rounding::same(6.0))
        .inner_margin(Margin::symmetric(10.0, 6.0))
}

pub fn badge_frame(color: Color32) -> egui::Frame {
    egui::Frame::none()
        .fill(color.linear_multiply(0.12))
        .stroke(Stroke::new(0.5, color.linear_multiply(0.35)))
        .rounding(Rounding::same(4.0))
        .inner_margin(Margin::symmetric(8.0, 3.0))
}

#[allow(dead_code)]
pub fn double_border_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Colors::BG_MEDIUM)
        .stroke(Stroke::new(1.5, Colors::BORDER_DARK))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::same(4.0))
        .outer_margin(Margin::same(2.0))
}

#[allow(dead_code)]
pub fn group_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Colors::BG_LIGHT)
        .stroke(Stroke::new(0.5, Colors::BORDER_SUBTLE))
        .rounding(Rounding::same(8.0))
        .inner_margin(Margin::same(12.0))
}

#[allow(dead_code)]
pub fn header_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Colors::BG_MEDIUM)
        .stroke(Stroke::new(0.5, Colors::BORDER_SUBTLE))
        .rounding(Rounding { nw: 10.0, ne: 10.0, sw: 0.0, se: 0.0 })
        .inner_margin(Margin::symmetric(16.0, 12.0))
}

pub fn primary_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(text).color(Color32::WHITE).size(13.0))
        .fill(Colors::BTN_PRIMARY)
        .stroke(Stroke::new(0.5, Colors::BORDER_ACCENT))
        .rounding(Rounding::same(6.0))
}

pub fn secondary_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(text).color(Colors::TEXT_PRIMARY).size(13.0))
        .fill(Colors::BTN_SECONDARY)
        .stroke(Stroke::new(0.5, Colors::BORDER_DARK))
        .rounding(Rounding::same(6.0))
}

pub fn danger_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(text).color(Color32::WHITE).size(13.0))
        .fill(Colors::BTN_DANGER)
        .stroke(Stroke::new(0.5, Colors::ACCENT_RED.linear_multiply(0.5)))
        .rounding(Rounding::same(6.0))
}

pub fn success_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(text).color(Color32::WHITE).size(13.0))
        .fill(Colors::ACCENT_GREEN.linear_multiply(0.7))
        .stroke(Stroke::new(0.5, Colors::ACCENT_GREEN.linear_multiply(0.5)))
        .rounding(Rounding::same(6.0))
}

pub fn icon_button(icon: &str) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(icon).size(14.0))
        .fill(Colors::BG_LIGHT)
        .stroke(Stroke::new(0.5, Colors::BORDER_DARK))
        .rounding(Rounding::same(5.0))
        .min_size(Vec2::new(28.0, 28.0))
}

pub fn section_header(ui: &mut egui::Ui, icon: &str, title: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(icon).size(16.0));
        ui.add_space(6.0);
        ui.label(egui::RichText::new(title).size(15.0).color(Colors::TEXT_HEADING).strong());
    });
    ui.add_space(10.0);
}

pub fn stat_chip(ui: &mut egui::Ui, text: &str, color: Color32) {
    egui::Frame::none()
        .fill(color.linear_multiply(0.08))
        .rounding(Rounding::same(4.0))
        .inner_margin(Margin::symmetric(8.0, 3.0))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(text).color(color).size(11.0));
        });
}

pub fn label_badge(ui: &mut egui::Ui, text: &str, color: Color32) {
    badge_frame(color).show(ui, |ui| {
        ui.label(egui::RichText::new(text).color(color).size(9.5).strong());
    });
}
