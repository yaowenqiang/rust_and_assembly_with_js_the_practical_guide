#!/bin/bash
# Rust 内存分析一键脚本

PROGRAM=${1:-memory_demo}

echo "=== Rust 程序内存分析工具 ==="
echo "分析程序: $PROGRAM"
echo ""

if [ ! -f "$PROGRAM" ]; then
    echo "错误: 程序 '$PROGRAM' 不存在"
    echo "使用方法: $0 [program_name]"
    exit 1
fi

# 1. 基本文件信息
echo "1. 基本文件信息:"
ls -lh "$PROGRAM"
echo ""

# 2. 段大小分析
echo "2. 内存段分布:"
size "$PROGRAM"
echo ""

# 3. 符号表分析
echo "3. 符号数量分析:"
echo "总符号数: $(nm "$PROGRAM" | wc -l)"
echo "外部符号: $(nm "$PROGRAM" | grep ' U ' | wc -l)"
echo "全局符号: $(nm "$PROGRAM" | grep ' T ' | wc -l)"
echo ""

# 4. 依赖库分析
echo "4. 动态库依赖:"
otool -L "$PROGRAM"
echo ""

# 5. 使用 dtrace 进行内存分析（macOS）
if command -v dtrace >/dev/null 2>&1; then
    echo "5. 内存分配统计 (运行10秒):"
    echo "sudo dtrace -n 'syscall::malloc:return { @ = count(); }' -c \"$PROGRAM\""
    echo ""
fi

# 6. 汇编代码查看
echo "6. 生成汇编代码:"
echo "otool -tV \"$PROGRAM\" > \"$PROGRAM.asm\""
echo "或使用 Rust 生成: rustc --emit asm source.rs"
echo ""

echo "=== 推荐的调试方法 ==="
cat << 'EOF'

1. 使用 LLDB 查看运行时内存:
   rust-lldb memory_demo
   (lldb) b main
   (lldb) r
   (lldb) x/20x &variable    # 查看内存内容
   (lldb) p sizeof(variable) # 查看变量大小

2. 使用 rustc 查看编译后的内存布局:
   rustc --emit asm --emit llvm-ir source.rs

3. 使用 cargo 进行完整的内存分析:
   cargo build --release
   cargo valgrind run (需要安装 cargo-valgrind)

4. 运行时内存跟踪:
   RUST_LOG=info cargo run

EOF