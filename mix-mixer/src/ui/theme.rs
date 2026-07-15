use eframe::egui::{
    self, pos2, vec2, Align2, Color32, Context, FontId, Frame, Id, Layout, Margin, Order, Rounding,
    Sense, Stroke, Ui,
};

use crate::audio::metrics::MetricsSnapshot;
use crate::i18n::{Locale, UiText};

pub struct Theme;

impl Theme {
    // iOS dark — UIColor system palette
    pub const BG: Color32 = Color32::from_rgb(0, 0, 0);
    pub const GROUP: Color32 = Color32::from_rgb(28, 28, 30);
    pub const SEPARATOR: Color32 = Color32::from_rgb(56, 56, 58);
    pub const TEXT: Color32 = Color32::from_rgb(255, 255, 255);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(174, 174, 178);
    pub const TEXT_TERTIARY: Color32 = Color32::from_rgb(118, 118, 122);
    pub const ACCENT: Color32 = Color32::from_rgb(10, 132, 255);
    pub const GREEN: Color32 = Color32::from_rgb(48, 209, 88);
    pub const WARNING: Color32 = Color32::from_rgb(255, 214, 10);
    pub const ERROR: Color32 = Color32::from_rgb(255, 69, 58);
    pub const CHEVRON: Color32 = Color32::from_rgb(142, 142, 147);
    pub const BTN_SECONDARY: Color32 = Color32::from_rgb(58, 58, 60);
    /// Subtle row hover (~4% white on dark). Visible but non-distracting (WCAG UI component guidance).
    pub const ROW_HOVER: Color32 = Color32::from_rgba_premultiplied(10, 10, 10, 10);
    pub const ROW_HOVER_RADIUS: f32 = 10.0;
    pub const ROW_HOVER_INSET_V: f32 = 3.0;

    pub const WINDOW_W: f32 = 480.0;

    pub const HEADER_BODY_H: f32 = 52.0;
    pub const HEADER_FRAME_V: f32 = 20.0;
    pub const FOOTER_BODY_H: f32 = 36.0;
    pub const FOOTER_FRAME_V: f32 = 28.0;

    pub const SECTION_FIRST_H: f32 = 33.0;
    pub const SECTION_NEXT_H: f32 = 41.0;
    pub const SECTION_FOOTER_H: f32 = 28.0;

    pub const TOAST_DURATION_SECS: f32 = 2.5;
    pub const TOAST_FADE_SECS: f32 = 0.3;

    pub fn header_height() -> f32 {
        Self::HEADER_BODY_H + Self::HEADER_FRAME_V
    }

    pub fn footer_height() -> f32 {
        Self::FOOTER_BODY_H + Self::FOOTER_FRAME_V
    }

    /// Client height sized to fit all rows without scrolling.
    pub fn window_height() -> f32 {
        let central = Self::SECTION_FIRST_H
            + Self::ROW_H
            + Self::SECTION_NEXT_H
            + Self::ROW_H * 3.0
            + 2.0
            + Self::SECTION_NEXT_H
            + Self::ROW_H
            + (Self::ROW_H + 1.0) * 3.0
            + Self::SECTION_FOOTER_H;
        Self::header_height() + central + Self::footer_height() + 12.0
    }

    pub const INSET: f32 = 16.0;
    pub const ROW_H: f32 = 44.0;
    /// Label column width shared by toggle, picker, and slider rows.
    pub const ROW_LABEL_W: f32 = 120.0;
    pub const ROW_VALUE_W: f32 = 36.0;
    pub const ROW_GAP: f32 = 10.0;
    pub const SLIDER_TRACK_H: f32 = 4.0;
    pub const SLIDER_THUMB_R: f32 = 8.0;
    pub const SLIDER_TRACK_BG: Color32 = Color32::from_rgb(60, 60, 64);
    pub const GROUP_RADIUS: f32 = 12.0;
    pub const SWITCH_W: f32 = 51.0;
    pub const SWITCH_H: f32 = 31.0;

