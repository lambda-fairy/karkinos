#!/bin/sh

set -e

cd "$(readlink -f "$(dirname "$0")")"

if ! [ -d rustaceans.org ]
then
    git clone https://github.com/nrc/rustaceans.org.git
else
    cd rustaceans.org
    git fetch
    git checkout master
    git reset --hard origin/master
fi
