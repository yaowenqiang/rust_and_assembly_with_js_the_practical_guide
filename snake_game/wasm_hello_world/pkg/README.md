# 🚀 Rust WebAssembly Hello World

一个完整的 Rust WebAssembly "Hello World" 项目，展示如何在浏览器中使用 Rust 编写的高性能代码。

## 🎯 项目特色

- ✅ **Rust 编写，编译为 WebAssembly** - 高性能、类型安全
- ✅ **多种实用函数** - 字符串处理、数学计算、数组操作
- ✅ **精美的 HTML 测试界面** - 现代化 UI 设计
- ✅ **完整的中文支持** - 无障碍中文交互
- ✅ **单元测试包含** - 确保 Rust 代码质量
- ✅ **详细的使用文档** - 快速上手指南

## 🛠️ 环境设置

### 必需工具

1. **Rust 工具链**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **wasm-pack** (WebAssembly 构建工具)
   ```bash
   cargo install wasm-pack
   ```

### 可选工具

- **Python HTTP 服务器** - 用于本地测试
  ```bash
  # macOS 大多已预装 Python 3
  python3 --version
  ```

- **现代浏览器** - Chrome、Firefox、Safari、Edge 都支持 WebAssembly

## 🚀 快速开始

### 方法一：使用构建脚本 (推荐)

```bash
# 1. 进入项目目录
cd wasm_hello_world

# 2. 运行构建脚本
./build.sh

# 3. 启动 HTTP 服务器
python3 -m http.server 8000

# 4. 浏览器访问
open http://localhost:8000/wasm_hello_world/
```

### 方法二：手动构建

```bash
# 1. 构建 WebAssembly
wasm-pack build --target web --out-dir pkg

# 2. 启动服务器
python3 -m http.server 8000

# 3. 浏览器访问
# http://localhost:8000/wasm_hello_world/
```

## 📚 WebAssembly 函数库

### 可用的 Rust 函数 (JavaScript 调用)

| 函数名 | 功能描述 | 参数 | 返回值 | 示例 |
|--------|----------|------|--------|------|
| `greet(name)` | 个性化问候 | `string` | `string` | `greet("张三")` → "Hello, 张三! 欢迎使用 Rust WebAssembly!" |
| `add_numbers(a, b)` | 整数加法 | `number, number` | `number` | `add_numbers(5, 3)` → `8` |
| `calculate_circle_area(radius)` | 圆面积计算 | `number` | `number` | `calculate_circle_area(2.0)` → `12.566...` |
| `reverse_string(s)` | 字符串反转 | `string` | `string` | `reverse_string("hello")` → `"olleh"` |
| `sum_array(numbers)` | 数组求和 | `number[]` | `number` | `sum_array([1,2,3])` → `6` |
| `log_message(msg)` | 控制台日志 | `string` | `void` | `log_message("测试")` |

## 📁 项目结构

```
wasm_hello_world/
├── src/
│   └── lib.rs                    # Rust 源代码
├── pkg/                          # 构建输出 (自动生成)
│   ├── wasm_hello_world.js       # JavaScript 绑定文件
│   ├── wasm_hello_world_bg.wasm  # WebAssembly 二进制文件
│   ├── wasm_hello_world.d.ts     # TypeScript 类型定义
│   └── package.json              # npm 包信息
├── index.html                    # 交互式测试页面
├── build.sh                      # 自动化构建脚本
├── Cargo.toml                    # Rust 项目配置
└── README.md                     # 本文件
```

## 🔧 Rust 代码解析

### 核心 WebAssembly 绑定

```rust
use wasm_bindgen::prelude::*;

// 导出函数给 JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! 欢迎使用 Rust WebAssembly!", name)
}

// 模块加载时自动执行
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::once();
    web_sys::console::log_1(&"Hello from WebAssembly!".into());
}
```

### 关键概念

- **`#[wasm_bindgen]`** - 导出 Rust 函数给 JavaScript
- **`#[wasm_bindgen(start)]`** - WebAssembly 模块加载时自动执行的函数
- **`wasm-bindgen`** 库 - 提供 Rust 和 JavaScript 之间的 FFI (外部函数接口)
- **`web-sys`** 库 - 提供 Web API 的 Rust 绑定

## 🧪 测试

### 运行 Rust 单元测试

```bash
cd wasm_hello_world
cargo test
```

### 浏览器测试

1. 构建项目: `./build.sh`
2. 启动服务器: `python3 -m http.server 8000`
3. 打开测试页面并使用各种功能

## 💡 WebAssembly 优势

### 相比 JavaScript 的性能提升

- **⚡ 接近原生性能** - 比大多数 JavaScript 代码快 10-100 倍
- **🔒 内存安全** - Rust 的所有权系统防止内存泄漏和数据竞争
- **📦 小型二进制** - 编译优化后的文件大小合理
- **🌐 跨平台** - 在所有现代浏览器中运行一致

### 适用场景

- ✅ **计算密集型任务** - 图像处理、加密、科学计算
- ✅ **游戏引擎** - 高性能游戏逻辑
- ✅ **多媒体处理** - 视频/音频编解码
- ✅ **密码学操作** - 安全的加密解密

## 🐛 常见问题解决

### 构建问题

**问题**: wasm-pack 命令未找到
```bash
# 解决方案
cargo install wasm-pack
```

**问题**: Rust 编译错误
```bash
# 检查 Rust 版本
rustc --version
# 更新 Rust
rustup update
```

### 运行时问题

**问题**: 浏览器控制台显示 CORS 错误
```bash
# 解决方案: 使用 HTTP 服务器而不是直接打开文件
python3 -m http.server 8000
```

**问题**: WebAssembly 不支持
```bash
# 检查浏览器版本
# Chrome 57+, Firefox 52+, Safari 11+, Edge 16+ 都支持
```

## 🎓 学习资源

### 官方文档
- [WebAssembly.org](https://webassembly.org/)
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen 文档](https://rustwasm.github.io/wasm-bindgen/)

### 社区资源
- [Rust WebAssembly GitHub](https://github.com/rustwasm)
- [MDN WebAssembly 指南](https://developer.mozilla.org/zh-CN/docs/WebAssembly)

## 📝 扩展建议

### 可以尝试的功能

1. **更多算法** - 添加排序、搜索、加密等算法
2. **图像处理** - 像素操作、滤镜效果
3. **数据分析** - 统计计算、数据可视化
4. **游戏逻辑** - 物理引擎、碰撞检测
5. **文件处理** - 文件上传、解析和处理

### UI 改进

- 🎨 添加更多交互效果
- 📊 性能对比图表
- 📱 移动端适配
- 🌙 深色模式支持

## 🎉 开始你的 WebAssembly 之旅吧！

这是一个完美的起点来学习 Rust + WebAssembly 开发。你可以：

1. 修改 `src/lib.rs` 添加新函数
2. 重新运行 `./build.sh` 构建
3. 在浏览器中测试新功能
4. 对比 WebAssembly vs JavaScript 的性能

**享受 Rust + WebAssembly 带来的高性能体验！** 🚀
