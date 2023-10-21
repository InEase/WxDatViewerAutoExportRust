#![allow(warnings, unused)]

use fltk::enums::FrameType;
use fltk::{prelude::*, *};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use FrameType::*;

pub mod img;
pub mod text;
pub mod hotspot;
pub mod variable_tag_control;
pub(crate) mod message;

pub use hotspot::{*};
pub use text::{*};
pub use img::{*};
pub use message::{*};

#[macro_export]
macro_rules! inject_fltk_theme {
    () => {
        use fltk_theme::{
            color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme,
        };

        // 设置主题
        let theme = ColorTheme::new(color_themes::BLACK_THEME);
        let widget_theme = WidgetTheme::new(ThemeType::HighContrast);
        widget_theme.apply();
        let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
        widget_scheme.apply();
        theme.apply();

    };
}

#[macro_export]
macro_rules! set_item_id {
    ($win:expr,$id:expr) => {
        $win.set_xclass($id);
        $win.set_id($id);
    };
}


