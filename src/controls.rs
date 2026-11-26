use eframe::egui::{
    self, pos2, vec2, Align2, Area, Color32, CursorIcon, FontId, Frame, Order, Painter, Pos2, Rect,
    Response, Sense, Shape, Stroke, StrokeKind, Ui, WidgetInfo, WidgetType,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    ZoomOut,
    ResetZoom,
    ZoomIn,
    Select,
    Hand,
    Home,
}

pub fn show(
    ctx: &egui::Context,
    rect: Rect,
    zoom: f64,
    hand: bool,
    composer_top: f32,
) -> Option<Action> {
    use Action::*;

    let mut clicked = None;
    let tools_y = composer_top - 10.0;
    let expanded = rect.bottom() - composer_top > 80.0;
    let (zoom_corner, zoom_pos) = if rect.width() < 450.0 {
        (Align2::RIGHT_TOP, rect.right_top() + vec2(-14.0, 12.0))
    } else {
        let y = if expanded && rect.width() < 950.0 {
            tools_y
        } else {
            rect.bottom() - 14.0
        };
        (Align2::LEFT_BOTTOM, pos2(rect.left() + 14.0, y))
    };
    capsule(ctx, "canvas_zoom", rect, zoom_corner, zoom_pos, |ui| {
        for action in [ZoomOut, ResetZoom, ZoomIn] {
            let enabled = match action {
                ZoomOut => zoom > 0.1,
                ZoomIn => zoom < 2.0,
                _ => true,
            };
            if ui
                .add_enabled(enabled, |ui: &mut Ui| control(ui, action, zoom, hand))
                .clicked()
            {
                clicked = Some(action);
            }
        }
    });
    capsule(
        ctx,
        "canvas_tools",
        rect,
        Align2::CENTER_BOTTOM,
        pos2(rect.center().x, tools_y),
        |ui| {
            for (label, is_asset) in [("添加节点（后续开放）", false), ("素材（后续开放）", true)]
            {
                ui.add_enabled(false, |ui: &mut Ui| {
                    let (box_rect, response) =
                        ui.allocate_exact_size(vec2(30.0, 30.0), Sense::click());
                    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, false, label));
                    let center = box_rect.center();
                    let stroke = Stroke::new(1.35_f32, Color32::from_gray(223));
                    let painter = ui.painter();
                    if is_asset {
                        painter.rect_stroke(
                            Rect::from_center_size(center, vec2(13.0, 15.0)),
                            3,
                            stroke,
                            StrokeKind::Inside,
                        );
                        painter.rect_stroke(
                            Rect::from_center_size(center - vec2(0.0, 2.8), vec2(7.3, 3.6)),
                            1,
                            stroke,
                            StrokeKind::Inside,
                        );
                        painter.line_segment(
                            [center + vec2(-4.2, 2.6), center + vec2(-1.1, 2.6)],
                            stroke,
                        );
                        painter.line_segment(
                            [center + vec2(-2.7, 1.1), center + vec2(-2.7, 4.1)],
                            stroke,
                        );
                        painter.circle_stroke(center + vec2(2.6, 2.8), 1.0, stroke);
                    } else {
                        painter.line_segment(
                            [center - vec2(6.4, 0.0), center + vec2(6.4, 0.0)],
                            stroke,
                        );
                        painter.line_segment(
                            [center - vec2(0.0, 6.4), center + vec2(0.0, 6.4)],
                            stroke,
                        );
                    }
                    response
                })
                .on_disabled_hover_text(label);
            }
            for action in [Select, Hand, Home] {
                if control(ui, action, zoom, hand).clicked() {
                    clicked = Some(action);
                }
            }
        },
    );
    clicked
}

fn capsule(
    ctx: &egui::Context,
    id: &'static str,
    rect: Rect,
    pivot: Align2,
    position: Pos2,
    buttons: impl FnOnce(&mut Ui),
) {
    Area::new(id.into())
        .order(Order::Foreground)
        .sense(Sense::hover())
        .constrain_to(rect)
        .pivot(pivot)
        .fixed_pos(position)
        .default_size(egui::Vec2::ZERO)
        .show(ctx, |ui| {
            Frame::NONE
                .inner_margin(5)
                .corner_radius(12)
                .fill(Color32::from_gray(36))
                .shadow(egui::epaint::Shadow {
                    offset: [0, 5],
                    blur: 16,
                    spread: 0,
                    color: Color32::from_black_alpha(62),
                })
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;
                    ui.horizontal(buttons);
                });
        });
}

