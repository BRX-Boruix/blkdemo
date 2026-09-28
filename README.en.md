# blkdemo

A **control diagnostic program** for BORUIX: it blocks on standard input, used to localise the origin of a CPU consumption defect.

[简体中文](README.md)

## Why it exists

There was once a defect: with a foreground child process blocked, the host sat near 100% CPU. Deciding whether the problem lay in the **event node blocking path** or in **the shell's foreground wait loop** required a control that is "identical in everything except the source being waited on".

This program is that control: its **only** difference from [`evdemo`](https://github.com/BRX-Boruix/evdemo) is the **source**.

| Program | Waits on | Kernel waiter |
| --- | --- | --- |
| `evdemo` | The event node (event path) | Event waiter |
| **`blkdemo`** | **fd 0 (legacy byte path)** | Keyboard waiter |

The two behave identically otherwise. So if only one of them drives CPU up, the defect is localised.

## Behaviour

It reads bytes from standard input, **interpreting nothing and echoing nothing**, only counting, and printing the total on exit — avoiding extra noise that would disturb the observation.

## Honest boundary

This program is **not a product feature but a diagnostic tool**. All it proves or disproves is where the defect belongs; it changes no product behaviour.

## Usage

Started by the coordinating side and run in the foreground. Watch host CPU usage, then send `Ctrl-D` or terminate it and read the count.

## Building

```bash
cargo build --release
```

## Layout

```
blkdemo/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # blocking read and counting
```

## Related projects

- [`evdemo`](https://github.com/BRX-Boruix/evdemo) — the control for the event path
- [`libsys`](https://github.com/BRX-Boruix/libsys) — the user-space syscall wrapper

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
