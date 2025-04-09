check_os() {
    # Check operating system
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "Linux"
        return 0
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "MacOS"
        return 0
    elif [[ "$OSTYPE" == "cygwin" || "$OSTYPE" == "msys" ]]; then
        echo "Windows"
        return 0
    fi

    echo "Unknown"
    return 1
}