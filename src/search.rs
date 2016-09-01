/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/cman
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use read_packages::read_packages;

pub fn command(args: Vec<String>) {

    let packages = read_packages();
    let packages = packages
        .as_table().expect("invalid packages file")
        .get("packages").expect("missing packages")
        .as_table().expect("invalid packages file: packages must be Table");

    for (name, package) in packages {
        let package = package
            .as_table().expect(format!("{}: right hands side must be Table\n{}", name, package).as_str());

        let version = package
            .get("version").expect(format!("{}: missing version", name).as_str())
            .as_str().expect(format!("{}: version must be String", name).as_str());

        let desc = package
            .get("description").expect(format!("{}: missing description", name).as_str())
            .as_str().expect(format!("{}: desctiption must be String", name).as_str());

        if name.contains(args[1].as_str()) || desc.contains(args[1].as_str()) {
            println!("{} {}: {}", name, version, desc);
        }
    }
}

