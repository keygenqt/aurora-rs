#!/bin/bash

# Project dir
cd "$(dirname "$(realpath "$0")")" || exit

VAR_CLI="aurora-cli"

# Check OS
if [[ "$OSTYPE" == "linux"* ]]; then
  # path to binary or install
  # VAR_CLI="$HOME/Documents/Gitcode/Projects/PROJECT/aurora-cli/target/debug/aurora-cli"
  VAR_ARCH="x86_64"
  VAR_TARGET="x86_64-unknown-linux-gnu"
fi

if [[ "$OSTYPE" == "darwin"* ]]; then
  # path to binary or install
  # VAR_CLI="$HOME/Documents/Gitcode/Projects/PROJECT/aurora-cli/target/debug/aurora-cli"
  VAR_ARCH="aarch64"
  VAR_TARGET="aarch64-unknown-linux-gnu"
fi

if [[ -z "$VAR_ARCH" ]]; then
  echo "Not supported $OSTYPE"
fi

# Clear build
rm -rf target
rm -rf rpm/RPMS
rm -rf rpm/bin

# Check sysroot
if [ ! -d "$HOME/.aurora-sysroot/$VAR_ARCH/usr/include" ]; then
  $VAR_CLI services --sysroot || exit
fi

# Moc Qt files
$VAR_CLI services --moc ./ || exit

# Install cross
# https://github.com/cross-rs/cross
cargo install cross --git https://github.com/cross-rs/cross

# Build
CROSS_CONFIG="Cross.local.toml" cross build --target="$VAR_TARGET" --verbose || exit

# Local build
# SYS="$HOME/.aurora-sysroot/x86_64"
# export PKG_CONFIG_SYSROOT_DIR=$SYS
# export PKG_CONFIG_PATH=$SYS/usr/lib/pkgconfig
# export PKG_CONFIG_LIBDIR=$SYS/usr/lib
# export LD_LIBRARY_PATH=$SYS/usr/lib
# cargo build --target="$VAR_TARGET" --verbose || exit
# unset PKG_CONFIG_SYSROOT_DIR
# unset PKG_CONFIG_PATH
# unset PKG_CONFIG_LIBDIR
# unset LD_LIBRARY_PATH

# Copy bin
mkdir rpm/bin
cp -fr $PWD/target/$VAR_TARGET/debug/aurora-rs $PWD/rpm/bin || exit

# Build rpm
$VAR_CLI services --rpmbuild ./rpm || exit

# Sign rpm
$VAR_CLI services --keysign ./rpm/RPMS/$VAR_ARCH/com.keygenqt.aurora_rs-0.0.1-1.$VAR_ARCH.rpm || exit

# Remove old rmp
ssh -o ConnectTimeout=2 \
  -o StrictHostKeyChecking=no \
  -p 2223 defaultuser@localhost \
  -i $HOME/AuroraOS/vmshare/ssh/private_keys/sdk \
  "rm /home/defaultuser/Downloads/com.keygenqt.aurora_rs-0.0.1-1.$VAR_ARCH.rpm"

# Upload
scp -P 2223 \
  -i $HOME/AuroraOS/vmshare/ssh/private_keys/sdk \
  ./rpm/RPMS/$VAR_ARCH/com.keygenqt.aurora_rs-0.0.1-1.$VAR_ARCH.rpm \
  defaultuser@localhost:/home/defaultuser/Downloads

# Uninstall
ssh -o ConnectTimeout=2 \
  -o StrictHostKeyChecking=no \
  -p 2223 defaultuser@localhost \
  -i $HOME/AuroraOS/vmshare/ssh/private_keys/sdk \
  "gdbus call --system --dest ru.omp.APM --object-path /ru/omp/APM --method ru.omp.APM.Remove com.keygenqt.aurora_rs '{}'"


# Install
ssh -o ConnectTimeout=2 \
  -o StrictHostKeyChecking=no \
  -p 2223 defaultuser@localhost \
  -i $HOME/AuroraOS/vmshare/ssh/private_keys/sdk \
  "gdbus call --system --dest ru.omp.APM --object-path /ru/omp/APM --method ru.omp.APM.Install ~/Downloads/com.keygenqt.aurora_rs-0.0.1-1.$VAR_ARCH.rpm '{}'" || exit

# Run
ssh -o ConnectTimeout=2 \
  -o StrictHostKeyChecking=no \
  -p 2223 defaultuser@localhost \
  -i $HOME/AuroraOS/vmshare/ssh/private_keys/sdk \
  "runtime-manager-tool Control startDebug com.keygenqt.aurora_rs --output-to-console" || exit
