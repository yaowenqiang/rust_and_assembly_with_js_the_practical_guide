# snake_rust — Rust 终端贪吃蛇

纯 Rust 实现，唯一依赖 `libc`（用于 raw 终端模式和非阻塞按键读取），构建走离线模式（`.cargo/config.toml` 里 `offline=true`）。

- 棋盘大小**随终端窗口自适应**（每格 2 列 × 1 行，留边框和状态栏），小终端也有下限保护
- 速度：初始 200ms/步，每吃 1 个食物加快 2ms，最快 100ms/步

## 操作

| 按键 | 功能 |
|---|---|
| ↑ ↓ ← → / W A S D | 控制方向 |
| P | 暂停 / 继续 |
| R | 重新开始 |
| Q / Esc / Ctrl+C | 退出 |

## 构建 & 运行

```sh
cargo build --release
./target/release/snake_rust
# 或在线环境：
cargo run --release
```

## 测试（10 个单元测试，覆盖核心规则）

```sh
cargo test
```

覆盖内容：

- 初始方向 / 合法转向 / 180° 反向输入被忽略
- 同帧内连按两次方向键只有最后一次生效
- 吃食物：变长、加分、新食物不落在蛇身上
- 撞墙死亡 / 撞身体死亡
- **追尾判定**：头撞向"即将让开"的尾巴不算死
- 速度随分数提升且有下限（同时修掉了 u64 下溢 bug，用 `saturating_sub`）
- 食物生成 100 次都不落在蛇身上

## 实现要点

- `libc::tcgetattr/tcsetattr + cfmakeraw` 进 raw 模式，`VMIN=0/VTIME=0` 非阻塞读
  按键；`Drop` 保证退出时恢复终端（即使 panic 也会恢复）。
- 方向键是 `ESC [ A/B/C/D` 序列，`read_key` 里做字节级解析。
- 伪随机用 SplitMix64 风格混淆，确定性播种方便写单元测试。
- 渲染用 ANSI 转义 + 备用屏幕（`?1049h/l`），每帧重绘。

## 文件

- `Cargo.toml`
- `src/main.rs` — 游戏逻辑、终端层、渲染、测试全在一个文件（约 500 行，便于阅读）
