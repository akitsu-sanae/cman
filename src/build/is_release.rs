/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use std::io::{stdout, stderr};
use std::process::exit;
use argparse::{ArgumentParser, StoreFalse};

#[allow(dead_code)]
pub fn parse(args: Vec<String>) -> bool {
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

