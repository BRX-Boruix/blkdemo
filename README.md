# blkdemo

BORUIX 的**对照诊断程序**：阻塞在标准输入上，用于定位一个 CPU 占用缺陷的归属。

[English](README.en.md)

## 为什么存在

曾有缺陷：前台子进程阻塞时，宿主的 CPU 占用接近 100%。要判定问题出在**事件节点的阻塞路径**还是
**shell 的前台等待循环**，需要一个「除了等待来源之外一切都相同」的对照。

本程序就是那个对照：它与 [`evdemo`](https://github.com/BRX-Boruix/evdemo) 的**唯一差别**是等待**来源**不同。

| 程序 | 等待来源 | 内核等待者 |
| --- | --- | --- |
| `evdemo` | 事件节点（事件路径） | 事件等待者 |
| **`blkdemo`** | **fd 0（旧字节路径）** | 键盘等待者 |

两者行为在其他方面完全一致。因此若只有其中一个推高 CPU，缺陷归属就确定了。

## 行为

从标准输入读字节，**不解释、不回显**，只计数，退出时打印总数——避免制造额外噪声干扰观察。

## 诚实边界

本程序**不是产品功能，是诊断工具**。它证明或证伪的只是缺陷归属，不改变任何产品行为。

## 用法

由协调端拉起，在前台运行。观察宿主 CPU 占用，然后按 `Ctrl-D` 或终止它，读取计数。

## 构建

```bash
cargo build --release
```

## 文件结构

```
blkdemo/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 阻塞读取与计数
```

## 相关项目

- [`evdemo`](https://github.com/BRX-Boruix/evdemo) —— 事件路径的对照程序
- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 用户态系统调用封装

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。
