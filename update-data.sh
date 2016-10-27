#!/bin/sh

set -e

# Make sure we're in the right directory
# macOS doesn't support `readlink -f`, so we can't use it here :(
# cd "$(readlink -f "$(dirname "$0")")"
cd "$(dirname "$0")"

if ! [ -d rustaceans.org ]
then
    # Clone the data from GitHub
    git clone https://github.com/nrc/rustaceans.org.git
else
    # Update the existing repo
    cd rustaceans.org
    git fetch
    git checkout master
    git reset --hard origin/master
fi
