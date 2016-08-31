#! /bin/sh

if [ -z $CMAN_CONFIG_PATH ]; then
    echo "\$CMAN_CONFIG_PATH was not set"
    echo "stop install cman"
    exit 1
fi

if [ ! -f ./config/Makefile.template ]; then
    echo "missing ./config/Makefile.template"
    echo "please run 'git clone ...' again"
    exit 1
fi

if [ ! -f ./config/main.cpp ]; then
    echo "missing ./config/main.cpp"
    echo "please run 'git clone ...' again"
    exit 1
fi

if [ ! -f ./config/cman.toml ]; then
    echo "missing ./config/cman.toml"
    echo "please run 'git clone ...' again"
    exit 1
fi

mkdir $CMAN_CONFIG_PATH
cp ./config/* $CMAN_CONFIG_PATH


