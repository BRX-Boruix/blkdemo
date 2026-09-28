# blkdemo

A BORUIX control/diagnostic program: foreground blocking reads on the legacy stdin byte path.

[简体中文](README.md)

## Purpose

Used in pairs with [`evdemo`](https://github.com/BRX-Boruix/evdemo). The two differ in exactly one
variable: `evdemo` reads the event stream node, this program reads legacy bytes on standard input.
Invocation, input and output formats are identical, so a "host CPU pinned while a foreground child
blocks" defect can be attributed to a specific path.

If both show the same profile the defect is outside the wait mechanism; if only one pins the CPU,
the defect belongs to that one.

## Usage

Run in the shell foreground, press keys, quit with `q`:

```
[blkdemo] blocking read on stdin (legacy byte path); press q to quit
[blkdemo] reads=4 bytes=4
[blkdemo] PASS
```

Read bytes are neither interpreted nor echoed, only counted.

## Exit codes

- `0` — quit normally on `q`

## Building

```bash
cargo build --release
```

## Repository layout

```
blkdemo/
├── Cargo.toml    # package manifest
├── build.rs      # injects the linker script
├── linker.ld     # user-space segment layout
└── src/
    └── main.rs   # blocking read loop and counting
```

## Related projects

- [`evdemo`](https://github.com/BRX-Boruix/evdemo) — the event-path counterpart
- [`shell`](https://github.com/BRX-Boruix/shell) — the foreground host

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
