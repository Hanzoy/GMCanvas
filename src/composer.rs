use crate::theme::Palette;
use eframe::egui::{
    self, pos2, vec2, Align2, Area, Color32, CursorIcon, FontId, Id, Key, Order, Rect, RichText,
    ScrollArea, Sense, Shape, Stroke, StrokeKind, TextEdit, Ui, UiBuilder, WidgetInfo, WidgetType,
};

const OPEN_PROMPT: &str = "让 agent 做点什么";

#[derive(Default)]
pub struct Composer {
    expanded: bool,
    focus_draft: bool,
    draft: String,
}

impl Composer {
    pub fn show(&mut self, ctx: &egui::Context, board: Rect) -> f32 {
        let editor = Id::new("agent_draft");
        if self.expanded && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, Key::Escape)) {
            self.expanded = false;
            ctx.memory_mut(|m| m.surrender_focus(editor));
        }
        let (area_name, size) = if self.expanded {
            (
                "agent_editor",
                vec2((board.width() - 24.0).min(640.0), 166.0),
            )
        } else {
            ("agent_pill", vec2(160.0, 38.0))
        };
        let top_left = pos2(
            board.center().x - size.x / 2.0,
            board.bottom() - 14.0 - size.y,
        );
        Area::new(area_name.into())
            .order(Order::Foreground)
            .fixed_pos(top_left)
            .movable(false)
            .default_size(size)
            .sense(Sense::hover())
            .show(ctx, |ui| {
                let (bounds, _) = ui.allocate_exact_size(size, Sense::hover());
                let palette = Palette::from_visuals(ui.visuals());
                let shadow_alpha = if ui.visuals().dark_mode { 72 } else { 28 };
                let radius = if self.expanded { 14 } else { 19 };
                let shadow = egui::epaint::Shadow {
                    offset: [0, 5],
                    blur: 20,
                    spread: 0,
                    color: Color32::from_black_alpha(shadow_alpha),
                };
                ui.painter().add(shadow.as_shape(bounds, radius));
                if self.expanded {
                    self.edit(ui, bounds, editor, palette);
                } else if pill(ui, bounds, palette) {
                    self.expanded = true;
                    self.focus_draft = true;
                    ctx.request_repaint();
                }
            });
        top_left.y
    }

    fn edit(&mut self, ui: &mut Ui, bounds: Rect, editor: Id, palette: Palette) {
        ui.painter().rect_filled(bounds, 14, palette.surface);
        let handle = Rect::from_center_size(
            pos2(bounds.center().x, bounds.top() + 12.0),
            vec2(80.0, 24.0),
        );
        let collapse = ui.interact(handle, ui.id().with("collapse"), Sense::click());
        collapse.widget_info(|| WidgetInfo::labeled(WidgetType::Button, true, "收起输入框"));
        let handle_ink = if collapse.hovered() || collapse.has_focus() {
            palette.muted
        } else {
            palette.border.lerp_to_gamma(palette.muted, 0.2)
        };
        ui.painter().rect_filled(
            Rect::from_center_size(handle.center(), vec2(32.0, 4.0)),
            2,
            handle_ink,
        );
        if collapse
            .on_hover_cursor(CursorIcon::PointingHand)
            .on_hover_text("收起输入框（Esc）")
            .clicked()
        {
            self.expanded = false;
            ui.memory_mut(|m| m.surrender_focus(editor));
            ui.ctx().request_repaint();
        }

        let field =
            Rect::from_min_max(bounds.min + vec2(11.0, 31.0), bounds.max - vec2(11.0, 36.0));
        ui.painter().rect_stroke(
            field,
            14,
            Stroke::new(1.0_f32, palette.border),
            StrokeKind::Inside,
        );
        let text = Rect::from_min_max(
            field.min + vec2(14.0, 13.0),
            field.right_top() + vec2(-14.0, 51.0),
        );
        ui.scope_builder(UiBuilder::new().max_rect(text), |ui| {
            ui.set_clip_rect(text);
            if self.focus_draft && ui.is_visible() && !ui.is_sizing_pass() {
                ui.memory_mut(|m| m.request_focus(editor));
                self.focus_draft = false;
            }
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add(
                        TextEdit::multiline(&mut self.draft)
                            .id(editor)
                            .frame(false)
                            .margin(0)
                            .desired_rows(2)
                            .desired_width(ui.available_width())
                            .font(FontId::proportional(13.0))
                            .text_color(palette.text)
                            .hint_text(RichText::new("说说想改什么").color(palette.muted)),
                    );
                });
        });
        draw_footer(ui, bounds, field, palette);
    }
}

