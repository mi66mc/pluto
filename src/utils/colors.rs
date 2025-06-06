pub const BLUE: &str = "\x1B[34m";
pub const GREEN: &str = "\x1B[32m";
pub const RED: &str = "\x1B[31m";
pub const YELLOW: &str = "\x1B[33m";
pub const RESET: &str = "\x1B[0m";
pub const BOLD: &str = "\x1B[1m";
pub const GREY: &str = "\x1B[90m";
// pub const RED_BACKGROUND: &str = "\x1B[41m";
// // pub const CYAN: &str = "\x1B[36m";
// // pub const MAGENTA: &str = "\x1B[35m";

// pub const STRING_COLOR: &str = "\x1B[32m";
// pub const NUMBER_COLOR: &str = "\x1B[36m";
// pub const KEYWORD_COLOR: &str = "\x1B[35m";
// pub const IDENTIFIER_COLOR: &str = "\x1B[94m";
// pub const OPERATOR_COLOR: &str = "\x1B[37m";

pub fn blue(text: &str) -> String {
    format!("{}{}{}", BLUE, text, RESET)
}

pub fn green(text: &str) -> String {
    format!("{}{}{}", GREEN, text, RESET)
}

pub fn red(text: &str) -> String {
    format!("{}{}{}", RED, text, RESET)
}

// pub fn yellow(text: &str) -> String {
//     format!("{}{}{}", YELLOW, text, RESET)
// }

pub fn bold(text: &str) -> String {
    format!("{}{}{}", BOLD, text, RESET)
}

// pub fn cyan(text: &str) -> String {
//     format!("{}{}{}", CYAN, text, RESET)
// }

// pub fn magenta(text: &str) -> String {
//     format!("{}{}{}", MAGENTA, text, RESET)
// }

// pub fn error(msg: &str) -> String {
//     format!("{}Error:{} {}", RED, RESET, msg)
// }

// pub fn success(msg: &str) -> String {
//     format!("{}{}{}", GREEN, msg, RESET)
// }

// pub fn info(msg: &str) -> String {
//     format!("{}{}{}", BLUE, msg, RESET)
// }

// pub fn warning(msg: &str) -> String {
//     format!("{}{}{}", YELLOW, msg, RESET)
// } 