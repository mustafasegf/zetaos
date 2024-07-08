#! /bin/sh
#
# This script will be executed by `cargo run`.

set -xe

LIMINE_GIT_URL="https://github.com/limine-bootloader/limine.git"

# Cargo passes the path to the built executable as the first argument.
KERNEL=$1

# Clone the `limine` repository if we don't have it yet.
if [ ! -d target/limine ]; then
  git clone $LIMINE_GIT_URL --depth=1 --branch v7.x-binary target/limine
fi

# Make sure we have an up-to-date version of the bootloader.
cd target/limine
git fetch
make
cd -

# Copy the needed files into an ISO image.
mkdir -p target/iso_root/boot
cp "$KERNEL" target/iso_root/boot

mkdir -p target/iso_root/boot/limine
cp limine.cfg target/limine/limine{-bios.sys,-bios-cd.bin,-uefi-cd.bin} target/iso_root/boot/limine

# Create the EFI boot tree and copy Limine's EFI executables over.
mkdir -p target/iso_root/EFI/BOOT
cp target/limine/{BOOTX64.EFI,BOOTIA32.EFI} target/iso_root/EFI/BOOT

# Create the bootable ISO.
xorriso -as mkisofs \
  -b boot/limine/limine-bios-cd.bin \
  -no-emul-boot -boot-load-size 4 -boot-info-table \
  --efi-boot boot/limine/limine-uefi-cd.bin \
  -efi-boot-part --efi-boot-image --protective-msdos-label \
  target/iso_root -o "$KERNEL".iso

# Install Limine stage 1 and 2 for legacy BIOS boot.
target/limine/limine bios-install "$KERNEL".iso

# Run the created image with QEMU.
qemu-system-x86_64 \
  -machine q35 -cpu qemu64 -M smm=off \
  -D target/log.txt -d int,guest_errors \
  -no-reboot -no-shutdown \
  -serial stdio \
  -boot d \
  -bios "$OVMF" \
  -drive file="$KERNEL".iso,format=raw
