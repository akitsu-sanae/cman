/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::env;
use std::process::Command;

pub fn command(_: Vec<String>) {
    let cman_config_path = env::var("CMAN_CONFIG_PATH")
        .expect("CMAN_CONFIG_PATH is not set");

    let packages_file = format!("{}/config/packages.toml", cman_config_path);

    let output = Command::new("curl")
        .arg("https://raw.githubusercontent.com/akitsu-sanae/cman/master/config/packages.toml")
        .arg("-o")
        .arg(packages_file.as_str())
        .output()
        .expect("failed to download...");

    if output.status.success() {
        println!("\u{001B}[32mSuccessfully Updated!!\u{001B}[39m");
    } else {
        println!("\u{001B}[31mFailed to Updated...\u{001B}[39m");
    }

}