fn pill(ui: &mut Ui, rect: Rect, palette: Palette) -> bool {
    let button = ui.interact(rect, ui.id().with("expand"), Sense::click());
    button.widget_info(|| WidgetInfo::labeled(WidgetType::Button, true, OPEN_PROMPT));
    let fill = if button.hovered() {
        palette.hover
    } else {
        palette.surface
    };
    let painter = ui.painter();
    painter.rect_filled(rect, 19, fill);
    if button.has_focus() {
        painter.rect_stroke(
            rect,
            19,
            Stroke::new(1.0_f32, palette.focus),
            StrokeKind::Inside,
        );
    }
    let icon = rect.left_center() + vec2(24.0, 0.0);
    let stroke = Stroke::new(1.0_f32, palette.secondary);
    painter.rect_stroke(
        Rect::from_center_size(icon, vec2(13.0, 12.0)),
        5,
        stroke,
        StrokeKind::Inside,
    );
    for (y, width) in [(-1.5, 6.0), (1.5, 4.0)] {
        let start = icon + vec2(-3.0, y);
        painter.line_segment([start, start + vec2(width, 0.0)], stroke);
    }
    painter.text(
        rect.left_center() + vec2(39.0, -0.5),
        Align2::LEFT_CENTER,
        OPEN_PROMPT,
        FontId::proportional(13.0),
        palette.text,
    );
    button.on_hover_cursor(CursorIcon::PointingHand).clicked()
}

fn draw_footer(ui: &mut Ui, bounds: Rect, field: Rect, palette: Palette) {
    let painter = ui.painter();
    let stroke = Stroke::new(1.2_f32, palette.muted);
    let row_y = field.bottom() - 29.0;
    let add = pos2(field.left() + 29.0, row_y);
    painter.line_segment([add - vec2(5.5, 0.0), add + vec2(5.5, 0.0)], stroke);
    painter.line_segment([add - vec2(0.0, 5.5), add + vec2(0.0, 5.5)], stroke);
    let send = pos2(field.right() - 28.0, row_y);
    painter.circle_filled(
        send,
        15.0,
        palette.surface.lerp_to_gamma(palette.muted, 0.7),
    );
    let arrow = Stroke::new(1.2_f32, palette.surface);
    painter.line_segment([send + vec2(0.0, 4.0), send - vec2(0.0, 4.0)], arrow);
    painter.add(Shape::line(
        vec![
            send + vec2(-3.0, -1.0),
            send - vec2(0.0, 4.0),
            send + vec2(3.0, -1.0),
        ],
        arrow,
    ));
    let chevron = send - vec2(35.0, 0.0);
    painter.add(Shape::line(
        vec![
            chevron - vec2(4.0, 2.0),
            chevron + vec2(0.0, 2.0),
            chevron + vec2(4.0, -2.0),
        ],
        stroke,
    ));
    painter.text(
        chevron - vec2(12.0, 0.0),
        Align2::RIGHT_CENTER,
        "未连接模型",
        FontId::proportional(12.0),
        palette.muted,
    );
    let quota = pos2(bounds.center().x - 13.0, bounds.bottom() - 20.0);
    painter.circle_stroke(
        quota,
        5.2,
        Stroke::new(1.6_f32, palette.border.lerp_to_gamma(palette.muted, 0.3)),
    );
    painter.text(
        quota + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        "额度",
        FontId::proportional(11.0),
        palette.muted,
    );
    ui.interact(
        Rect::from_center_size(add, vec2(30.0, 30.0)),
        ui.id().with("attachments"),
        Sense::hover(),
    )
    .on_hover_text("附件功能尚未接入");
    ui.interact(
        Rect::from_min_max(chevron - vec2(88.0, 16.0), send + vec2(16.0, 16.0)),
        ui.id().with("model"),
        Sense::hover(),
    )
    .on_hover_text("尚未连接模型，草稿仅保留在当前窗口");
    ui.interact(
        Rect::from_center_size(quota + vec2(13.0, 0.0), vec2(54.0, 22.0)),
        ui.id().with("quota"),
        Sense::hover(),
    )
    .on_hover_text("连接模型后可查看额度");
}
