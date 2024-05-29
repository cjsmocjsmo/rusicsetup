# rusicsetup

![Project Screenshot](screenshot.png "width=450px")

rusicsetup is part of the rusic-svelte and rusic trio.


## Prerequisites

1. raspberrypiOS (bookworm)
2. docker


## Usage

Git clone this repository to any folder on your box.
Execute RUN.sh, it will build a docker contianer for the architecture you specify.

```bash
git clone https://github.com/cjsmocjsmo/rusic-svelte.git

#for the rpi3
sh RUN.sh 32 0.0.1

# or if on the rpi4 and above

sh RUN.sh 64 0.0.1
```