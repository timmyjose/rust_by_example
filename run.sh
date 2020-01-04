#!/bin/bash

file="$1"
filename="${file%.*}"

rustc -O ${file} && ./${filename}
rm -rf ${filename}