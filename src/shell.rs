use eframe::egui::{
    self, pos2, vec2, Align2, Color32, CornerRadius, FontId, Painter, Pos2, Rect, Sense, Shape,
    Stroke, StrokeKind, Ui, ViewportCommand,
};

const BASE: Color32 = Color32::from_gray(20);
const BAR: Color32 = Color32::from_gray(29);
const MUTED: Color32 = Color32::from_gray(139);
const TEXT: Color32 = Color32::from_gray(232);

pub fn workspace(ui: &mut Ui, logo: egui::TextureId, home_open: &mut bool) -> Rect {
    let window = ui.max_rect();
    let p = ui.painter();
    let left = window.left();
    let top = window.top();
    let right = window.right();
    let bottom = window.bottom();
    p.rect_filled(window, 0, BASE);
    let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    let title = Rect::from_min_max(window.min, pos2(right - 144.0, top + 48.0));
    let drag = ui.interact(title, ui.id().with("title_bar"), Sense::click_and_drag());
    if drag.double_clicked() {
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!maximized));
    } else if drag.drag_started_by(egui::PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }
    let buttons = [
        (Icon::Minimize, "最小化", ViewportCommand::Minimized(true)),
        (
            if maximized {
                Icon::Restore
            } else {
                Icon::Maximize
            },
            if maximized { "还原" } else { "最大化" },
            ViewportCommand::Maximized(!maximized),
        ),
        (Icon::Close, "关闭", ViewportCommand::Close),
    ];
    for (index, (glyph, name, command)) in buttons.into_iter().enumerate() {
        let center = pos2(right - 115.0 + 46.0 * index as f32, top + 24.0);
        let hit = Rect::from_center_size(center, vec2(46.0, 44.0));
        let button = ui.interact(hit, ui.id().with(("window", index)), Sense::click());
        button.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, name));
        if button.hovered() || button.has_focus() {
            let color = if index == 2 {
                Color32::from_rgb(168, 47, 54)
            } else {
                BAR
            };
            p.rect_filled(hit.shrink(2.0), 5, color);
        }
        icon(p, center, glyph, Color32::from_gray(185));
        if button.on_hover_text(name).clicked() {
            ui.ctx().send_viewport_cmd(command);
        }
    }

    let home_center = pos2(left + 28.0, top + 72.0);
    let hit = Rect::from_center_size(home_center, vec2(42.0, 42.0));
    let button = ui.interact(hit, ui.id().with("home"), Sense::click());
    button.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "回到主页"));
    if button.hovered() || *home_open {
        p.rect_filled(hit, 9, BAR);
    }
    if button.has_focus() {
        p.rect_stroke(hit, 9, Stroke::new(1.0_f32, MUTED), StrokeKind::Inside);
    }
    p.image(
        logo,
        Rect::from_center_size(home_center, vec2(34.0, 34.0)),
        Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    if button
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text("回到主页")
        .clicked()
    {
        *home_open = true;
    }

    let profile = pos2(left + 28.0, bottom - 32.0);
    p.circle_filled(profile, 14.0, Color32::from_gray(35));
    icon(p, profile, Icon::User, MUTED);

    let panel = Rect::from_min_max(
        pos2(left + 56.0, top + 48.0),
        pos2(right - 8.0, bottom - 8.0),
    );
    let header = Rect::from_min_max(panel.min, pos2(panel.right(), panel.top() + 36.0));
    p.rect_filled(
        header,
        CornerRadius {
            nw: 12,
            ne: 12,
            sw: 0,
            se: 0,
        },
        BAR,
    );
    let tab = header.left_center();
    if *home_open {
        p.text(
            tab + vec2(16.0, 0.0),
            Align2::LEFT_CENTER,
            "主页",
            FontId::proportional(12.0),
            TEXT,
        );
    } else {
        icon(p, tab + vec2(20.0, 0.0), Icon::Board, TEXT);
        p.text(
            tab + vec2(33.0, 0.0),
            Align2::LEFT_CENTER,
            "画布",
            FontId::proportional(12.0),
            TEXT,
        );
        icon(p, tab + vec2(98.0, 0.0), Icon::Add, MUTED);
        icon(
            p,
            header.right_center() - vec2(19.0, 0.0),
            Icon::Sliders,
            MUTED,
        );
    }
    if !maximized {
        resize_edges(ui, window);
    }
    Rect::from_min_max(header.left_bottom(), panel.max)
}

