#!/usr/bin/env bash

cd "$(dirname "$0")/.." || exit 1

tree -I "target|node_modules|dist|.git" | tee >(xclip -selection clipboard)