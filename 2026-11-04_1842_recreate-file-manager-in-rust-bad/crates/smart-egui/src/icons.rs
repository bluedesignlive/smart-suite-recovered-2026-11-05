//! Icon glyph mapping (Phosphor: regular by default, fill when active) and font installation.
use egui::{Context, FontData, FontDefinitions, FontFamily};
use smart_core::Icon;
use std::sync::Arc;

macro_rules! icons {
    ($($v:ident => $n:ident),* $(,)?) => {
        pub fn glyph(i: Icon, fill: bool) -> &'static str {
            match i { $(Icon::$v => if fill { egui_phosphor::fill::$n } else { egui_phosphor::regular::$n }),* }
        }
    };
}
icons! {
    Home => HOUSE, Clock => CLOCK, Monitor => MONITOR, Doc => FILE_TEXT, Download => DOWNLOAD_SIMPLE, Image => IMAGE,
    Music => MUSIC_NOTES, Film => FILM_STRIP, Drive => HARD_DRIVE, Globe => GLOBE, Cloud => CLOUD, Folder => FOLDER,
    FolderOpen => FOLDER_OPEN, File => FILE, Zip => FILE_ZIP, Pdf => FILE_PDF, Code => FILE_CODE, Search => MAGNIFYING_GLASS,
    Sparkle => SPARKLE, Grid => SQUARES_FOUR, List => LIST_BULLETS, Columns => COLUMNS, Sort => SORT_ASCENDING, More => DOTS_THREE,
    Back => CARET_LEFT, Forward => CARET_RIGHT, Up => ARROW_UP, ChevronDown => CARET_DOWN, ChevronRight => CARET_RIGHT,
    Sidebar => SIDEBAR_SIMPLE, Copy => COPY, Cut => SCISSORS, Paste => CLIPBOARD, Trash => TRASH, Rename => PENCIL_SIMPLE,
    Share => SHARE_NETWORK, Tag => TAG, Archive => ARCHIVE, Eye => EYE, Info => INFO, Plus => PLUS, Close => X, Check => CHECK,
    CheckCircle => CHECK_CIRCLE, Refresh => ARROW_CLOCKWISE, NewFolder => FOLDER_PLUS, NewFile => FILE_PLUS,
    Terminal => TERMINAL_WINDOW, Play => PLAY, Pause => PAUSE, Stop => STOP, Eject => EJECT, Usb => USB, Phone => DEVICE_MOBILE,
    Broom => BROOM, Pie => CHART_PIE_SLICE, Command => COMMAND, Link => LINK, Move => ARROWS_OUT_CARDINAL, Star => STAR,
    Lock => LOCK, Warning => WARNING, Send => PAPER_PLANE_TILT, Moon => MOON, Sun => SUN, Gear => GEAR, Keyboard => KEYBOARD,
    Upload => UPLOAD_SIMPLE, Wand => MAGIC_WAND, Undo => ARROW_COUNTER_CLOCKWISE, Restore => CLOCK_COUNTER_CLOCKWISE,
    EyeOff => EYE_SLASH, SelectAll => SELECTION_ALL, App => APP_WINDOW, OpenExt => ARROW_SQUARE_OUT, Duplicate => COPY_SIMPLE,
    Wave => WAVEFORM, Cube => CUBE, Rocket => ROCKET_LAUNCH, Layers => STACK, Sliders => SLIDERS_HORIZONTAL, Hash => HASH,
    Bolt => LIGHTNING, Plug => PLUGS, ArrowDown => ARROW_DOWN, ArrowUp => ARROW_UP,
}

pub fn medium() -> FontFamily {
    FontFamily::Name("medium".into())
}
pub fn semi() -> FontFamily {
    FontFamily::Name("semi".into())
}
pub fn icons_family(fill: bool) -> FontFamily {
    FontFamily::Name(if fill { "icons-fill" } else { "icons" }.into())
}

pub fn install_fonts(ctx: &Context) {
    let mut f = FontDefinitions::default();
    let add = |f: &mut FontDefinitions, name: &str, data: FontData| {
        f.font_data.insert(name.into(), Arc::new(data));
    };
    add(&mut f, "inter", FontData::from_static(include_bytes!("../../../assets/fonts/Inter-Regular.ttf")));
    add(&mut f, "inter-medium", FontData::from_static(include_bytes!("../../../assets/fonts/Inter-Medium.ttf")));
    add(&mut f, "inter-semi", FontData::from_static(include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf")));
    f.font_data.insert("phosphor".into(), egui_phosphor::Variant::Regular.font_data().into());
    f.font_data.insert("phosphor-fill".into(), egui_phosphor::Variant::Fill.font_data().into());
    let fallback: Vec<String> = f.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    let mut prop = vec!["inter".to_string()];
    prop.extend(fallback.clone());
    f.families.insert(FontFamily::Proportional, prop);
    let mut m = vec!["inter-medium".to_string(), "inter".to_string()];
    m.extend(fallback.clone());
    f.families.insert(medium(), m);
    let mut s = vec!["inter-semi".to_string(), "inter".to_string()];
    s.extend(fallback);
    f.families.insert(semi(), s);
    f.families.insert(icons_family(false), vec!["phosphor".into()]);
    f.families.insert(icons_family(true), vec!["phosphor-fill".into()]);
    ctx.set_fonts(f);
}