    pub fn apply(ctx: &Context) {
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = vec2(0.0, 0.0);
        style.spacing.button_padding = vec2(16.0, 10.0);
        style.spacing.indent = 0.0;
        style.spacing.slider_width = 320.0;
        style.spacing.slider_rail_height = 4.0;
        style.spacing.interact_size = vec2(32.0, 32.0);
        style.interaction.selectable_labels = false;

        let v = &mut style.visuals;
        v.dark_mode = true;
        v.override_text_color = Some(Self::TEXT);
        v.window_fill = Self::BG;
        v.panel_fill = Self::BG;
        v.extreme_bg_color = Self::GROUP;
        v.faint_bg_color = Self::GROUP;
        v.window_stroke = Stroke::NONE;
        v.window_rounding = Rounding::same(12.0);
        v.widgets.noninteractive.bg_fill = Self::GROUP;
        v.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, Self::SEPARATOR);
        v.widgets.noninteractive.rounding = Rounding::same(8.0);
        v.widgets.inactive.bg_fill = Color32::TRANSPARENT;
        v.widgets.inactive.fg_stroke = Stroke::NONE;
        v.widgets.inactive.rounding = Rounding::same(8.0);
        v.widgets.hovered.bg_fill = Color32::TRANSPARENT;
        v.widgets.hovered.fg_stroke = Stroke::NONE;
        v.widgets.active.bg_fill = Self::ACCENT;
        v.widgets.active.fg_stroke = Stroke::NONE;
        v.widgets.open.bg_fill = Color32::TRANSPARENT;
        v.selection.bg_fill = Color32::from_rgba_premultiplied(10, 132, 255, 90);
        v.selection.stroke = Stroke::NONE;
        v.hyperlink_color = Self::ACCENT;
        v.warn_fg_color = Self::WARNING;
        v.error_fg_color = Self::ERROR;
        v.widgets.inactive.weak_bg_fill = Self::SEPARATOR;
        v.widgets.noninteractive.weak_bg_fill = Self::SEPARATOR;
        v.slider_trailing_fill = true;
        v.handle_shape = egui::style::HandleShape::Circle;

        ctx.set_style(style);
    }
}

fn body_text(text: &str) -> egui::RichText {
    egui::RichText::new(text).size(15.0).color(Theme::TEXT)
}

fn caption_text(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .size(13.0)
        .color(Theme::TEXT_TERTIARY)
}

fn value_text(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .size(15.0)
        .color(Theme::TEXT_SECONDARY)
}

/// Inset, rounded hover fill aligned with grouped list rows (iOS-style).
fn paint_row_hover(ui: &Ui, rect: egui::Rect) {
    let hover_rect = egui::Rect::from_min_max(
        pos2(rect.left(), rect.top() + Theme::ROW_HOVER_INSET_V),
        pos2(rect.right(), rect.bottom() - Theme::ROW_HOVER_INSET_V),
    );
    ui.painter().rect_filled(
        hover_rect,
        Rounding::same(Theme::ROW_HOVER_RADIUS),
        Theme::ROW_HOVER,
    );
}

/// One-line picker row: label left, value right; entire row opens the menu (no text selection).
fn dropdown_row(
    ui: &mut Ui,
    id: &str,
    label: &str,
    value: &str,
    first: bool,
) -> (egui::Response, Id) {
    if !first {
        inset_separator(ui);
    }

    let popup_id = Id::new(id);
    let display = truncate_end(value, 40);
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(vec2(width, Theme::ROW_H), Sense::click());

    if response.hovered() {
        paint_row_hover(ui, rect);
    }

    let painter = ui.painter();
    painter.text(
        pos2(rect.left() + Theme::INSET, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(15.0),
        Theme::TEXT,
    );
    painter.text(
        pos2(rect.right() - Theme::INSET, rect.center().y),
        Align2::RIGHT_CENTER,
        display,
        FontId::proportional(15.0),
        Theme::TEXT_SECONDARY,
    );

    (response, popup_id)
}

/// Compact EN / FR chip for the header. Returns true if locale changed.
fn header_locale_chip(ui: &mut Ui, current: &mut Locale, target: Locale) -> bool {
    let active = *current == target;
    let code = target.short_code();
    let (rect, response) = ui.allocate_exact_size(vec2(26.0, 18.0), Sense::click());

    let color = if active {
        Theme::TEXT
    } else {
        Theme::TEXT_TERTIARY
    };

    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        code,
        FontId::proportional(12.0),
        color,
    );

    if response.hovered() && !active {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    if response.clicked() && !active {
        *current = target;
        return true;
    }
    false
}

fn header_locale_chips(ui: &mut Ui, locale: &mut Locale) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        if header_locale_chip(ui, locale, Locale::Fr) {
            changed = true;
        }
        if header_locale_chip(ui, locale, Locale::En) {
            changed = true;
        }
    });
    changed
}

