#!/usr/bin/env bash
set -euo pipefail

MYSELF=$(realpath "$0")
MYDIR=$(dirname "$MYSELF")

# cargo -Zunstable-options -C"$MYDIR" build --release

cargo -Zunstable-options -C"$MYDIR" b -r --target=.cargo/target-i386.json --package microloader

echo "run 'qemu-system-x86_64 -drive format=raw,file=target/target-i386/release/microloader' to test this"

# TODO: make a dos disk image
# write mbr and put os in a fs in it or smth
#

# dd if=/dev/zero of="$MYDIR/output.bin" bs=510 count=1 &>/dev/null
# printf "\x55\xAA" >> "$MYDIR/output.bin"
# dd if="$MYDIR/target/target-amd64/release/microloader" of="$MYDIR/output.bin" conv=notrunc &>/dev/null
