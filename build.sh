if [ -z "$1" ]; then
    echo "Error: missing version argument." >&2
    echo "Usage: ./build.sh <version>" >&2
    exit 1
fi

if [ -f "./Cargo.toml" ]; then
    cargo build --release
    cargo build --release --target armv7-unknown-linux-gnueabihf
else
    echo "Cargo.toml not found. Please run this script from the root of your Rust project."
    exit 1
fi

ARM64=./target/release/rusicsetup
ARM32=./target/armv7-unknown-linux-gnueabihf/release/rusicsetup
ENV=./.env

if [ -f "../rusic/setup/rusicsetup-rpi4-*" ]; then
    echo "Removing existing ARM64 binary from setup directory..."
    rm ../rusic/setup/rusicsetup-rpi4-*
fi

if [ -f "../rusic/setup/rusicsetup-rpi3b-*" ]; then
    echo "Removing existing ARM32 binary from setup directory..."
    rm ../rusic/setup/rusicsetup-rpi3b-*
fi

if [ -f "../rusic/setup/.env" ]; then
    echo "Removing existing .env file from setup directory..."
    rm ../rusic/setup/.env
fi

if [ -f "$ARM64" ]; then
    echo "Copying ARM64 binary to current directory..."
    cp "$ARM64" ./rusicsetup-rpi4-"$1"
    cp "$ARM64" ../rusic/setup/rusicsetup-rpi4-"$1"
else
    echo "ARM64 binary not found. Please ensure it was built successfully."
fi

if [ -f "$ARM32" ]; then
    echo "Copying ARM32 binary to current directory..."
    cp "$ARM32" ./rusicsetup-rpi3b-"$1"
    cp "$ARM32" ../rusic/setup/rusicsetup-rpi3b-"$1"
else
    echo "ARM32 binary not found. Please ensure it was built successfully."
fi

if [ -f "$ENV" ]; then
    echo "Copying .env file to setup directory..."
    cp "$ENV" ../rusic/setup/.env
else
    echo ".env file not found. Please ensure it exists in the root of your project."
fi