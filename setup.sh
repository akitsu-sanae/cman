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

cp ./config/* $CMAN_CONFIG_PATH


