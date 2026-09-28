# blkdemo

BORUIX 的对照诊断程序：前台阻塞读标准输入的旧字节路径。

[English](README.en.md)

## 用途

与 [`evdemo`](https://github.com/BRX-Boruix/evdemo) 配对使用。两者唯一的差别是等待源：
`evdemo` 读事件流节点，本程序读标准输入的旧字节路径。运行方式、输入与输出格式完全相同，
用于把「前台子进程阻塞时宿主 CPU 占用过高」的缺陷归属到具体路径。

两者占用形态一致，缺陷在等待机制之外；只有一方占用异常，缺陷归属那一方。

## 用法

在 shell 前台运行，按键，按 `q` 退出：

```
[blkdemo] blocking read on stdin (legacy byte path); press q to quit
[blkdemo] reads=4 bytes=4
[blkdemo] PASS
```

读到的字节不解释、不回显，只计数。

## 退出码

- `0`——读到 `q` 正常退出

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
    └── main.rs   # 阻塞读循环与计数
```

## 相关项目

- [`evdemo`](https://github.com/BRX-Boruix/evdemo) —— 事件路径的对照程序
- [`shell`](https://github.com/BRX-Boruix/shell) —— 前台运行环境

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。
