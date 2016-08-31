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
use toml::Value;

pub fn write(toml: Value) {
    let mut makefile = String::new();
    let config = Config::new(&toml);

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

struct Config {
    package: Package,
    path: Path,
    build: Build,
}
impl Config {
    pub fn new(toml: &Value) -> Config {
        let toml = toml.as_table().expect("invalid config");

        let package = toml.get("package").expect("invalid config: missing package");
        let path = toml.get("path").expect("invalid config: missing path");
        let build = toml.get("build").expect("invalid config: missing build");
        Config {
            package: Package::new(package),
            path: Path::new(path),
            build: Build::new(build)
        }
    }
}

struct Package {
    name: String,
}
impl Package {
    pub fn new(toml: &Value) -> Package {
        let toml = toml.as_table().expect("invalid package config");
        let name = toml.get("name")
            .expect("invalid package config: missing name")
            .as_str().expect("invalid package config: name must be String").to_string();
        Package {
            name: name
        }
    }
}

struct Path {
    lib_dirs: Vec<String>,
    libs: Vec<String>,
    includes: Vec<String>,
    src_dir: String,
}
impl Path {
    pub fn new(toml: &Value) -> Path {
        let toml = toml.as_table().expect("invalid path config");

        let lib_dirs = toml.get("lib_dirs")
            .expect("invalid path config: missing lib_dirs")
            .as_slice().expect("invalid path config: lib_dirs must be Array")
            .into_iter().map(|value| {
                value.as_str().expect("invalid path config: element in lib_dirs must be String").to_string()
            }).collect();
        let libs = toml.get("libs")
            .expect("invalid path config: missing libs")
            .as_slice().expect("invalid path config: libs must be Array")
            .into_iter().map(|value| {
                value.as_str().expect("invalid path config: element in libs must be String").to_string()
            }).collect();
        let includes = toml.get("includes")
            .expect("invalid path cofig: missing includes")
            .as_slice().expect("invalid path config: includes must be Array")
            .into_iter().map(|value| {
                value.as_str().expect("invalid path config: element in includes must be String").to_string()
            }).collect();
        let src_dir = toml.get("source")
            .expect("invalid path config: missing source")
            .as_str().expect("invalid path config: source must be String").to_string();

        Path {
            lib_dirs: lib_dirs,
            libs: libs,
            includes: includes,
            src_dir: src_dir,
        }
    }
}

struct Build {
    compiler: String,
    args: Vec<String>,
}

impl Build {
    pub fn new(toml: &Value) -> Build {
        let toml = toml.as_table().expect("invalid build config");
        let compiler = toml.get("compiler")
            .expect("invalid build config: missing compiler")
            .as_str().expect("invalid build config: compiler must be String").to_string();
        let args = toml.get("args")
            .expect("invalid build config: missing args")
            .as_slice().expect("invalid bulid config: args must be Array")
            .into_iter().map(|value| {
                value.as_str().expect("invalid build config: element in agrs must be String").to_string()
            }).collect();
        Build {
            compiler: compiler,
            args: args,
        }
    }
}


