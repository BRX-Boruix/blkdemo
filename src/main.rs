//! BORUIX blkdemo：**对照程序**——前台阻塞在 stdin（旧字节路径）。
//!
//! # 为什么存在（S24 单一变量对照）
//!
//! §6.13 的缺陷是「前台子进程阻塞时宿主 CPU ≈ 99%」。要判定它属于
//! **事件节点阻塞路径**还是**shell 的前台 wait 循环**，必须有一个「除了等待源
//! 之外一切都相同」的对照：本程序与 `evdemo` 的唯一差别是——
//! `evdemo` 读 `/devices/input/events`（事件路径，`IN_EVENT_WAITER`），
//! 本程序读 **fd 0**（旧字节路径，`KBD_WAITER`）。
//!
//! 读到的字节**不解释、不回显**，只计数并在退出时打印，避免额外噪声。
//!
//! 诚实边界（S09）：本程序**不是**产品功能，是诊断工具。
//! 它证明或证伪的只是「缺陷归属」，不改变任何产品行为。
#![no_std]
#![no_main]

use libsys::{read, write, STDOUT, STDIN};

fn out(b: &[u8]) {
    let _ = write(STDOUT, b);
}

fn out_hex(mut v: u64) {
    const HEX: &[u8] = b"0123456789abcdef";
    if v == 0 { out(b"0"); return; }
    let mut buf = [0u8; 16];
    let mut i = buf.len();
    while v > 0 {
        i -= 1;
        buf[i] = HEX[(v % 16) as usize];
        v /= 16;
    }
    out(&buf[i..]);
}

#[unsafe(no_mangle)]
pub extern "C" fn user_main(_argc: isize, _argv: *const *const u8) -> i32 {
    out(b"[blkdemo] blocking read on stdin (legacy byte path); press q to quit\n");
    let mut n_reads: u64 = 0;
    let mut n_bytes: u64 = 0;
    loop {
        let mut buf = [0u8; 64];
        // 阻塞读 fd 0：旧字节路径（内核登记 KBD_WAITER）。
        let n = match read(STDIN, &mut buf) {
            Ok(n) => n,
            Err(_) => {
                // 如实重试（与 shell 的 StdinSource 同纪律）。
                continue;
            }
        };
        n_reads += 1;
        n_bytes += n as u64;
        // 只在收到 q 时退出；其余字节静默吞掉（不产生输出噪声）。
        if buf[..n].contains(&b'q') {
            out(b"[blkdemo] reads=");
            out_hex(n_reads);
            out(b" bytes=");
            out_hex(n_bytes);
            out(b"\n[blkdemo] PASS\n");
            return 0;
        }
    }
}