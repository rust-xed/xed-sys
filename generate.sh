#!/usr/bin/bash

set -euo pipefail

cd "${0%/*}"
SCRIPT_DIR="$(pwd)"

# The first step we need to do is build a XED kit. XED doesn't provide a way
# to build just the headers but luckily they are quite consistent between
# different architectures.

cd xed
python3 -B mfile.py install         \
    --jobs=$(nproc)                 \
    --silent                        \
    --install-dir=../target/xed     \
    --build-dir=../target/xed-build
cd ..

# Now we generate the bindgen bindings. We use the experimental --wrap-static-fns
# flag in order to also generate bindings for static functions along with a
# source file that forwards to the C definitions.

BINDGEN_ARGS=(
    --rust-target           1.64
    --allowlist-type        'xed3?_.*'
    --allowlist-function    '(str2)?xed3?_.*'
    --allowlist-var         '(XED|xed)_.*'
    --blocklist-item        'XED_.*_DEFINED'
    --disable-header-comment
    --use-core
    --no-prepend-enum-name
    --impl-debug
    --experimental
    --wrap-static-fns
    --wrap-static-fns-path      src/xed-static.c
    --wrap-static-fns-suffix    _xed_sys_inline
)

bindgen xed.h -o src/bindings.rs    \
    "${BINDGEN_ARGS[@]}"            \
    --                              \
    -Itarget/xed/include
