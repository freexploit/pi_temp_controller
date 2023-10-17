#!/bin/sh

export PATH="/home/freexploit/code/gcc-arm/gcc-arm-8.3-2019.03-x86_64-aarch64-linux-gnu/bin/:$PATH"

PI_IP=192.168.1.247
TARGET=aarch64-unknown-linux-gnu
USERNAME=freexploit

# build binary
cargo build --target $TARGET --release
aarch64-linux-gnu-strip --strip-all ./target/$TARGET/release/pi_temp_controller

# upload binary
scp -r ./target/$TARGET/release/pi_temp_controller $USERNAME@$PI_IP:/home/freexploit

# execute binary
ssh $USERNAME@$PI_IP '~/pi_temp_controller --threshold 40 '
