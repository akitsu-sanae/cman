/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::process::exit;
use std::env;

pub fn command(args: Vec<String>) {
    let ref pack_name = args[1];

    let cman_dir = env::var("CMAN_CONFIG_PATH")
        .expect("CMAN_CONFIG_PATH is not set");

    let output = Command::new("cp")
        .arg("-r")
        .arg(format!("{}/template", cman_dir).as_str())
        .arg(pack_name)
        .output()
        .expect("\u{001B}[31mfaild to create cman project...\u{001B}[39m");

    if !output.status.success() {
        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        println!("\u{001B}[31mfaild to create cman project...\u{001B}[39m");
        exit(-1);
    }

    let mut cman_toml = String::new();
    File::open(format!("{}/cman.toml", pack_name).as_str()).and_then(|mut f| {
        f.read_to_string(&mut cman_toml)
    }).expect("\u{001B}[31mfaild to open cman.toml\u{001B}[39m");

    let user_name = env::var("USER")
        .expect("USER is not set");

    File::create(format!("{}/cman.toml", pack_name).as_str()).and_then(|mut f| {
        f.write(cman_toml
                .replace("__PAC_NAME__", pack_name)
                .replace("__USER_NAME__", user_name.as_str())
                .as_bytes())
    }).expect("\u{001B}[31mfaild to create cman.toml\u{001B}[39m");
}

