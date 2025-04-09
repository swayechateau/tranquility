#!/bin/bash

# Define colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Define variables
declare -A VM_MAP  # Use an associative array for VM names and addresses
USERNAME=$USER

# Define VMs and their addresses
VM_MAP=(
  ["example1"]="vps.example.com"
  ["example2"]="vps.example2.com"
  ["example3"]="vps3.example.com"
)

# Check if user supplied a username as an argument with -u
if [[ "$1" == "-u" ]]; then
    echo -e "${GREEN}Username: $2${NC}"
    USERNAME="$2"
    shift 2 # Shift arguments to remove -u and username
else
    echo -e "${YELLOW}No username provided. Using default: $USERNAME${NC}"
fi

# Ask user which VM from the list to connect to
echo -e "${GREEN}Select a VM to connect to:${NC}"
select VM in "${!VM_MAP[@]}"; do # Use keys of the associative array
    if [[ "${!VM_MAP[@]}" =~ "$VM" ]]; then
        VM_ADDRESS="${VM_MAP[$VM]}"
        echo -e "${GREEN}Connecting to $VM ($VM_ADDRESS)...${NC}"
        ssh "$USERNAME@$VM_ADDRESS"
        break
    else
        echo -e "${RED}Invalid selection. Please try again.${NC}"
    fi
done