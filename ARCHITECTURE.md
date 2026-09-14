# Architecture

`scan(&Path) -> io::Result<ScanReport>` returns file count, total logical size,
and elapsed time. The core is read-only and independent of CLI output.

CLI: `cargo run -- scan <path>`.

```text
src/
  main.rs           Application entry point and result output
  cli.rs            clap argument definitions
  lib.rs            Public core interface
  scan.rs           Scan entry point
  filesystem.rs     Windows directory enumeration and metadata access
  model.rs          Scan report

tests/
  test.rs           Test code

benches/
  core.rs           Criterion entry point for scan benchmarks
```

The CLI calls `scan` through `lib.rs`. `scan` uses `filesystem` to read metadata
and updates totals without retaining individual file records.

Use Criterion to measure end-to-end scan time with:

```powershell
cargo bench --bench core
```

Peak process memory requires separate OS measurement; Criterion measures time.
