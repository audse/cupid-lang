pub mod modifier {
    pub static ITALIC: &str = "3";
    pub static RESET: &str = "0";
    pub static BOLD: &str = "1";
    pub static DIM: &str = "2";
    pub static UNDERLINE: &str = "4";
    pub static BLINK: &str = "5";
    pub static REVERSE: &str = "7";
    pub static HIDDEN: &str = "8";
}

pub mod fg {
    pub static BLACK: &str = "30";
    pub static RED: &str = "31";
    pub static GREEN: &str = "32";
    pub static YELLOW: &str = "33";
    pub static BLUE: &str = "34";
    pub static MAGENTA: &str = "35";
    pub static CYAN: &str = "36";
    pub static WHITE: &str = "37";
    pub static GREY: &str = "90";
}

pub mod bg {
    pub static BLACK: &str = "40";
    pub static RED: &str = "41";
    pub static GREEN: &str = "42";
    pub static YELLOW: &str = "43";
    pub static BLUE: &str = "44";
    pub static MAGENTA: &str = "45";
    pub static CYAN: &str = "46";
    pub static WHITE: &str = "47";
    pub static GREY: &str = "100";
}

#[derive(Default, Debug)]
pub struct ColorString {
    pub string: String,
    pub fg: Option<&'static str>,
    pub bg: Option<&'static str>,
    pub fg_modifiers: Vec<&'static str>,
    pub bg_modifiers: Vec<&'static str>,
}

pub fn color<S: Into<String>>(string: S) -> ColorString {
    ColorString {
        string: string.into(),
        ..Default::default()
    }
}

fn to_console_color<S: Into<String>>(inner: S) -> String {
    format!("\x1b[{}m", inner.into())
}

macro_rules! color_fn {
    ( $( ($($tail:tt)*) ),* $(,)? ) => {
        $(
            color_fn! { $($tail)* }
        )*
    };
    ($id:ident $(, $bg_id:ident)? -> with $color:tt) => {
        pub fn $id(mut self) -> Self {
            self.fg_modifiers.push(modifier::$color);
            self
        }
        $( pub fn $bg_id(mut self) -> Self {
            self.bg_modifiers.push(modifier::$color);
            self
        } )?
    };
    ($id:ident, $bg_id:ident -> $color:tt) => {
        pub fn $id(self) -> Self {
            Self { fg: Some(fg::$color), ..self }
        }
        pub fn $bg_id(self) -> Self {
            Self { bg: Some(bg::$color), ..self }
        }
    };
    () => {};
}

impl ColorString {
    color_fn! {
        // Foreground & background colors
        (black, bg_black -> BLACK),
        (red, bg_red -> RED),
        (green, bg_green -> GREEN),
        (yellow, bg_yellow -> YELLOW),
        (blue, bg_blue -> BLUE),
        (magenta, bg_magenta -> MAGENTA),
        (cyan, bg_cyan -> CYAN),
        (white, bg_white -> WHITE),
        (grey, bg_grey -> GREY),

        // Modifiers
        (italic -> with ITALIC),
        (reset, bg_reset -> with RESET),
        (bold -> with BOLD),
        (dim, bg_dim -> with DIM),
        (underline -> with UNDERLINE),
        (blink, bg_blink -> with BLINK),
        (reverse -> with REVERSE),
        (hidden -> with HIDDEN)
    }

    pub fn ok(&self) -> String {
        let full_color: Vec<&str> = [
            self.fg_modifiers.to_vec(),
            self.bg_modifiers.to_vec(),
            self.fg.map(|c| vec![c]).unwrap_or_default(),
            self.bg.map(|c| vec![c]).unwrap_or_default(),
        ]
        .into_iter()
        .flatten()
        .filter(|color| !color.is_empty())
        .collect();
        format!(
            "{begin}{content}{end}",
            begin = to_console_color(full_color.join(";")),
            content = self.string,
            end = to_console_color(modifier::RESET)
        )
    }
}

impl std::fmt::Display for ColorString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ok())
    }
}
