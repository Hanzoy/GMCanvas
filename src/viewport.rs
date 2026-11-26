use crate::theme::Palette;
use eframe::egui::{
    emath::GuiRounding, epaint::Mesh, pos2, CornerRadius, Painter, Pos2, Rect, Vec2,
};

const GRID_STEP: f64 = 24.0;

pub struct Viewport {
    center: [f64; 2],
    zoom: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            zoom: 1.0,
        }
    }
}

impl Viewport {
    pub fn zoom(&self) -> f64 {
        self.zoom
    }

    pub fn pan(&mut self, delta: Vec2) {
        self.center[0] -= f64::from(delta.x) / self.zoom;
        self.center[1] -= f64::from(delta.y) / self.zoom;
    }

    pub fn zoom_at(&mut self, factor: f64, anchor: Pos2, rect: Rect) {
        let next = (self.zoom * factor).clamp(0.1, 2.0);
        let offset = anchor - rect.center();
        // Keep the world position under the anchor fixed as the scale changes.
        let shift = self.zoom.recip() - next.recip();
        self.center[0] += f64::from(offset.x) * shift;
        self.center[1] += f64::from(offset.y) * shift;
        self.zoom = next;
    }

    pub fn paint(&self, painter: &Painter, rect: Rect) {
        let painter = painter.with_clip_rect(rect);
        let palette = Palette::from_visuals(&painter.ctx().style().visuals);
        painter.rect_filled(
            rect,
            CornerRadius {
                nw: 0,
                ne: 0,
                sw: 9,
                se: 9,
            },
            palette.canvas,
        );

        let origin_x = f64::from(rect.center().x) - self.center[0] * self.zoom;
        let origin_y = f64::from(rect.center().y) - self.center[1] * self.zoom;
        let mut spacing = GRID_STEP * self.zoom;
        // At low zoom draw every second (then fourth, etc.) world grid point.
        while spacing < 14.0 {
            spacing *= 2.0;
        }

        let left = f64::from(rect.left());
        let top = f64::from(rect.top());
        let first_x = left + (origin_x - left).rem_euclid(spacing);
        let pixels_per_point = painter.pixels_per_point();
        let mut y = top + (origin_y - top).rem_euclid(spacing);
        let mut dots = Mesh::default();
        while y <= f64::from(rect.bottom()) {
            let mut x = first_x;
            while x <= f64::from(rect.right()) {
                let pixel = pos2(x as f32, y as f32).round_to_pixels(pixels_per_point);
                let dot = Rect::from_min_size(pixel, Vec2::splat(1.0));
                dots.add_colored_rect(dot, palette.grid);
                x += spacing;
            }
            y += spacing;
        }
        painter.add(dots);
    }
}
