# ryzenadj-rs

The `ryzenadj` safe wrapper permits one live `RyzenAdj` handle at a time.
`RyzenAdj::new()` returns `Error::AlreadyInUse` while another handle exists.
This restriction does not cover direct calls to `ryzenadj-sys`.

## Windows limitation

The upstream C Windows backend can dereference an uninitialized PM table mapping
when cleaning up a handle. In particular, dropping a handle before accessing its
PM table, or some initialization failures, may crash.

## STAPM time limitation

For several CPU families, the upstream C `set_stapm_time()` switch falls through
after its first SMU request and sends a second request. The returned status may
describe that second request. `RyzenAdj::set_stapm_time()` preserves this upstream
behavior until it is fixed in C.
