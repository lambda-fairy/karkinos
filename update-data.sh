#!/bin/sh

set -e

cd "$(readlink -f "$(dirname "$0")")"

if ! [ -d rustaceans.org ]
then
    git clone https://github.com/nrc/rustaceans.org.git
    ln -f -s rustaceans.org/data data
else
    cd data
    git fetch
    git checkout master
    git reset --hard origin/master
fi
