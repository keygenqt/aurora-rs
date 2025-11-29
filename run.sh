#!/bin/bash

# Project dir
cd "$(dirname "$(realpath "$0")")" || exit

# Set docker image build tools
# export AURORA_CLI_IMAGE="aurora-build-tools:5.2.0.70"

# Set aurora-cli
VAR_CLI="aurora-cli"
VAR_CLI="/Users/keygenqt/Documents/Gitcode/Projects/aurora-cli/target/debug/aurora-cli"

# Check OS
if [[ "$OSTYPE" == "linux"* ]]; then
  VAR_ARCH="x86_64"
  VAR_TARGET="x86_64-unknown-linux-gnu"
  VAR_SDK="$HOME/AuroraOS"
fi

if [[ "$OSTYPE" == "darwin"* ]]; then
  VAR_ARCH="aarch64"
  VAR_TARGET="aarch64-unknown-linux-gnu"
  VAR_SDK="$HOME/AuroraOS"
fi

if [[ "$OSTYPE" == "msys"* ]]; then
  VAR_ARCH="x86_64"
  VAR_TARGET="x86_64-unknown-linux-gnu"
  VAR_SDK="$HOME/../../AuroraOS"
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

# # Moc Q_OBJECT files
$VAR_CLI services --moc ./ || exit

# Install cross
# https://github.com/cross-rs/cross
cargo install cross --git https://github.com/cross-rs/cross

# Build
CROSS_CONFIG="Cross.local.toml" cross build --target="$VAR_TARGET" --verbose || exit

# Copy bin
mkdir rpm/bin && cp -fr $PWD/target/$VAR_TARGET/debug/aurora-rs $PWD/rpm/bin || exit

# Build rpm
$VAR_CLI services --rpmbuild "$PWD/rpm" || exit

# Validate
$VAR_CLI services --validate "$PWD/rpm/RPMS/$VAR_ARCH/com.keygenqt.aurora_rs-0.0.1-1.$VAR_ARCH.rpm" || exit

# Sign rpm
$VAR_CLI services --keysign "$PWD/rpm/RPMS/$VAR_ARCH/com.keygenqt.aurora_rs-0.0.1-1.$VAR_ARCH.rpm" || exit

# Install on emulator
$VAR_CLI emulator --install "$PWD/rpm/RPMS/$VAR_ARCH/com.keygenqt.aurora_rs-0.0.1-1.$VAR_ARCH.rpm" || exit

# Run on emulator
$VAR_CLI emulator --run "com.keygenqt.aurora_rs" || exit
