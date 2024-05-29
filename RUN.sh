#!/bin/bash

if [ -d /usr/share/rusic/rusic/nfo]; then
    rm -rf /usr/share/rusic/rusic/nfo/*;
else
    mkdir /usr/share/rusic/rusic/nfo;
fi

if [ -d /usr/share/rusic/rusic/assets]; then
    rm -rf /usr/share/rusic/rusic/assets/*;
else
    mkdir /usr/share/rusic/rusic/assets;
fi

if [ -d /usr/share/rusic/rusic/db]; then
    rm -rf /usr/share/rusic/rusic/db/*;
else
    mkdir /usr/share/rusic/rusic/db;
    touch /usr/share/rusic/rusic/db/rusic.db;
fi

cd /usr/share/rusicsetup/rusicsetup;

git pull;

cargo run --release -- -d default;