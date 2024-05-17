#!/usr/bin/sh

cargo run src/test/conditional.te && cargo run src/test/variables.te && cargo run src/test/functions.te || echo "Test failed"
