#!/bin/bash

# Get the PID of the current shell
pid=$$

# Retrieve the command associated with the PID
shell_name=$(ps -p "$pid" -o command --no-headers)

# Use the shell_name variable as needed
echo "Shell name: $shell_name"
