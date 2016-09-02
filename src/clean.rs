/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::process::Command;

pub fn command(_: Vec<String>) {
    let output = Command::new("make")
        .arg("clean")
        .output()
        .expect("\u{001B}[31mfaild to clean\u{001B}[39m");

    println!("{}\u{001B}[39m", if output.status.success() {
        "\u{001B}[32msuccessfully cleaned"
    } else {
        "\u{001B}[31mfaild to clean..."
    });
}

