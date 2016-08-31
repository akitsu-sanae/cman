/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use std::process::Command;
use toml::Value;

mod is_release;
mod read_toml;
mod write_makefile;

pub fn command(_args: Vec<String>) {
    // let release = is_release::parse(args);
    let toml = read_toml::read();
    write_makefile::write(Value::Table(toml));
    Command::new("make")
        .output()
        .expect("faild to make");
}

