#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod composer;
mod controls;
mod logo;
mod shell;
mod theme;
mod viewport;

use controls::Action;
use eframe::egui::{
    self, vec2, Color32, CursorIcon, FontData, FontDefinitions, FontFamily, Key, PointerButton,
    Rect, Sense, Stroke, StrokeKind,
};
use viewport::Viewport;

fn main() -> eframe::Result {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png"))
        .expect("bundled canvas icon");
    let logo = logo::SidebarLogo::new(&icon);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([1264.0, 848.0])
            .with_icon(icon)
            .with_min_inner_size([540.0, 420.0]),
        ..Default::default()
    };
    eframe::run_native(
        "GMCanvas",
        options,
        Box::new(move |cc| {
            cc.egui_ctx.options_mut(|o| o.zoom_with_keyboard = false);
            load_ui_font(&cc.egui_ctx);
            Ok(Box::new(CanvasApp {
                logo,
                home_open: false,
                view: Viewport::default(),
                hand: false,
                composer: composer::Composer::default(),
            }))
        }),
    )
}

fn load_ui_font(ctx: &egui::Context) {
    let windows_dir = std::env::var_os("WINDIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_default();
    let candidates = [
        windows_dir.join("Fonts/msyh.ttc"),
        "/System/Library/Fonts/PingFang.ttc".into(),
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc".into(),
    ];
    if let Some(data) = candidates.iter().find_map(|path| std::fs::read(path).ok()) {
        let mut fonts = FontDefinitions::default();
        fonts
            .font_data
            .insert("cjk".into(), FontData::from_owned(data).into());
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .push("cjk".into());
        ctx.set_fonts(fonts);
    }
}

struct CanvasApp {
    logo: logo::SidebarLogo,
    home_open: bool,
    view: Viewport,
    hand: bool,
    composer: composer::Composer,
}

impl CanvasApp {
    fn apply(&mut self, action: Action, rect: Rect) {
        match action {
            Action::ZoomIn => self.view.zoom_at(1.25, rect.center(), rect),
            Action::ZoomOut => self.view.zoom_at(0.8, rect.center(), rect),
            Action::ResetZoom => self
                .view
                .zoom_at(1.0 / self.view.zoom(), rect.center(), rect),
            Action::Select => self.hand = false,
            Action::Hand => self.hand = true,
            Action::Home => self.view = Viewport::default(),
        }
    }

    fn draw_canvas(&mut self, ui: &mut egui::Ui) {
        let rect = ui.max_rect();
        let canvas = ui.allocate_rect(rect, Sense::drag());
        if canvas.drag_started() {
            canvas.request_focus();
        }
        if canvas.has_focus() || ui.memory(|m| m.focused().is_none()) {
            let delta = ui.input(|i| {
                let horizontal =
                    i.num_presses(Key::ArrowLeft) as f32 - i.num_presses(Key::ArrowRight) as f32;
                let vertical =
                    i.num_presses(Key::ArrowUp) as f32 - i.num_presses(Key::ArrowDown) as f32;
                vec2(horizontal, vertical) * 48.0
            });
            self.view.pan(delta);
        }
        let (space, scroll, zoom, pointer, origin) = ui.input(|i| {
            (
                i.key_down(Key::Space),
                i.smooth_scroll_delta,
                i.zoom_delta(),
                i.pointer.interact_pos(),
                i.pointer.press_origin(),
            )
        });
        let hand = self.hand || space;
        let moving = canvas.dragged_by(PointerButton::Middle)
            || (hand && canvas.dragged_by(PointerButton::Primary));
        if moving {
            let delta = if canvas.drag_started() {
                pointer
                    .zip(origin)
                    .map_or(egui::Vec2::ZERO, |(now, start)| now - start)
            } else {
                canvas.drag_delta()
            };
            self.view.pan(delta);
        }
        if let Some(anchor) = pointer.filter(|_| canvas.hovered()) {
            self.view.pan(scroll);
            self.view.zoom_at(f64::from(zoom), anchor, rect);
        }
        if canvas.hovered() || moving {
            ui.ctx().set_cursor_icon(if moving {
                CursorIcon::Grabbing
            } else if hand {
                CursorIcon::Grab
            } else {
                CursorIcon::Default
            });
        }
        self.view.paint(ui.painter(), rect);
        if !moving && canvas.dragged_by(PointerButton::Primary) {
            if let (Some(start), Some(end)) = (origin, pointer) {
                let selection = Rect::from_two_pos(start, end);
                ui.painter().rect(
                    selection,
                    2,
                    Color32::from_rgba_unmultiplied(170, 185, 210, 22),
                    Stroke::new(1.0_f32, Color32::from_gray(130)),
                    StrokeKind::Inside,
                );
            }
        }
        let composer_top = self.composer.show(ui.ctx(), rect);
        if let Some(action) =
            controls::show(ui.ctx(), rect, self.view.zoom(), self.hand, composer_top)
        {
            self.apply(action, rect);
            ui.ctx().request_repaint();
        }
    }
}

impl eframe::App for CanvasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let board = shell::workspace(ui, self.logo.texture(ctx), &mut self.home_open);
                if self.home_open {
                    if shell::home(ui, board).clicked() {
                        self.home_open = false;
                        ctx.request_repaint();
                    }
                    return;
                }
                let mut board_ui = ui.new_child(egui::UiBuilder::new().max_rect(board));
                board_ui.set_clip_rect(board);
                self.draw_canvas(&mut board_ui);
            });
    }
}
