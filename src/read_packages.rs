/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use std::process::exit;
use std::env;
use std::fs::File;
use std::io::Read;
use toml::{Parser, Value};

pub fn read_packages() -> Value {
    let mut input = String::new();
    let filename = format!("{}/config/packages.toml", env::var("CMAN_CONFIG_PATH").expect("CMAN_CONFIG_PATH is not set"));

    File::open(filename).and_then(|mut f| {
        f.read_to_string(&mut input)
    }).expect("missing packages.toml");

    let input = input;

    let mut parser = Parser::new(&input);
    match parser.parse() {
        Some(toml) => Value::Table(toml),
        None => {
            for err in &parser.errors {
                let (low_line, low_col) = parser.to_linecol(err.lo);
                let (hi_line, hi_col) = parser.to_linecol(err.hi);
                println!("fail parsing packages.toml at {}:{}-{}:{} : {}",
                         low_line, low_col, hi_line, hi_col, err.desc);
            }
            exit(-1);
        },
    }
}


