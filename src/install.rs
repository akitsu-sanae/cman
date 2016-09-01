/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::process::Command;
use std::process::exit;
use std::env;
use std::fs::File;
use std::io::Read;
use toml::{Parser, Value};


pub fn command(args: Vec<String>) {
    let packages = read_packages();
    let packages = packages
        .as_table().expect("invalid package file");

    let date = packages
        .get("date").expect("invalid packages file: missing date")
        .as_str().expect("date must be String");
    let install_dir = packages
        .get("install_dir").expect("invalid packages file: missing install_dir")
        .as_str().expect("install_dir must be String");

    let packages = packages
        .get("packages").expect("missing packages")
        .as_table().expect("package data must be Table");

    let name = args[1].as_str();

    let package = packages
        .get(name).expect(format!("{} not found", name).as_str())
        .as_table().expect("package data must be Table");

    let url = package
        .get("url").expect("invalid package file: missing url")
        .as_str().expect("invalid package file: url must be String");

    let version = package
        .get("version").expect("invalid package file: missing version")
        .as_str().expect("invalid package file: version must be String");

        println!("\u{001B}[32mInstall {} ver{} to {}\u{001B}[39m", name, version, install_dir);

    let commands = package
        .get("build_commands").expect("invalid package file: missing build_commands")
        .as_slice().expect("invalid package file: build_commands must be Array");

    for command in commands {
        let mut command = command
            .as_slice().expect("invalid package file: command must be Array")
            .into_iter().map(|e| {
                e.as_str().expect("invalid packages file: element of command must be String").to_string()
            }).collect::<Vec<String>>();

        let args = command.split_off(1)
            .into_iter().map(move |arg| {
                arg
                    .replace("__URL__", url)
                    .replace("__VERSION__", version)
                    .replace("__INSTALL_DIR__", install_dir)
            }).collect::<Vec<String>>();
        let exe = command[0].clone();

        let output = Command::new(exe)
            .args(args.as_slice())
            .output().expect(format!("\u{001B}[31mfailed at: {}\u{001B}[39m", command.join(" ")).as_str());

        println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    println!("\u{001B}[32mSuccessfully Installed [{} ver {}]\u{001B}[39m", name, version);
}

fn read_packages() -> Value {
    let mut input = String::new();
    let filename = format!("{}/packages.toml", env::var("CMAN_CONFIG_PATH").expect("CMAN_CONFIG_PATH is not set"));

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


