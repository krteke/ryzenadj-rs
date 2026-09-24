#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")"

bindgen wrapper.h \
    --allowlist-function 'init_ryzenadj|cleanup_ryzenadj|init_table|refresh_table|get_.*|set_.*' \
    --allowlist-type 'ryzen_family|ryzen_access|_ryzen_access' \
    --allowlist-var 'RYZENADJ_.*|ADJ_ERR_.*' \
    --opaque-type '_ryzen_access' \
    --no-layout-tests \
    --output src/bindings.rs \
    -- \
    -Ivendor/RyzenAdj/lib

rustfmt src/bindings.rs