/// Routing toggle only (locale lives in the header).
pub fn routing_row(ui: &mut Ui, texts: &UiText, routing: &mut bool) {
    toggle_row(ui, texts.routing_enable, true, routing);
}

pub fn header(ui: &mut Ui, snap: &MetricsSnapshot, texts: &UiText, locale: &mut Locale) -> bool {
    let mut locale_changed = false;
    ui.set_min_height(Theme::HEADER_BODY_H);

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(body_text("MixMixer").strong().size(17.0));
            ui.add_space(4.0);
            ui.label(caption_text(texts.header_subtitle));
        });

        ui.with_layout(Layout::top_down(egui::Align::RIGHT), |ui| {
            let (label, color) = if snap.routing_live {
                (texts.status_active, Theme::GREEN)
            } else if snap.reconnect_pending {
                (texts.status_reconnecting, Theme::WARNING)
            } else {
                (texts.status_inactive, Theme::TEXT_SECONDARY)
            };
            ui.label(
                egui::RichText::new(format!(
                    "{label} · {:.1} ms · {:.0} %",
                    snap.estimated_latency_ms, snap.voice_buffer_pct
                ))
                .size(13.0)
                .color(color),
            );
            ui.add_space(2.0);
            if header_locale_chips(ui, locale) {
                locale_changed = true;
            }
        });
    });

    locale_changed
}

pub fn section_header(ui: &mut Ui, title: &str, first: bool) {
    ui.add_space(if first { 12.0 } else { 20.0 });
    ui.label(
        egui::RichText::new(title)
            .size(13.0)
            .color(Theme::TEXT_TERTIARY),
    );
    ui.add_space(8.0);
}

pub fn section_footer(ui: &mut Ui, text: &str) {
    ui.add_space(8.0);
    ui.label(caption_text(text));
}

pub fn group_box<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::none()
        .fill(Theme::GROUP)
        .rounding(Rounding::same(Theme::GROUP_RADIUS))
        .inner_margin(Margin::same(0.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            add_contents(ui)
        })
        .inner
}

/// iOS-style inset divider — painter, not egui Separator (which renders white).
fn inset_separator(ui: &mut Ui) {
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(width, 1.0), Sense::hover());
    let y = rect.center().y;
    ui.painter().hline(
        (rect.left() + Theme::INSET)..=(rect.right() - Theme::INSET),
        y,
        Stroke::new(1.0_f32, Theme::SEPARATOR),
    );
}

fn ios_switch(ui: &mut Ui, on: &mut bool) -> egui::Response {
    let (rect, response) =
        ui.allocate_exact_size(vec2(Theme::SWITCH_W, Theme::SWITCH_H), Sense::click());

    if response.clicked() {
        *on = !*on;
    }

    let t = ui.ctx().animate_bool(response.id, *on);
    let painter = ui.painter();
    let track = Color32::from_rgb(lerp_u8(57, 48, t), lerp_u8(57, 209, t), lerp_u8(61, 88, t));
    painter.rect_filled(rect, Rounding::same(Theme::SWITCH_H / 2.0), track);

    let thumb_r = 13.5;
    let thumb_x = egui::lerp(
        (rect.left() + thumb_r + 2.0)..=(rect.right() - thumb_r - 2.0),
        t,
    );
    let thumb_center = pos2(thumb_x, rect.center().y);
    painter.circle_filled(
        thumb_center + vec2(0.0, 1.0),
        thumb_r,
        Color32::from_black_alpha(40),
    );
    painter.circle_filled(thumb_center, thumb_r, Color32::WHITE);

    response
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

fn list_row<R>(
    ui: &mut Ui,
    height: f32,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<R> {
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), height),
        Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.set_width(ui.available_width());
            add_contents(ui)
        },
    )
}

