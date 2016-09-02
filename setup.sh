#! /bin/sh

if [ -z $CMAN_CONFIG_PATH ]; then
    echo "\$CMAN_CONFIG_PATH was not set"
    echo "stop install cman"
    exit 1
fi

if [ ! -f ./template/Makefile ]; then
    echo "missing ./template/Makefile"
    echo "please run 'git clone ...' again"
    exit 1
fi

if [ ! -f ./template/cman.toml ]; then
    echo "missing ./template/cman.toml"
    echo "please run 'git clone ...' again"
    exit 1
fi

mkdir $CMAN_CONFIG_PATH
mkdir $CMAN_CONFIG_PATH/bin
mkdir $CMAN_CONFIG_PATH/tmp
mkdir $CMAN_CONFIG_PATH/bundle
mkdir $CMAN_CONFIG_PATH/config
mkdir $CMAN_CONFIG_PATH/template
cp -r ./config/* $CMAN_CONFIG_PATH/config
cp -r ./template/* $CMAN_CONFIG_PATH/template

cargo build --release
cp ./target/release/cman $CMAN_CONFIG_PATH/bin


