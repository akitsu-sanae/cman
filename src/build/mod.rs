/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use std::process::Command;
use config::Config;

mod is_release;
mod write_makefile;

pub fn command(_args: Vec<String>) {
    // let release = is_release::parse(args);

    let config = Config::load("./cman.toml")
        .expect("missing ./cman.toml");
    write_makefile::write(config);
    let output = Command::new("make")
        .output()
        .expect("faild to make");

    println!("stdout\n{}", String::from_utf8_lossy(&output.stdout));
    println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));

    println!("build: {}\u{001B}[39m", if output.status.success() {
        "\u{001B}[32msuccess!!"
    } else {
        "\u{001B}[31mfaild..."
    });
}

