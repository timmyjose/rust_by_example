#!/bin/bash

file="$1"
filename="${file%.*}"

shift
rustc -O ${file} && ./${filename} "$@"
rm -rf ${filename}