#!/usr/bin/env bash
set -euo pipefail

MYSELF=$(realpath "$0")
MYDIR=$(dirname "$MYSELF")


BOOTPARTDIR="$MYDIR/etc/partitions/boot"
rm -rf "$BOOTPARTDIR"
mkdir -p "$BOOTPARTDIR"
echo "test file :P" >> "$BOOTPARTDIR/FILE.TXT"
echo "and another file lol" >> "$BOOTPARTDIR/FILE2.TXT"


# according to chatGPT, min DOS fs with a fat32 fs is 32MB, just double that :P
# and no sparse, not worth for these sizes
BINOUT="$MYDIR/output.img"
RELBINOUT=$(realpath -m --relative-to="$PWD" "$BINOUT")
dd status=none if=/dev/zero of="$BINOUT" bs=1M count=64
parted "$BINOUT" --script \
    mklabel msdos \
    mkpart primary fat32 1MiB 100% \
    set 1 boot on


# some bold assumptions
BOOTOFFSET=2048
SECTORSIZE=512
mkfs.fat -F 32 -n "BOOT" "$BINOUT" \
    --offset $(("$BOOTOFFSET" / "$SECTORSIZE")) > /dev/null # offset on sectors ._.
mcopy -i "$BINOUT"@@"$BOOTOFFSET" -s "$BOOTPARTDIR"/* ::/


cargo -Zunstable-options -C"$MYDIR" b -r --target=.cargo/target-i386.json --package microloader
# cant override partition entries.... dos fs carries that and the boot signature tho
dd status=none if="$MYDIR/target/target-i386/release/microloader" bs=446 count=1 of="$BINOUT" conv=notrunc


echo "'qemu-system-x86_64 -drive format=raw,file=\"$RELBINOUT\"' to run this in a VM"
