#!/bin/sh

# Check architecture
arch=$(uname -m)
if [[ "$arch" == "x86_64" ]]; then
    arch="x64"
elif [[ "$arch" == *"arm"* ]]; then
    arch="ARM"
else
    arch="Unknown"
fi

echo $arch