/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::env;
use config::Config;

pub fn write(config: Config) {
    let mut makefile = String::new();

    let filename = format!("{}/Makefile.template", env::var("CMAN_CONFIG_PATH").expect("CMAN_CONFIG_PATH is not set"));
    File::open(filename).and_then(|mut f| {
        f.read_to_string(&mut makefile)
    }).expect("can not open Makefile.template");


    File::create("./Makefile").and_then(|mut f| {
        f.write(makefile
                .replace("__PAC_NAME__",
                         config.package.name.as_str())
                .replace("__COMPILER__",
                         config.build.compiler.as_str())
                .replace("__ARGS__",
                         config.build.args.join(" ").as_str())
                .replace("__LIB_DIRS__",
                         config.path.lib_dirs.clone()
                             .into_iter().map(|dir| format!("-L{}", dir))
                             .collect::<Vec<String>>()
                         .join(" ").as_str())
                .replace("__LIBS__",
                         config.path.libs.clone()
                             .into_iter().map(|lib| format!("-l{}", lib))
                             .collect::<Vec<String>>()
                         .join(" ").as_str())
                .replace("__INCLUDES__",
                         config.path.includes.clone()
                             .into_iter().map(|inc| format!("-I{}", inc))
                             .collect::<Vec<String>>()
                         .join(" ").as_str())
                .replace("__SRC_DIR__",
                         config.path.src_dir.as_str())
                .as_bytes())

    }).expect("can not create Makefile");
}