fn control(ui: &mut Ui, action: Action, zoom: f64, hand: bool) -> Response {
    use Action::*;

    let label = match action {
        ZoomOut => "缩小",
        ResetZoom => "恢复 100% 缩放",
        ZoomIn => "放大",
        Select => "选择",
        Hand => "平移画布",
        Home => "回到原点",
    };
    let width = if action == ResetZoom { 44.0 } else { 30.0 };
    let (rect, response) = ui.allocate_exact_size(vec2(width, 30.0), Sense::click());
    let selected = (action == Select && !hand) || (action == Hand && hand);
    response.widget_info(|| match action {
        Select | Hand => WidgetInfo::selected(
            WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        ),
        _ => WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), label),
    });

    let painter = ui.painter();
    if selected || response.is_pointer_button_down_on() {
        painter.rect_filled(rect, 8, Color32::from_gray(48));
    } else if response.hovered() {
        painter.rect_filled(rect, 8, Color32::from_gray(42));
    }
    if response.has_focus() {
        painter.rect_stroke(
            rect,
            8,
            Stroke::new(1.25_f32, Color32::from_gray(185)),
            StrokeKind::Inside,
        );
    }
    let ink = if selected || response.hovered() {
        Color32::from_gray(236)
    } else {
        Color32::from_gray(163)
    };
    if action == ResetZoom {
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            format!("{:.0}%", zoom * 100.0),
            FontId::monospace(11.0),
            ink,
        );
    } else {
        draw_icon(painter, rect.center(), action, ink);
    }
    response
        .on_hover_cursor(CursorIcon::PointingHand)
        .on_hover_text(label)
}

fn draw_icon(painter: &Painter, center: Pos2, action: Action, ink: Color32) {
    use Action::*;

    let stroke = Stroke::new(1.55_f32, ink);
    let point = |x, y| center + vec2(x, y);
    let outline = |points: &[(f32, f32)]| {
        let points = points.iter().map(|&(x, y)| point(x, y)).collect();
        painter.add(Shape::closed_line(points, stroke));
    };
    match action {
        ZoomOut | ZoomIn => {
            painter.line_segment([point(-5.5, 0.0), point(5.5, 0.0)], stroke);
            if action == ZoomIn {
                painter.line_segment([point(0.0, -5.5), point(0.0, 5.5)], stroke);
            }
        }
        Select => outline(&[
            (-6.1, -5.9),
            (-5.9, -6.5),
            (-5.1, -6.5),
            (3.9, -3.3),
            (4.4, -2.4),
            (3.0, -0.1),
            (6.0, 3.3),
            (6.1, 4.1),
            (4.4, 5.8),
            (3.6, 5.9),
            (0.1, 2.2),
            (-2.3, 4.1),
            (-3.2, 3.8),
        ]),
        Hand => outline(&[
            (-3.8, 0.5),
            (-3.8, -5.6),
            (-3.3, -6.5),
            (-2.2, -6.7),
            (-1.4, -5.8),
            (-1.4, -1.5),
            (-1.4, -7.1),
            (-0.7, -8.0),
            (0.5, -8.0),
            (1.1, -7.0),
            (1.1, -1.9),
            (1.1, -6.1),
            (1.8, -6.9),
            (2.9, -6.9),
            (3.6, -6.0),
            (3.6, -1.0),
            (3.6, -4.2),
            (4.4, -5.0),
            (5.5, -4.9),
            (6.2, -3.9),
            (6.2, 2.3),
            (5.7, 4.9),
            (4.2, 6.8),
            (2.2, 7.7),
            (-0.8, 7.7),
            (-2.8, 7.1),
            (-4.2, 5.8),
            (-7.6, 1.0),
            (-7.6, 0.0),
            (-6.9, -0.8),
            (-5.8, -0.9),
        ]),
        Home => {
            for sign in [-1.0, 1.0] {
                let tip = point(7.0 * sign, -7.0 * sign);
                painter.line_segment([tip, point(2.4 * sign, -2.4 * sign)], stroke);
                painter.add(Shape::line(
                    vec![
                        tip - vec2(4.7 * sign, 0.0),
                        tip,
                        tip + vec2(0.0, 4.7 * sign),
                    ],
                    stroke,
                ));
            }
        }
        ResetZoom => {}
    }
}
