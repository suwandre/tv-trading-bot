#!/bin/bash
# Used to check if the app is running in the correct directory (in Railway)
echo "Current working directory: $(pwd)"
echo "Contents of /app/target/release:"
ls -l /app/target/release
echo "Starting the app..."
/app/target/release/tv-trading-bot