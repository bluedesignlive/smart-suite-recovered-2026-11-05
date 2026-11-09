//! Design tokens (from smart.toml) as egui colours.
use egui::Color32;

pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(r, g, b, (a.clamp(0.0, 1.0) * 255.0).round() as u8)
}
pub fn hex(h: u32) -> Color32 {
    Color32::from_rgb((h >> 16) as u8, (h >> 8) as u8, h as u8)
}
pub fn with_a(c: Color32, a: f32) -> Color32 {
    let [r, g, b, _] = c.to_srgba_unmultiplied();
    rgba(r, g, b, a)
}

#[derive(Clone, Copy)]
pub struct Pal {
    pub dark: bool,
    pub glass: bool,
    pub ink1: Color32,
    pub ink2: Color32,
    pub ink3: Color32,
    pub ink4: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub accent_press: Color32,
    pub accent_ink: Color32,
    pub accent_tint: Color32,
    pub danger: Color32,
    pub good: Color32,
    pub warn: Color32,
    pub win_top: Color32,
    pub win_bottom: Color32,
    pub side_top: Color32,
    pub side_bottom: Color32,
    pub content: Color32,
    pub control: Color32,
    pub control_hover: Color32,
    pub panel_top: Color32,
    pub panel_bottom: Color32,
    pub menu: Color32,
    pub scrim: Color32,
}

impl Pal {
    pub fn new(dark: bool, glass: bool) -> Pal {
        if !dark {
            Pal {
                dark, glass,
                ink1: hex(0x141d30), ink2: hex(0x3f4b60), ink3: hex(0x6b7789), ink4: hex(0x97a1b2),
                accent: hex(0x2f6feb), accent_hover: hex(0x3d7bf7), accent_press: hex(0x2560d4), accent_ink: hex(0x1c56c4),
                accent_tint: rgba(47, 111, 235, 0.105), danger: hex(0xd34450), good: hex(0x2b9b6d), warn: hex(0xc9861a),
                win_top: if glass { rgba(255, 255, 255, 0.845) } else { hex(0xfdfeff) },
                win_bottom: if glass { rgba(242, 247, 253, 0.795) } else { hex(0xf1f6fc) },
                side_top: if glass { rgba(222, 233, 248, 0.74) } else { hex(0xdfe9f8) },
                side_bottom: if glass { rgba(230, 239, 250, 0.60) } else { hex(0xe6eef9) },
                content: rgba(255, 255, 255, 0.30),
                control: rgba(255, 255, 255, 0.60), control_hover: rgba(255, 255, 255, 0.86),
                panel_top: rgba(252, 253, 255, 0.985), panel_bottom: rgba(246, 249, 255, 0.975),
                menu: rgba(250, 252, 255, 0.985), scrim: rgba(20, 30, 48, 0.30),
            }
        } else {
            Pal {
                dark, glass,
                ink1: hex(0xeef3fb), ink2: hex(0xc6cfdf), ink3: hex(0x96a1b5), ink4: hex(0x707b90),
                accent: hex(0x6f9bff), accent_hover: hex(0x86acff), accent_press: hex(0x5b87f0), accent_ink: hex(0xa7c1ff),
                accent_tint: rgba(111, 155, 255, 0.155), danger: hex(0xff7a84), good: hex(0x59c79a), warn: hex(0xe5b05a),
                win_top: if glass { rgba(20, 27, 40, 0.86) } else { hex(0x141b28) },
                win_bottom: if glass { rgba(13, 19, 30, 0.88) } else { hex(0x0d131e) },
                side_top: if glass { rgba(13, 19, 30, 0.72) } else { hex(0x0d131e) },
                side_bottom: if glass { rgba(10, 15, 25, 0.62) } else { hex(0x0a0f19) },
                content: rgba(255, 255, 255, 0.022),
                control: rgba(255, 255, 255, 0.072), control_hover: rgba(255, 255, 255, 0.115),
                panel_top: rgba(24, 32, 47, 0.985), panel_bottom: rgba(15, 22, 34, 0.99),
                menu: rgba(27, 35, 50, 0.98), scrim: rgba(3, 7, 14, 0.56),
            }
        }
    }
    pub fn kind_color(&self, k: smart_core::Kind) -> Color32 {
        use smart_core::Kind::*;
        match k {
            Folder => self.accent,
            Image => hex(0x2f9bd3),
            Audio => hex(0xd0742f),
            Video => hex(0x8a5cd6),
            Document => hex(0xd34450),
            Text => hex(0x6a86b4),
            Code => hex(0x2b9b6d),
            Archive => hex(0xc9861a),
            App => hex(0x4b63d9),
            Data => hex(0x5f8f8f),
            Other => hex(0x8792a6),
        }
    }
}
