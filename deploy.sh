#!/bin/bash
# Print the working directory
echo "Current working directory: $(pwd)"

# List all directories in /app
echo "Contents of /app:"
ls -l /app

# List all directories recursively
echo "Complete directory tree:"
find /app

# Attempt to run the binary
echo "Trying to start the app..."
/app/target/release/tv-trading-bot