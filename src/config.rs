/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::process::exit;
use std::fs::File;
use std::io::Read;
use std::option::Option;
use toml::{Parser, Value};

fn read_toml(filename: &str) -> Option<Value> {
    let mut input = String::new();
    match File::open(filename).and_then(|mut f| {
        f.read_to_string(&mut input)
    }) {
        Ok(_) => (),
        Err(_) => return None,
    }
    let input = input;

    let mut parser = Parser::new(&input);
    match parser.parse() {
        Some(toml) => Some(Value::Table(toml)),
        None => {
            for err in &parser.errors {
                let (low_line, low_col) = parser.to_linecol(err.lo);
                let (hi_line, hi_col) = parser.to_linecol(err.hi);
                println!("fail parsing cman.toml at {}:{}-{}:{} : {}",
                         low_line, low_col, hi_line, hi_col, err.desc);
            }
            exit(-1);
        },
    }
}

pub struct Config {
    pub package: Package,
    pub path: Path,
    pub build: Build,
}
impl Config {

    pub fn current() -> Option<Config> {
        Config::load("./cman.toml")
    }

    pub fn load(filename: &str) -> Option<Config> {

        let toml = match read_toml(filename) {
            Some(toml) => toml,
            None => return None,
        };
        let toml = toml.as_table().expect("invalid config");

        let package = toml.get("package").expect("invalid config: missing package");
        let path = toml.get("path").expect("invalid config: missing path");
        let build = toml.get("build").expect("invalid config: missing build");
        Some(Config {
            package: Package::new(package),
            path: Path::new(path),
            build: Build::new(build)
        })
    }
}

pub struct Package {
    pub name: String,
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

pub struct Path {
    pub lib_dirs: Vec<String>,
    pub libs: Vec<String>,
    pub includes: Vec<String>,
    pub src_dir: String,
    pub dest_dir: String,
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
        let dest_dir = toml.get("dest")
            .expect("invalid path config: missing dest")
            .as_str().expect("invalid path config: dest must be String").to_string();

        Path {
            lib_dirs: lib_dirs,
            libs: libs,
            includes: includes,
            src_dir: src_dir,
            dest_dir: dest_dir,
        }
    }
}

pub struct Build {
    pub compiler: String,
    pub args: Vec<String>,
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


