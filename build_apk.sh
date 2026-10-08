#!/usr/bin/env bash
# Build the Android APK using the notfl3/cargo-apk container (bundles Android SDK + NDK).
# Output lands in target/android-artifacts/release/apk/
set -euo pipefail
cd "$(dirname "$0")"

# The container ships an older Cargo that can't read the host's v4 Cargo.lock,
# so stash the host lockfile for the duration of the container build.
if [ -f Cargo.lock ]; then
    mv Cargo.lock Cargo.lock.host
    trap 'mv -f Cargo.lock.host Cargo.lock' EXIT
fi

podman run --rm \
    -v "$PWD":/root/src:Z \
    -w /root/src \
    docker.io/notfl3/cargo-apk \
    sh -c 'cargo quad-apk build --release; rm -f Cargo.lock'

echo
echo "APK(s) built:"
find target/android-artifacts -name '*.apk' 2>/dev/null
