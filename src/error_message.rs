/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use std::process;

pub struct ErrMsg {
    filetype: String,
}

impl ErrMsg {
    pub fn new(filetype: &str) -> ErrMsg {
        ErrMsg {
            filetype: filetype.to_string(),
        }
    }

    pub fn invalid(&self, v: &str) ->  String {
        format!("invalid {} file: invalid {} config", self.filetype, v)
    }
    pub fn invalid_file(&self) -> String {
        format!("invalid {} file", self.filetype)
    }
    pub fn missing(&self, var: &str) -> String {
        format!("invalid {} file: missing {}", self.filetype, var)
    }
    pub fn must_be(&self, v: &str) -> String {
        let t = match v {
            // cman.toml
            "name" | "version" | "source" | "dest" | "compiler" |
            "element of author" | "element of lib_dirs" |
            "element of libs" | "element of includes" |
            "element of args"
                => "String",
            "author" | "lib_dirs" | "libs" | "includes" | "args"
                => "Array",
            "package" | "path" | "build"
                => "Table",

            // packages.toml
            "date" | "install_dir" | "description" |
            "url" | "element of command"
                => "String",
            "build commands" | "command"
                => "Array",
            "packages" | "package data"
                => "Table",

             _ => {
                 println!("invalid value: {}", v);
                 process::exit(2)
             },
        };
        format!("invalid {} file: {} must be {}", self.filetype, v, t)
    }
}


