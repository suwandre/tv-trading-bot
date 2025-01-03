#!/bin/bash
# Issues with Railway CI not building the binary correctly means that we need to debug the build process
# and run the binary manually to confirm that it works as expected.
echo "Starting debug process..."

# Run build manually to confirm output
echo "Running cargo build --release --target-dir /app/target"
cargo build --release --target-dir /app/target

# Check if binary exists
echo "Checking for binary in /app/target/release:"
ls -l /app/target/release

# Attempt to execute the binary
if [ -f /app/target/release/tv-trading-bot ]; then
    echo "Binary found! Starting the app..."
    /app/target/release/tv-trading-bot
else
    echo "Binary not found! Build step may have failed."
    exit 1
fi