#!/bin/bash

if sudo -n true 2>/dev/null; then
    echo true
else
    echo false
fi
