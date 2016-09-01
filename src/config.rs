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
use error_message::ErrMsg;

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
        let em = ErrMsg::new(filename);
        let toml = match read_toml(filename) {
            Some(toml) => toml,
            None => return None,
        };
        let toml = toml.as_table().expect(em.invalid_file().as_str());

        let package = toml.get("package").expect(em.missing("package").as_str());
        let path = toml.get("path").expect(em.missing("path").as_str());
        let build = toml.get("build").expect(em.missing("build").as_str());
        Some(Config {
            package: Package::new(package, &em),
            path: Path::new(path, &em),
            build: Build::new(build, &em)
        })
    }
}

pub struct Package {
    pub name: String,
}
impl Package {
    pub fn new(toml: &Value, em: &ErrMsg) -> Package {
        let toml = toml.as_table().expect(em.invalid("package").as_str());
        let name = toml.get("name")
            .expect(em.missing("name").as_str())
            .as_str().expect(em.must_be("name").as_str()).to_string();
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
    pub fn new(toml: &Value, em: &ErrMsg) -> Path {
        let toml = toml.as_table().expect(em.invalid("path").as_str());

        let lib_dirs = toml.get("lib_dirs")
            .expect(em.missing("lib_dirs").as_str())
            .as_slice().expect(em.must_be("lib_dirs").as_str())
            .into_iter().map(|value| {
                value.as_str().expect(em.must_be("element of lib_dirs").as_str()).to_string()
            }).collect();
        let libs = toml.get("libs")
            .expect(em.missing("libs").as_str())
            .as_slice().expect(em.must_be("libs").as_str())
            .into_iter().map(|value| {
                value.as_str().expect(em.must_be("element of libs").as_str()).to_string()
            }).collect();
        let includes = toml.get("includes")
            .expect(em.missing("includes").as_str())
            .as_slice().expect(em.must_be("includes").as_str())
            .into_iter().map(|value| {
                value.as_str().expect(em.must_be("element of includes").as_str()).to_string()
            }).collect();
        let src_dir = toml.get("source")
            .expect(em.missing("source").as_str())
            .as_str().expect(em.must_be("source").as_str()).to_string();
        let dest_dir = toml.get("dest")
            .expect(em.missing("dest").as_str())
            .as_str().expect(em.must_be("dest").as_str()).to_string();

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
    pub fn new(toml: &Value, em: &ErrMsg) -> Build {
        let toml = toml.as_table().expect(em.invalid("build").as_str());
        let compiler = toml.get("compiler")
            .expect(em.missing("compiler").as_str())
            .as_str().expect(em.must_be("compiler").as_str()).to_string();
        let args = toml.get("args")
            .expect(em.missing("args").as_str())
            .as_slice().expect(em.must_be("args").as_str())
            .into_iter().map(|value| {
                value.as_str().expect(em.must_be("element of args").as_str()).to_string()
            }).collect();
        Build {
            compiler: compiler,
            args: args,
        }
    }
}


