# What is cman?

C++ is a build system and package manager for C++.

# How to install

1. run `cargo build --release`
2. add your shell config file `export CMAN_CONFIG_PATH="$HOME/.config/cman"`
3. run 'sh ./setup.sh'


# How to use

* cman new ... make cman project directory and create some initial files.
* cman build ... create Makefile from cman.toml and then run `make`
* cman run ... run `make run`
* cman clean ... run `make clean`

and so on...  

# Copyright
Copyright (C) 2016 akitsu sanae.  
Distributed under the Boost Software License, Version 1.0. 
(See accompanying file LICENSE_1_0.txt or copy at http://www.boost/org/LICENSE_1_0.txt)  

cman uses [argparse](https://github.com/tailhook/rust-argparse), which is licensed by the MIT license.  

> Copyright (c) 2014-2015 Paul Colomiets

cman uses [toml](https://github.com/alexcrichton/toml-rs), which is licensed by the MIT license and the Apache License (Version 2.0).  

> Copyright (c) 2014 Alex Crichton


