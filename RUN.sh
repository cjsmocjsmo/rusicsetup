#!/bin/bash

rm -rf /usr/share/rusicsetup/rusicsetup/nfo/*;

rm -rf /usr/share/rusicsetup/rusicsetup/db/*;

touch /usr/share/rusicsetup/rusicsetup/db/rusic.db;

rm -rf /usr/share/rusicsetup/rusicsetup/thumbnails/*;

cd /usr/share/rusicsetup/rusicsetup;

git pull;

cargo run --release

