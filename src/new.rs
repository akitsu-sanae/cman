/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use std::fs;
use std::env;

fn mkdir(path: &String) {
    fs::create_dir_all(path.as_str())
        .expect(format!("can not create directory: {}", path).as_str());
}

pub fn command(args: Vec<String>) {
    let ref path = args[1];
    mkdir(path);
    mkdir(&format!("{}/build/obj", path));
    mkdir(&format!("{}/include", path));
    mkdir(&format!("{}/src", path));

    let cman_dir = env::var("CMAN_CONFIG_PATH")
        .expect("CMAN_CONFIG_PATH is not set");

    fs::copy(format!("{}/main.cpp", cman_dir), format!("{}/main.cpp", path))
        .expect("can not copy main.cpp");
    fs::copy(format!("{}/cman.toml", cman_dir), format!("{}/cman.toml", path))
        .expect("can not copy cman.toml");
}

