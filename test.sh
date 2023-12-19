#!/bin/bash
export H=/Users/zhangsan/.cargo/bin
export PATH=$PATH:$H
set -x


cargo tarpaulin --out Html --all-features --output-dir target/tarpaulin



cargo html --input-dir target/tarpaulin --output-dir target/tarpaulin