fn row_label_cell(ui: &mut Ui, label: &str) {
    ui.add_space(Theme::INSET);
    ui.allocate_ui_with_layout(
        vec2(Theme::ROW_LABEL_W, Theme::ROW_H),
        Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.label(body_text(label));
        },
    );
}

fn row_trailing_flex(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
        ui.set_width(ui.available_width());
        ui.add_space(Theme::INSET);
        add_contents(ui);
    });
}

fn slider_area_width(ui: &Ui) -> f32 {
    let reserve = Theme::ROW_VALUE_W + Theme::INSET + Theme::ROW_GAP;
    (ui.available_width() - reserve).max(120.0)
}

pub fn toggle_row(ui: &mut Ui, label: &str, first: bool, value: &mut bool) {
    if !first {
        inset_separator(ui);
    }

    list_row(ui, Theme::ROW_H, |ui| {
        row_label_cell(ui, label);
        row_trailing_flex(ui, |ui| {
            ios_switch(ui, value);
        });
    });
}

pub fn picker_row(
    ui: &mut Ui,
    id: &str,
    label: &str,
    first: bool,
    names: &[String],
    selected: &mut String,
) {
    if !first {
        inset_separator(ui);
    }

    if names.is_empty() {
        list_row(ui, Theme::ROW_H, |ui| {
            row_label_cell(ui, label);
            row_trailing_flex(ui, |ui| {
                ui.add(
                    egui::TextEdit::singleline(selected)
                        .desired_width(180.0)
                        .font(FontId::proportional(15.0)),
                );
            });
        });
        return;
    }

    let display = if names.iter().any(|name| name == selected) {
        selected.clone()
    } else {
        names
            .get(crate::devices::best_device_index(names, selected))
            .cloned()
            .unwrap_or_else(|| selected.clone())
    };

    let (response, popup_id) = dropdown_row(ui, id, label, &truncate_end(&display, 28), true);

    if response.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }

    egui::popup::popup_below_widget(
        ui,
        popup_id,
        &response,
        egui::popup::PopupCloseBehavior::CloseOnClickOutside,
        |ui| {
            ui.set_min_width(ui.available_width().max(320.0));
            Frame::popup(ui.style())
                .fill(Theme::GROUP)
                .stroke(Stroke::new(1.0_f32, Theme::SEPARATOR))
                .rounding(Rounding::same(10.0))
                .show(ui, |ui| {
                    for name in names {
                        let is_selected = *selected == *name;
                        let item = ui.selectable_label(is_selected, body_text(name));
                        if item.clicked() {
                            *selected = name.clone();
                            ui.close_menu();
                        }
                    }
                });
        },
    );
}

fn slider_t(value: f32, range: &std::ops::RangeInclusive<f32>) -> f32 {
    let start = *range.start();
    let end = *range.end();
    if (end - start).abs() < f32::EPSILON {
        return 0.0;
    }
    ((value - start) / (end - start)).clamp(0.0, 1.0)
}

fn value_from_t(t: f32, range: &std::ops::RangeInclusive<f32>) -> f32 {
    let start = *range.start();
    let end = *range.end();
    start + t * (end - start)
}

