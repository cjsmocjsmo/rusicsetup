# rusicsetup

![Project Screenshot](screenshot.png "width=400px")

rusicsetup is part of the rusicsetup and rusic trio (rusic, rusicsetup, rusic-svelte).
Rusic has been designed to run on the raspberry pi 3 and above.
Rusicsetup is exactly what it says it is, it interates over your music
collection extracts tag info, populates the sqlite3 db etc... 

Rust was choosen for it's speed.  Rusicsetup takes approx 3min to go through 2100 mp3s.

## Prerequisites

1. raspberrypiOS (bookworm)
2. rust

Rusicsetup scans the $Home/Music directory, it will also scan for any USB devices found in /media. Please put your music collection is in any of these locations.

## Usage

1. Create the directory /usr/share/rusicsetup
2. Adjust permissions as needed to do the next step
3. Git clone this repository to /usr/share/rusicsetup/
4. Edit .env as needed for your setup (pagination, address, port, etc...)
5. Execute RUN.sh

```bash

#insure rust is installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#make the needed dirs adjust permissions
mkdir /usr/share/rusicsetup
chmod 755 -R /usr/share/rusicsetup
chown pipi:pipi /usr/share/rusicsetup # change pipi to meet your needs

#clone this repo
git clone https://github.com/cjsmocjsmo/rusicsetup.git
cd /usr/share/rusicsetup/rusicsetup

#run the script
sh RUN.sh
```

## Warning

Rusic was designed to run on your home network and not on the wider internet.  It has no authentication system so you have been warned!!!