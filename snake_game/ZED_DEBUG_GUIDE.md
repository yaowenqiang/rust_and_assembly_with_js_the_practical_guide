# 🎯 Zed 编辑器 Rust 调试完整指南

## 📋 设置步骤

### 1. 确保 Rust 工具链安装
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | source /dev/stdin

# 安装 LLDB 调试器
brew install llvm
```

### 2. 重启 Zed
创建配置文件后，重启 Zed 以加载新配置。

## 🚀 使用 Zed 调试

### 方法1: 调试当前文件 (推荐)
1. 打开 `main.rs` 文件
2. 在想要断点的行号旁边点击，出现红点 🔴
3. 按 `Cmd + Shift + D` 打开调试面板
4. 选择 "Debug Current File" 配置
5. 点击绿色播放按钮 ▶️ 或按 `F5`

### 方法2: 调试编译后的二进制
1. 先编译：`cargo build`
2. 选择 "Debug Binary (main)" 配置
3. 开始调试

### 方法3: 直接调试现有二进制
1. 选择 "Debug with LLDB (Direct)" 配置
2. 直接调试 `main_debug_v2` 文件

## 🎮 调试控制

### 快捷键 (macOS)
- **F5**: 开始/继续调试
- **Shift + F5**: 停止调试
- **F10**: 单步跳过 (Step Over)
- **F11**: 单步进入 (Step Into)
- **Shift + F11**: 单步跳出 (Step Out)
- **Cmd + Shift + D**: 打开/关闭调试面板

### 断点操作
- **点击行号**: 设置/删除断点
- **右键点击**: 条件断点、日志断点
- **红色圆点**: 激活的断点
- **空心圆点**: 禁用的断点

## 🔍 调试面板功能

### 变量查看
- **局部变量**: 自动显示当前作用域的所有变量
- **监视表达式**: 添加自定义表达式监视
- **悬停查看**: 鼠标悬停变量显示值

### 调用堆栈
- 显示函数调用链
- 点击不同帧查看上下文

### 调试控制台
- 执行 LLDB 命令
- 查看程序输出
- 输入表达式求值

## 🎯 实用调试技巧

### 1. 设置条件断点
```rust
// 只在特定条件下停止
for i in 0..100 {
    println!("{}", i);  // 设置条件：i == 50
}
```
右键点击断点 → "Edit Breakpoint" → 添加条件 `i == 50`

### 2. 日志断点
不停止程序，只记录信息：
```
右键点击断点 → "Edit Breakpoint" → 添加日志消息
格式："当前值: {message}"
```

### 3. 监视表达式
在调试面板的 "Watch" 部分：
- 添加：`message.len()`
- 添加：`custom_num + hex_num`
- 添加：`&message` (查看地址)

### 4. 内存查看
在调试控制台中：
```lldb
x/10x &variable        # 查看内存
x/2gx &message         # 查看 &str 结构
```

## 📁 项目结构

```
snake_game/
├── .zed/
│   ├── settings.json          # Zed 项目设置
│   └── debug.json            # 调试配置
├── main.rs                   # 主程序
├── memory_demo.rs            # 内存演示程序
├── memory_enhanced_main.rs    # 增强版内存分析
├── Cargo.toml                # Cargo 配置
└── target/debug/snake_game   # 编译输出
```

## 🔧 故障排除

### 问题1: 调试器无法启动
```bash
# 检查 LLDB 是否安装
which lldb
# 如果没有，安装：
brew install llvm
```

### 问题2: 找不到调试目标
```bash
# 确保项目已编译
cargo build

# 检查二进制文件路径
ls -la target/debug/
```

### 问题3: 断点不起作用
- 确保使用 debug 模式编译（不是 `--release`）
- 检查源代码行号是否匹配
- 尝试重新编译

### 问题4: 变量显示为 "optimized out"
```bash
# 使用 debug 配置而不是 release
cargo build   # 不是 cargo build --release
```

## 💡 高级配置

### 自定义调试配置
编辑 `.zed/debug.json`，添加更多配置：

```json
{
  "name": "Custom Debug",
  "type": "lldb",
  "request": "launch",
  "program": "${workspaceFolder}/target/debug/custom_binary",
  "args": ["--test", "--verbose"],
  "cwd": "${workspaceFolder}",
  "environment": [
    {
      "name": "RUST_LOG",
      "value": "debug"
    }
  ],
  "sourceLanguages": ["rust"]
}
```

### 任务配置
编辑 `.zed/settings.json` 添加自定义任务：

```json
{
  "tasks": {
    "run_tests": {
      "command": "cargo",
      "args": ["test", "--", "--test-threads=1"],
      "label": "Run Tests Sequentially"
    },
    "clippy": {
      "command": "cargo",
      "args": ["clippy", "--", "-D", "warnings"],
      "label": "Run Clippy"
    }
  }
}
```

## 🎮 完整调试工作流

1. **准备阶段**:
   ```bash
   # 确保最新编译
   cargo build
   ```

2. **设置断点**:
   - 在 `main.rs` 第2行设置断点

3. **启动调试**:
   - 按 `Cmd + Shift + D` 打开调试面板
   - 选择配置并点击 ▶️

4. **调试过程**:
   - 使用 F10/F11 单步执行
   - 在调试控制台执行 LLDB 命令
   - 观察变量变化

5. **分析结果**:
   - 检查变量值
   - 查看内存布局
   - 分析调用栈

## 🚀 现在开始调试！

1. 在 Zed 中打开 `main.rs`
2. 在第2行设置断点
3. 按 `Cmd + Shift + D` 打开调试面板
4. 选择 "Debug Current File"
5. 按 F5 开始调试

享受 Zed 的强大调试功能！🎉