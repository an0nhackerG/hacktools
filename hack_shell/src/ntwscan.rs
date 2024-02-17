#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use crate::{
    logger::*,
    shared::{ModuleName, NtwsError},
};

use tokio;
use std::io::prelude::*;
use std::net::*;

struct Scanner {
    ip: Vec<String>,
    ports: Vec<u16>,
    results: String,
    timeout: u32,
    verbosity_level: u8,
    module_name: String,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            ip: Vec::new(),
            ports: Vec::new(),
            results: String::new(),
            timeout: 0,
            verbosity_level: 4,
            module_name: "NTWS".to_string(),
        }
    }
}

impl Logger for Scanner {
    fn log(&self, message: String, target_verbose_level: u8, log_type: LogType) {
        if self.verbosity_level >= target_verbose_level {
            use_log(target_verbose_level, &self.module_name, message, log_type)
        }
    }
}

pub fn ntws_main(args: Vec<String>) {
    let mut scanner = Scanner::new();
    println!("Ntwscan Started");
}