pub fn home(ui: &mut Ui, rect: Rect) -> egui::Response {
    ui.painter().rect_filled(
        rect,
        CornerRadius {
            nw: 0,
            ne: 0,
            sw: 9,
            se: 9,
        },
        Color32::from_gray(24),
    );
    ui.put(
        Rect::from_center_size(rect.center(), vec2(144.0, 40.0)),
        egui::Button::new("继续画布").corner_radius(8),
    )
    .on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn resize_edges(ui: &Ui, window: Rect) {
    use egui::CursorIcon::{ResizeHorizontal, ResizeNeSw, ResizeNwSe, ResizeVertical};
    use egui::ResizeDirection::*;

    let inner = window.shrink(5.0);
    let xs = [window.left(), inner.left(), inner.right(), window.right()];
    let ys = [window.top(), inner.top(), inner.bottom(), window.bottom()];
    let primary_pressed = ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Primary));
    for (col, row, direction, cursor) in [
        (1, 0, North, ResizeVertical),
        (1, 2, South, ResizeVertical),
        (0, 1, West, ResizeHorizontal),
        (2, 1, East, ResizeHorizontal),
        (0, 0, NorthWest, ResizeNwSe),
        (2, 0, NorthEast, ResizeNeSw),
        (0, 2, SouthWest, ResizeNeSw),
        (2, 2, SouthEast, ResizeNwSe),
    ] {
        let hit = Rect::from_min_max(pos2(xs[col], ys[row]), pos2(xs[col + 1], ys[row + 1]));
        let response = ui
            .interact(
                hit,
                ui.id().with(("window_resize", col, row)),
                Sense::click_and_drag() - Sense::focusable_noninteractive(),
            )
            .on_hover_and_drag_cursor(cursor);
        if primary_pressed && response.is_pointer_button_down_on() {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::BeginResize(direction));
        }
    }
}

#[derive(Clone, Copy)]
enum Icon {
    Minimize,
    Maximize,
    Restore,
    Close,
    Sliders,
    User,
    Board,
    Add,
}

fn icon(p: &Painter, at: Pos2, glyph: Icon, ink: Color32) {
    let pen = Stroke::new(1.1_f32, ink);
    let pt = |x, y| at + vec2(x, y);
    let path = |points: &[(f32, f32)]| {
        p.add(Shape::line(
            points.iter().map(|&(x, y)| pt(x, y)).collect(),
            pen,
        ));
    };
    let ring = |x, y, r| {
        p.circle_stroke(pt(x, y), r, pen);
    };
    let box_at = |x, y, w, h, radius| {
        p.rect_stroke(
            Rect::from_center_size(pt(x, y), vec2(w, h)),
            radius,
            pen,
            StrokeKind::Inside,
        );
    };
    match glyph {
        Icon::Minimize => path(&[(-4.5, 0.5), (4.5, 0.5)]),
        Icon::Maximize => box_at(0.0, 0.0, 10.0, 10.0, 0),
        Icon::Restore => {
            box_at(-1.0, 1.0, 8.0, 8.0, 0);
            path(&[
                (-2.5, -3.0),
                (-2.5, -5.0),
                (5.0, -5.0),
                (5.0, 2.5),
                (3.0, 2.5),
            ]);
        }
        Icon::Close => {
            path(&[(-4.0, -4.0), (4.0, 4.0)]);
            path(&[(-4.0, 4.0), (4.0, -4.0)]);
        }
        Icon::Sliders => {
            for (y, x) in [(-3.4, 2.0), (3.0, -2.2)] {
                path(&[(-6.5, y), (x - 2.0, y)]);
                ring(x, y, 1.8);
                path(&[(x + 2.0, y), (6.5, y)]);
            }
        }
        Icon::User => {
            ring(0.0, -3.5, 2.5);
            box_at(0.0, 3.7, 9.0, 4.8, 2);
        }
        Icon::Board => {
            box_at(0.0, 0.0, 12.0, 10.0, 3);
            path(&[(-4.0, 1.0), (-2.0, -1.0), (1.0, 2.0), (3.5, 0.0)]);
        }
        Icon::Add => {
            path(&[(-4.5, 0.0), (4.5, 0.0)]);
            path(&[(0.0, -4.5), (0.0, 4.5)]);
        }
    }
}
