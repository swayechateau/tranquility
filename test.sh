#!/bin/bash

# show server name
echo "Server name is: $(hostname)"
# display the current date and time
echo "Current date and time is: $(date)"
# display the current user
echo "Current user is: $(whoami)"
# display the current directory
echo "Current directory is: $(pwd)"
# display the IP address of the server
echo "IP address of the server is: $(hostname -I)"
echo -e "\n"
# display the memory usage
echo "Memory usage is: $(free -h)"
echo -e "\n"
# display the disk usage
echo "Disk usage is: $(df -h)"
echo -e "\n"
# display the number of users currently logged in
echo "Number of users currently logged in: $(who | wc -l)"
echo -e "\n"
# display the top 5 processes by memory usage
echo "Top 5 processes by memory usage are: $(ps aux --sort -rss | head -n 6)"
echo -e "\n"
# display the top 5 processes by CPU usage
echo "Top 5 processes by CPU usage are: $(ps aux --sort -pcpu | head -n 6)"
echo -e "\n"
# display the number of files in the current directory
echo "Number of files in the current directory is: $(ls -l | wc -l)"
# display the number of directories in the current directory
echo "Number of directories in the current directory is: $(find . -maxdepth 1 -type d | wc -l)"