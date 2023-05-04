#!/bin/bash
while read fname; do
  cp $fname ./lib
  echo "found libonnxruntime.so.1.8.1 and copied it!"
  break
done <<< "$(find ./target/release/build/ -name "libonnxruntime.so.1.8.1")"
