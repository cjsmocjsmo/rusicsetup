# rusicsetup

![Project Screenshot](screenshot.png "width=450px")

rusicsetup is part of the rusic-svelte and rusic trio.


## Prerequisites

1. raspberrypiOS (bookworm)
2. rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


## Usage

Create the directory /usr/share/rusicsetup
Adjust permissions as needed to do the next step
Git clone this repository to /usr/share/rusicsetup/
Edit .env as needed for your setup (pagination, address, port, etc...)
Execute RUN.sh


```bash
mkdir /usr/share/rusicsetup
chmod 755 -R /usr/share/rusicsetup
chown pipi:pipi /usr/share/rusicsetup # change pipi to meet your needs
git clone https://github.com/cjsmocjsmo/rusicsetup.git
cd /usr/share/rusicsetup/rusicsetup
sh RUN.sh
```