fn ios_inline_slider_f32(
    ui: &mut Ui,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    width: f32,
) -> egui::Response {
    let height = Theme::ROW_H;
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click_and_drag());

    if response.clicked() || response.dragged() {
        if let Some(pos) = response.interact_pointer_pos() {
            let t = ((pos.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
            *value = value_from_t(t, &range);
        }
    }

    let t = slider_t(*value, &range);
    paint_ios_slider(ui, rect, t);

    response
}

fn ios_inline_slider_buffer(ui: &mut Ui, value: &mut u32, width: f32) -> egui::Response {
    let height = Theme::ROW_H;
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click_and_drag());
    let range = 128.0..=512.0;
    let mut frames = *value as f32;

    if response.clicked() || response.dragged() {
        if let Some(pos) = response.interact_pointer_pos() {
            let t = ((pos.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
            frames = value_from_t(t, &range);
            frames = (frames / 128.0).round() * 128.0;
            *value = frames as u32;
        }
    }

    let t = slider_t(*value as f32, &range);
    paint_ios_slider(ui, rect, t);

    response
}

pub fn slider_block(
    ui: &mut Ui,
    label: &str,
    first: bool,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
) {
    if !first {
        inset_separator(ui);
    }

    list_row(ui, Theme::ROW_H, |ui| {
        row_label_cell(ui, label);
        ui.add_space(Theme::ROW_GAP);
        let slider_w = slider_area_width(ui);
        ios_inline_slider_f32(ui, value, range, slider_w);
        ui.add_space(Theme::ROW_GAP);
        row_trailing_flex(ui, |ui| {
            ui.label(value_text(&format!("{value:.2}")));
        });
    });
}

pub fn buffer_block(ui: &mut Ui, first: bool, value: &mut u32, label: &str) {
    if !first {
        inset_separator(ui);
    }

    list_row(ui, Theme::ROW_H, |ui| {
        row_label_cell(ui, label);
        ui.add_space(Theme::ROW_GAP);
        let slider_w = slider_area_width(ui);
        ios_inline_slider_buffer(ui, value, slider_w);
        ui.add_space(Theme::ROW_GAP);
        row_trailing_flex(ui, |ui| {
            ui.label(value_text(&format!("{value}")));
        });
    });
}

fn paint_ios_slider(ui: &Ui, rect: egui::Rect, t: f32) {
    let painter = ui.painter();
    let track_y = rect.center().y;
    let half_h = Theme::SLIDER_TRACK_H / 2.0;
    let track_left = rect.left();
    let track_right = rect.right();
    let thumb_x = egui::lerp(track_left..=track_right, t);

    let track_rect = egui::Rect::from_min_max(
        pos2(track_left, track_y - half_h),
        pos2(track_right, track_y + half_h),
    );
    painter.rect_filled(track_rect, Rounding::same(2.0), Theme::SLIDER_TRACK_BG);

    if thumb_x > track_left + 1.0 {
        let fill_rect = egui::Rect::from_min_max(
            pos2(track_left, track_y - half_h),
            pos2(thumb_x, track_y + half_h),
        );
        painter.rect_filled(fill_rect, Rounding::same(2.0), Theme::ACCENT);
    }

    let thumb_center = pos2(thumb_x, track_y);
    painter.circle_filled(
        thumb_center + vec2(0.0, 1.0),
        Theme::SLIDER_THUMB_R,
        Color32::from_black_alpha(35),
    );
    painter.circle_filled(thumb_center, Theme::SLIDER_THUMB_R, Color32::WHITE);
    painter.circle_stroke(
        thumb_center,
        Theme::SLIDER_THUMB_R,
        Stroke::new(0.5_f32, Color32::from_black_alpha(40)),
    );
}

/// Floating toast overlay (does not affect window layout).
pub fn toast(ctx: &Context, message: &str, ok: bool, age_secs: f32) {
    let total = Theme::TOAST_DURATION_SECS + Theme::TOAST_FADE_SECS;
    if age_secs >= total {
        return;
    }

    let fade = Theme::TOAST_FADE_SECS;
    let alpha = if age_secs < fade {
        age_secs / fade
    } else if age_secs > Theme::TOAST_DURATION_SECS {
        1.0 - (age_secs - Theme::TOAST_DURATION_SECS) / fade
    } else {
        1.0
    }
    .clamp(0.0, 1.0);

    let accent = if ok { Theme::GREEN } else { Theme::ERROR };
    let bg_alpha = (230.0 * alpha) as u8;
    let offset_y = -(Theme::footer_height() + 20.0);

    egui::Area::new(Id::new("mixmixer_toast"))
        .anchor(Align2::CENTER_BOTTOM, vec2(0.0, offset_y))
        .order(Order::Foreground)
        .interactable(false)
        .show(ctx, |ui| {
            Frame::none()
                .fill(Color32::from_rgba_premultiplied(44, 44, 46, bg_alpha))
                .stroke(Stroke::new(1.0_f32, accent.gamma_multiply(0.35 * alpha)))
                .inner_margin(Margin::symmetric(18.0, 11.0))
                .rounding(Rounding::same(14.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(if ok { "✓" } else { "!" })
                                .size(14.0)
                                .color(accent.gamma_multiply(alpha)),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(message)
                                .size(14.0)
                                .color(Theme::TEXT.gamma_multiply(alpha)),
                        );
                    });
                });
        });
}

