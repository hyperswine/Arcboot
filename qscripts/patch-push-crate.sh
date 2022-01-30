#!/bin/bash
# apparently doesnt work

cargo bump patch
git add . && git commit -m "Updates" && git push
cargo publish
