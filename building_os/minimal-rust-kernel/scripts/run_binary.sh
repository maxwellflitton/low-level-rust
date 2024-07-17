#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

cargo rustc -- -C link-args="-e __start -static -nostartfiles"
./target/debug/freestanding-binary
