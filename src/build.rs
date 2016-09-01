/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use std::process::Command;
use std::io::{stdout, stderr};
use std::process::exit;
use write_makefile;
use argparse::{ArgumentParser, StoreFalse};
use config::Config;

pub fn command(_args: Vec<String>) {
    let config = Config::current()
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

#[allow(dead_code)]
fn is_release(args: Vec<String>) -> bool {
    let mut is_release = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("compile the current cman project");
        ap.refer(&mut is_release)
            .add_option(&["--release"], StoreFalse,
                        r#"Release Build"#);
        match ap.parse(args, &mut stdout(), &mut stderr()) {
            Ok(()) => (),
            Err(x) => exit(x),
        }
    }
    is_release
}

