
use crate::shared::{
    BLUE_FG, GREEN_FG, LIGHT_BLUE_FG, LIGHT_MAGENTA_FG, LIGHT_RED_FG, MAGENTA_FG, RED_FG, RESET_FG,
};
pub enum VerbosityLevel {
    Quiet,
    Normal,
    High,
    Debug,
    Trace,
}

pub enum LogType {
    Info,
    Error,
    Sucess,
}

use crate::shared::ModuleName;

// Trait Logger will be implemented in all modules.
pub trait Logger {
    fn log(&self, message: String, target_verbose_level: u8, log_type: LogType);
}

pub fn use_log(_verbosity_level: u8, module_name: &str, message: String, log_type: LogType) {
    match log_type {
        LogType::Info => println!(
            "[{BLUE_FG}{}{RESET_FG}]-[{LIGHT_BLUE_FG}INFO{RESET_FG}]-> {}",
            module_name, message
        ),
        LogType::Error => println!(
            "[{BLUE_FG}{}{RESET_FG}]-[{RED_FG}ERROR{RESET_FG}]-> {}",
            module_name, message
        ),
        LogType::Sucess => println!(
            "[{BLUE_FG}{}{RESET_FG}]-[{GREEN_FG}SUCESS{RESET_FG}]-> {}",
            module_name, message
        ),
    }
}
