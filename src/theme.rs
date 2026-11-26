use eframe::egui::{Color32, Visuals};

#[derive(Clone, Copy)]
pub struct Palette {
    pub app: Color32,
    pub bar: Color32,
    pub canvas: Color32,
    pub grid: Color32,
    pub surface: Color32,
    pub hover: Color32,
    pub selected: Color32,
    pub border: Color32,
    pub text: Color32,
    pub secondary: Color32,
    pub muted: Color32,
    pub focus: Color32,
}

impl Palette {
    pub fn from_visuals(visuals: &Visuals) -> Self {
        let gray = |dark, light| Color32::from_gray(if visuals.dark_mode { dark } else { light });
        Self {
            app: gray(20, 243),
            bar: gray(29, 250),
            canvas: gray(24, 247),
            grid: gray(51, 208),
            surface: gray(36, 255),
            hover: gray(42, 233),
            selected: gray(48, 222),
            border: gray(52, 211),
            text: gray(236, 30),
            secondary: gray(175, 76),
            muted: gray(139, 102),
            focus: gray(185, 81),
        }
    }
}
