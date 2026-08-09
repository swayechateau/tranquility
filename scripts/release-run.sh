#!/bin/bash

# DEV_SETUP=./target/debug/tranquility
PROD_SETUP=./target/release/tranquility

tranquility() {
  echo "Running tranquility..."
  $PROD_SETUP "$@"
}

# Check if the tranquility executable exists
if [ ! -f "$PROD_SETUP" ]; then
  echo "Error: tranquility executable not found. Running the './build' command."
  ./build
fi

# Call tranquility with passed arguments
tranquility "$@"
