# blkdemo

**简体中文** | [English](#english)

BORUIX 上用于**定位 CPU 占用缺陷归属**的对照实验程序。

当发现"前台子进程阻塞时宿主 CPU 占用接近 100%"这个现象时，需要回答一个关键问题：**CPU 是被
谁烧掉的？** 是内核里等待键盘输入的那条路径，还是 shell 等待前台进程的那个循环？

单看一个程序无法回答。要归因，就必须有一个**除了等待源之外一切都相同**的对照组。

---

## 设计：单一变量对照

`blkdemo` 与 [`evdemo`](https://github.com/BRX-Boruix/evdemo) 构成一组对照，两者**唯一的差别
是等待的数据来源**：

| 程序 | 等待源 | 内核路径 |
| --- | --- | --- |
| `evdemo` | `/devices/input/events` 事件节点 | 事件路径 |
| `blkdemo` | **标准输入 fd 0**（传统字节路径） | 字节路径 |

其余完全相同：都是阻塞读、都不做自旋、都由同一个 shell 以前台方式启动。

## 判据

三个场景各自测量宿主进程的 CPU 占用，唯一变量是"前台子进程阻塞在什么上"：

| 场景 | 含义 | 修复前实测 | 修复后实测 |
| --- | --- | --- | --- |
| shell 空转 | 无子进程（基线） | 6.4% | 4.4% |
| `blkdemo` | 阻塞在 stdin（字节路径） | 100.0% | 21.6% |
| `evdemo` | 阻塞在事件节点（事件路径） | 99.6% | 23.1% |

**结论**：两条路径**同为 ~100%**，说明缺陷与等待源无关，真因在 shell 的前台等待循环。

修复后残余的 18–27% 不是缺陷：shell 使用有界等待（10 ms 一片）以便在前台子进程运行期间仍能
探测 `^C`，因此每秒醒来约 100 次；采样证实 CPU 停机在 `halt` 指令。

## 它做什么

- 阻塞读 fd 0，读到的字节**不解释、不回显**，只计数
- 按 `q` 退出，打印读次数与字节数
- 读取出错时如实重试，不静默吞错

这个程序刻意保持极简：任何多余行为（回显、解析、格式化）都会给 CPU 测量引入噪声，而它唯一的
用途就是让 CPU 占用数字可比较。想看到按键回显，请用 `evdemo`。

## 这不是产品功能

`blkdemo` 是一个**诊断工具**，不是 BORUIX 的功能组件。它证明或证伪的只是"缺陷归属"，不改变
任何产品行为。

它被保留下来，是因为它让那次归因**可复现**——配套的验收脚本会重新运行它，而不是只留下一个
"当时量过"的说法。

## 构建

```bash
cargo build --release
```

编译产物部署为 BORUIX 系统中的 `/programs/blkdemo.elf`，然后在 shell 中执行。

## 文件结构

```
blkdemo/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 程序本体
```

## 相关项目

- [`evdemo`](https://github.com/BRX-Boruix/evdemo) —— 本程序的对照组，走事件路径
- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 用户态系统调用封装

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。

---

# English

[简体中文](#blkdemo) | **English**

A controlled-experiment program for **attributing a CPU-usage defect** on BORUIX.

On finding that "host CPU sits near 100% while a foreground child blocks", the key question is:
**who is burning the CPU?** Is it the kernel path waiting for keyboard input, or the shell's loop
waiting on the foreground process?

No single program can answer that. Attribution requires a **control that differs only in what it
waits on**.

---

## Design: a single-variable control

`blkdemo` and [`evdemo`](https://github.com/BRX-Boruix/evdemo) form a pair whose **only difference
is the data source they wait on**:

| Program | Waits on | Kernel path |
| --- | --- | --- |
| `evdemo` | the `/devices/input/events` event node | event path |
| `blkdemo` | **standard input, fd 0** (the legacy byte path) | byte path |

Everything else is identical: both block on read, neither spins, and both are launched in the
foreground by the same shell.

## The criterion

Host CPU usage is measured in three scenarios, with the only variable being what the foreground
child blocks on:

| Scenario | Meaning | Before the fix | After the fix |
| --- | --- | --- | --- |
| shell idle | no child process (baseline) | 6.4% | 4.4% |
| `blkdemo` | blocked on stdin (byte path) | 100.0% | 21.6% |
| `evdemo` | blocked on the event node (event path) | 99.6% | 23.1% |

**Conclusion**: both paths measured **~100%**, showing the defect was unrelated to the wait source
and actually lay in the shell's foreground-wait loop.

The residual 18-27% after the fix is not a defect: the shell uses a bounded wait (10 ms slices) so
it can still notice `^C` while a foreground child runs, waking roughly 100 times per second.
Sampling confirmed the CPU stops in the `halt` instruction.

## What it does

- Blocks on a read from fd 0; bytes read are **neither interpreted nor echoed**, only counted
- Exits on `q`, printing the read count and byte count
- Retries honestly on read errors rather than silently swallowing them

The program is deliberately minimal: any extra behaviour (echoing, parsing, formatting) would inject
noise into the CPU measurement, and its only purpose is to make the CPU numbers comparable. To see
keystrokes echoed back, use `evdemo`.

## This is not a product feature

`blkdemo` is a **diagnostic tool**, not a BORUIX component. It only proves or disproves where a
defect belongs; it changes no product behaviour.

It is kept because it makes that attribution **reproducible** — the accompanying acceptance script
re-runs it, rather than leaving behind only a claim that it was measured once.

## Building

```bash
cargo build --release
```

The artifact is deployed as `/programs/blkdemo.elf` in a BORUIX system, then run from the shell.

## Layout

```
blkdemo/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # the program itself
```

## Related projects

- [`evdemo`](https://github.com/BRX-Boruix/evdemo) — this program's control, using the event path
- [`libsys`](https://github.com/BRX-Boruix/libsys) — the user-space syscall wrapper

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
