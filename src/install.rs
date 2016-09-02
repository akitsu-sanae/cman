/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::process::exit;
use std::process::Command;
use std::env;
use error_message::ErrMsg;
use read_packages::read_packages;


pub fn command(args: Vec<String>) {
    let em = ErrMsg::new("packages.toml");

    let packages = read_packages();
    let packages = packages
        .as_table().expect(em.invalid_file().as_str());

    let cman_config_path = env::var("CMAN_CONFIG_PATH")
        .expect("CMAN_CONFIG_PATH is not set");

    let date = packages
        .get("date").expect(em.missing("date").as_str())
        .as_str().expect(em.must_be("date").as_str());
    let install_dir = packages
        .get("install_dir").expect(em.missing("install_dir").as_str())
        .as_str().expect(em.must_be("install_dir").as_str())
        .to_string().replace("__CMAN_CONFIG_PATH__", cman_config_path.as_str());


    let packages = packages
        .get("packages").expect(em.missing("packages").as_str())
        .as_table().expect(em.must_be("packages").as_str());

    let name = args[1].as_str();

    let package = packages
        .get(name).expect(format!("{} not found", name).as_str())
        .as_table().expect(em.must_be("package data").as_str());

    let url = package
        .get("url").expect(em.missing("url").as_str())
        .as_str().expect(em.missing("url").as_str());

    let version = package
        .get("version").expect(em.missing("version").as_str())
        .as_str().expect(em.must_be("version").as_str());

        println!("\u{001B}[32mInstall {} ver{} to {}\u{001B}[39m", name, version, install_dir);

    let commands = package
        .get("build_commands").expect(em.missing("build commands").as_str())
        .as_slice().expect(em.must_be("build commands").as_str());

    for command in commands {
        let mut command = command
            .as_slice().expect(em.must_be("command").as_str())
            .into_iter().map(|e| {
                e.as_str().expect(em.must_be("element of command").as_str()).to_string()
            }).collect::<Vec<String>>();

        let args = command.split_off(1)
            .into_iter().map(|arg| {
                arg
                    .replace("__URL__", url)
                    .replace("__VERSION__", version)
                    .replace("__INSTALL_DIR__", install_dir.as_str())
            }).collect::<Vec<String>>();
        let exe = command[0].clone();

        let line = format!("{} {}", exe, args.join(" "));
        println!("\u{001B}[34mRun `{}`\u{001B}[39m", line);

        let output = Command::new(exe)
            .args(args.as_slice())
            .output().expect(format!("\u{001B}[31mfailed at: {}\u{001B}[39m", line).as_str());

        println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        if !output.status.success() {
            println!("\u{001B}[31mInstall Failed...\u{001B}[39m");
            exit(-1);
        }
    }

    println!("\u{001B}[32mSuccessfully Installed [{} ver {}]\u{001B}[39m", name, version);
}

