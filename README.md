# ryzenadj-rs

Rust bindings and a wrapper for the bundled RyzenAdj C library:

- `ryzenadj-sys` builds the C library and exposes its raw FFI.
- `ryzenadj` owns the handle, converts errors, and provides typed controls and
  borrowed access to the PM Table cache.

## Build and run the reading example

The default build requires Rust, the RyzenAdj submodule, and these native tools:

| Target              | Requirements                                                                                                        |
| ------------------- | ------------------------------------------------------------------------------------------------------------------- |
| x86_64 Linux        | C compiler, `pkg-config`, and libpci headers/library discoverable as `libpci`                                       |
| x86_64 Windows MSVC | MSVC C/C++ tools and the bundled WinRing0x64 import library; WinRing0x64 runtime components are needed when running |

Other architectures and Windows GNU targets are not supported by the build script.
See the [Windows limitation](#windows-limitation) before running on Windows.

From the workspace root:

```sh
git submodule update --init
cargo build -p ryzenadj --example inspect
```

```sh
cargo run -p ryzenadj --example inspect

# or use root at Linux
cargo build -p ryzenadj --example inspect
sudo ./target/debug/examples/inspect
```

The [inspect example](ryzenadj/examples/inspect.rs) prints CPU information and PM
Table readings, explicitly refreshes once, and reads STAPM again. It calls query and refresh
APIs only. PM Table refresh may send SMU commands to request a table transfer.
Errors propagate from `main`, producing a nonzero exit status.

## Handle and PM Table lifetime

The `ryzenadj` safe wrapper permits one live `RyzenAdj` handle at a time.
`RyzenAdj::new()` returns `Error::AlreadyInUse` while another handle exists.
Dropping the handle calls C cleanup and releases the wrapper's instance guard.
This restriction is local to the process and does not cover direct calls to
`ryzenadj-sys`.

`new()` creates the handle. `power_table()` refreshes the C cache, initializing it
on demand, and returns a `PowerTable` that exclusively borrows the handle. Reading
methods use that cache; call `refresh()` to update it explicitly. Setters do not
refresh the cache automatically. End the table borrow before using the handle
for another operation, as demonstrated by the example's inner scope.

`values()` returns a borrowed slice of raw cached floats. While that slice is still being
used, Rust prevents refreshing the table. Structured readings such as `PowerLimitReading`
are owned copies and can be kept after refreshing or releasing the table borrow.

## Values, units, and core indices

Structured readings turn each NaN field into `None` independently.
The raw slice from `values()` preserves NaNs.

| Quantity              | Setter input            | Reading unit    |
| --------------------- | ----------------------- | --------------- |
| Power limit           | `types::Milliwatts`     | Watts           |
| TDC/EDC current limit | `types::Milliamps`      | Amperes         |
| Temperature limit     | `types::DegreesCelsius` | Degrees Celsius |
| STAPM/slow PPT time   | `types::Seconds`        | Seconds         |

`PowerTable::core(index)` takes a PM Table slot index. Per-core controls instead
take `types::CoreAddress`, containing firmware CCD/CCX/core indices. Neither API
translates OS logical CPU numbers. `CoreAddress::new` checks the encoding width,
not whether a core exists. PM Table getters also inherit C's incomplete core
count checks, documented on `PowerTable::core`.

## Windows limitation

The upstream C Windows backend can dereference an uninitialized PM table mapping
when cleaning up a handle. In particular, dropping a handle before accessing its
PM table, or some initialization failures, may crash.

## STAPM time limitation

For several CPU families, the upstream C `set_stapm_time()` switch falls through
after its first SMU request and sends a second request. The returned status may
describe that second request. `RyzenAdj::set_stapm_time()` preserves this upstream
behavior until it is fixed in C.
