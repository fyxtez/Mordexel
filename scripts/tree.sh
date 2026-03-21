#!/usr/bin/env bash

# Go one directory up from where the script is located
cd "$(dirname "$0")/.." || exit 1

# Run tree ignoring unwanted folders
tree -I "target|node_modules|dist|.git"