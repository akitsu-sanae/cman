/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use std::process::exit;
use toml::{Parser, Table};
use std::fs::File;
use std::io::Read;

pub fn read() -> Table {
    let mut input = String::new();
    File::open("./cman.toml").and_then(|mut f| {
        f.read_to_string(&mut input)
    }).expect("can not open ./cman.toml");
    let input = input;

    let mut parser = Parser::new(&input);
    match parser.parse() {
        Some(toml) => toml,
        None => {
            for err in &parser.errors {
                let (low_line, low_col) = parser.to_linecol(err.lo);
                let (hi_line, hi_col) = parser.to_linecol(err.hi);
                println!("fail parsing cman.toml at {}:{}-{}:{} : {}",
                         low_line, low_col, hi_line, hi_col, err.desc);
            }
            exit(-1);
        }
    }
}

