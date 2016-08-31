/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use std::fs;
use std::env;
use config::Config;

fn mkdir(path: &String) {
    fs::create_dir_all(path.as_str())
        .expect(format!("can not create directory: {}", path).as_str());
}

pub fn command(args: Vec<String>) {
    let ref pack_name = args[1];
    let cman_dir = env::var("CMAN_CONFIG_PATH")
        .expect("CMAN_CONFIG_PATH is not set");

    let config = Config::load(format!("{}/cman.toml", cman_dir).as_str())
        .expect("missing default cman.toml");

    mkdir(pack_name);
    mkdir(&format!("{}/build/obj", pack_name));
    for inc in config.path.includes {
        if inc.as_bytes()[0] != '.' as u8 { continue } // maybe, this is not relative path
        mkdir(&format!("{}/{}", pack_name, inc));
    }

    if config.path.src_dir.as_bytes()[0] == '.' as u8 {
        mkdir(&format!("{}/{}", pack_name, config.path.src_dir));
    }

    fs::copy(format!("{}/main.cpp", cman_dir), format!("{}/{}/main.cpp", pack_name, config.path.src_dir))
        .expect("can not copy main.cpp");
    fs::copy(format!("{}/cman.toml", cman_dir), format!("{}/cman.toml", pack_name))
        .expect("can not copy cman.toml");
}

