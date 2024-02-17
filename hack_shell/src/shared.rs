use termion::color::{
    self, Blue, Fg, Green, LightBlue, LightMagenta, LightRed, Magenta, Red, Reset,
};

// this archive is used to declare some enum/structs/const that will be used by all modules in a individual form,
// like errors, names, and colors to use in logs and the CLI itself

#[derive(Debug)]
pub enum ModuleName {
    Ntwscan,
    Exploited,
    MalwareLab,
}

pub enum CommonErros {
    InternalError,
    UnkownError,
    InvalidParameter,
}

pub enum NtwsError {
    ErrorSendingPackage,
    PortClosed,
    ConnectionRefused,
}

pub enum DatabaseError {
    ErrorAddingInfo,
}

// Some colors for use in the CLI, just use like "{RED_RG}someStringHere{RESET_FG}"
pub const RED_FG: Fg<Red> = color::Fg(color::Red);
pub const LIGHT_RED_FG: Fg<LightRed> = color::Fg(color::LightRed);
pub const BLUE_FG: Fg<Blue> = color::Fg(color::Blue);
pub const LIGHT_BLUE_FG: Fg<LightBlue> = color::Fg(color::LightBlue);
pub const MAGENTA_FG: Fg<Magenta> = color::Fg(color::Magenta);
pub const LIGHT_MAGENTA_FG: Fg<LightMagenta> = color::Fg(color::LightMagenta);
pub const RESET_FG: Fg<Reset> = color::Fg(color::Reset);
pub const GREEN_FG: Fg<Green> = color::Fg(color::Green);
