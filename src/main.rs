/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use std::str::FromStr;
extern crate argparse;
extern crate toml;

use argparse::{ArgumentParser, Store, List};

mod config;

mod build;
mod clean;
mod new;
mod run;

#[derive(Debug)]
enum Command {
    Build,
    Clean,
    New,
    Run,
    Test,
    Update,
    Upgrade,
    Search,
    Install,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(src: &str) -> Result<Command, ()> {
        return match src {
            "build" => Ok(Command::Build),
            "clean" => Ok(Command::Clean),
            "new" => Ok(Command::New),
            "run" => Ok(Command::Run),
            "test" => Ok(Command::Test),
            "update" => Ok(Command::Update),
            "upgrade" => Ok(Command::Upgrade),
            "search" => Ok(Command::Search),
            "install" => Ok(Command::Install),
            _ => Err(()),
        }
    }
}

fn main() {
    let mut command = Command::Build;
    let mut args = vec!();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("C++ bulidtool and package manager.");
        ap.refer(&mut command).required()
            .add_argument("<command>", Store,
                r#""build", "clean", "new", "run", "test", "update", "upgrade", "search" or "install""#);
        ap.refer(&mut args)
            .add_argument("arguments", List,
                r#"argmument for command"#);
        ap.stop_on_first_argument(true);
        ap.parse_args_or_exit();
    }

    args.insert(0, format!("command {:?}", command));

    match command {
        Command::Build => build::command(args),
        Command::Clean => clean::command(args),
        Command::New => new::command(args),
        Command::Run => run::command(args),
        Command::Test => println!("test"),
        Command::Update => println!("update"),
        Command::Upgrade => println!("upgrade"),
        Command::Search => println!("search"),
        Command::Install => println!("install"),
    }
}