pub struct FooterActions {
    pub apply: bool,
    pub cancel: bool,
}

/// Settings footer: version (left), Cancel + Apply (right, Windows property-sheet order).
pub fn settings_footer(
    ui: &mut Ui,
    texts: &UiText,
    version: &str,
    has_unsaved: bool,
) -> FooterActions {
    let mut actions = FooterActions {
        apply: false,
        cancel: false,
    };

    ui.set_min_height(Theme::FOOTER_BODY_H);

    ui.horizontal(|ui| {
        ui.hyperlink_to(
            egui::RichText::new(format!("MixMixer v{version}"))
                .size(12.0)
                .color(Theme::ACCENT),
            env!("CARGO_PKG_REPOSITORY"),
        );
        if has_unsaved {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(texts.footer_unsaved)
                    .size(12.0)
                    .color(Theme::WARNING),
            );
        }

        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            ui.set_width(ui.available_width());

            let apply_clicked = ui
                .add_enabled(has_unsaved, |ui: &mut Ui| {
                    ui.add(
                        egui::Button::new(
                            egui::RichText::new(texts.btn_apply)
                                .size(15.0)
                                .color(Color32::WHITE),
                        )
                        .fill(if has_unsaved {
                            Theme::ACCENT
                        } else {
                            Theme::BTN_SECONDARY
                        })
                        .stroke(Stroke::NONE)
                        .rounding(Rounding::same(8.0))
                        .min_size(vec2(96.0, 36.0)),
                    )
                })
                .clicked();
            if apply_clicked {
                actions.apply = true;
            }

            if btn_secondary(ui, texts.btn_cancel) {
                actions.cancel = true;
            }
        });
    });

    actions
}

pub fn btn_primary(ui: &mut Ui, label: &str) -> bool {
    ui.add(
        egui::Button::new(egui::RichText::new(label).size(15.0).color(Color32::WHITE))
            .fill(Theme::ACCENT)
            .stroke(Stroke::NONE)
            .rounding(Rounding::same(8.0))
            .min_size(vec2(96.0, 36.0)),
    )
    .clicked()
}

pub fn btn_secondary(ui: &mut Ui, label: &str) -> bool {
    ui.add(
        egui::Button::new(egui::RichText::new(label).size(15.0).color(Theme::TEXT))
            .fill(Theme::BTN_SECONDARY)
            .stroke(Stroke::NONE)
            .rounding(Rounding::same(8.0))
            .min_size(vec2(96.0, 36.0)),
    )
    .clicked()
}

pub fn btn_text(ui: &mut Ui, label: &str) -> bool {
    ui.add(
        egui::Button::new(egui::RichText::new(label).size(15.0).color(Theme::ACCENT))
            .fill(Color32::TRANSPARENT)
            .stroke(Stroke::NONE)
            .min_size(vec2(64.0, 36.0)),
    )
    .clicked()
}

pub fn panel_frame() -> Frame {
    Frame::none()
        .fill(Theme::BG)
        .inner_margin(Margin::symmetric(Theme::INSET, 0.0))
}

pub fn footer_frame() -> Frame {
    Frame::none()
        .fill(Theme::BG)
        .inner_margin(Margin::symmetric(Theme::INSET, 12.0))
}

pub fn header_frame() -> Frame {
    Frame::none()
        .fill(Theme::BG)
        .inner_margin(Margin::symmetric(Theme::INSET, 10.0))
}

fn truncate_end(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let truncated: String = text.chars().take(max_chars.saturating_sub(1)).collect();
    format!("{truncated}…")
}
