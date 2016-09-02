/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::process::Command;
use config::Config;
use write_makefile;

pub fn command(_: Vec<String>) {
    let config = Config::current()
        .expect("missing ./cman.toml");
    write_makefile::write(config);

    let output = Command::new("make")
        .arg("run")
        .output()
        .expect("faild to run");
    println!("{}", output.status);
    println!("output:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("error:\n{}", String::from_utf8_lossy(&output.stderr));
}


