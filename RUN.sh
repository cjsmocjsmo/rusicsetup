#!/bin/bash

if [ -d /usr/share/rusicsetup/rusicsetup/nfo]; then
    rm -rf /usr/share/rusicsetup/rusicsetup/nfo/*;
else
    mkdir /usr/share/rusicsetup/rusicsetup/nfo;
fi

if [ -d /usr/share/rusicsetup/rusicsetup/db]; then
    rm -rf /usr/share/rusicsetup/rusicsetup/db/*;
    touch /usr/share/rusicsetup/rusicsetup/db/rusic.db;
else
    mkdir /usr/share/rusicsetup/rusicsetup/db;
    touch /usr/share/rusicsetup/rusicsetup/db/rusic.db;
fi

cd /usr/share/rusicsetup/rusicsetup;

git pull;

cargo run --release