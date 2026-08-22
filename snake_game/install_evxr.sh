#!/bin/bash
# Rust REPL (Evcxr) 快速安装和演示

echo "🦀 Rust REPL (Evcxr) 安装指南"
echo ""

# 1. 检查 Rust 环境
echo "1. 检查 Rust 环境..."
if command -v rustc &> /dev/null; then
    echo "✅ Rust 已安装: $(rustc --version)"
else
    echo "❌ Rust 未安装，请先安装 Rust:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

if command -v cargo &> /dev/null; then
    echo "✅ Cargo 已安装: $(cargo --version)"
else
    echo "❌ Cargo 未安装"
    exit 1
fi

echo ""

# 2. 检查 Evcxr 是否已安装
echo "2. 检查 Evcxr 安装状态..."
if command -v evcxr &> /dev/null; then
    echo "✅ Evcxr 已安装: $(evcxr --version 2>/dev/null || echo '版本未知')"
    EVCXR_INSTALLED=true
else
    echo "❌ Evcxr 未安装"
    echo "💡 安装命令: cargo install evcxr_repl"
    EVCXR_INSTALLED=false
fi

echo ""

# 3. 提供安装选项
if [ "$EVCXR_INSTALLED" = false ]; then
    echo "3. 📦 安装 Evcxr REPL..."
    echo ""
    echo "选项 1: 直接安装（推荐）"
    echo "  cargo install evcxr_repl"
    echo ""
    echo "选项 2: 启用所有特性"
    echo "  cargo install evcxr_repl --features \"evcxr_repl\""
    echo ""
    echo "💡 Jupyter 支持（可选）:"
    echo "  cargo install evcxr_jupyter"
    echo ""

    read -p "是否现在安装 Evcxr? (y/n): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "正在安装 Evcxr REPL..."
        cargo install evcxr_repl

        if [ $? -eq 0 ]; then
            echo "✅ Evcxr 安装成功！"
            EVCXR_INSTALLED=true
        else
            echo "❌ 安装失败，请检查错误信息"
            exit 1
        fi
    else
        echo "跳过安装"
    fi
fi

echo ""

# 4. 提供使用演示
if [ "$EVCXR_INSTALLED" = true ]; then
    echo "4. 🎮 Evcxr REPL 使用演示..."
    echo ""

    cat << 'EOF'
启动 REPL:
  evcxr

基本命令:
  :help              - 显示帮助
  :quit              - 退出 REPL
  :vars              - 显示所有变量
  :type <expr>       - 显示表达式类型
  :dep <crate> <ver> - 添加依赖

示例会话:
  >> 1 + 2
  >> let x = 42;
  >> x * 2
  >> println!("Hello, REPL!");
  >> :dep rand = "0.8"
  >> use rand::Rng;

EOF

    # 提供一个交互式示例
    read -p "是否启动交互式 REPL 演示? (y/n): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "启动 Evcxr REPL..."
        echo "试试这些命令:"
        echo "  1 + 2"
        echo "  let x = 42;"
        echo "  x * 2"
        echo "  :vars"
        echo "  :help"
        echo ""
        echo "退出时输入 :quit"
        echo ""

        if command -v evcxr &> /dev/null; then
            evcxr
        else
            echo "❌ Evcxr 未找到，请先安装"
        fi
    fi
fi

echo ""

# 5. Jupyter 支持
echo "5. 📊 Jupyter Notebook 支持（可选）..."
echo ""
if command -v jupyter &> /dev/null; then
    echo "✅ Jupyter 已安装"

    if command -v evcxr_jupyter &> /dev/null; then
        echo "✅ Evcxr Jupyter 内核已安装"
    else
        echo "💡 安装 Jupyter 内核:"
        echo "   cargo install evcxr_jupyter"
        echo "   evcxr_jupyter --install"
    fi
else
    echo "❌ Jupyter 未安装"
    echo "💡 安装 Jupyter:"
    echo "   pip install jupyter"
    echo "   cargo install evcxr_jupyter"
    echo "   evcxr_jupyter --install"
fi

echo ""

# 6. 在线替代方案
echo "6. 🌐 在线替代方案（无需安装）..."
echo ""
echo "Rust Playground (官方):"
echo "  https://play.rust-lang.org/"
echo ""
echo "Rust Godbolt (查看汇编):"
echo "  https://rust.godbolt.org/"
echo ""

echo "🎉 设置完成！"
echo ""
echo "快速开始:"
if [ "$EVCXR_INSTALLED" = true ]; then
    echo "  evcxr                    # 启动 REPL"
else
    echo "  cargo install evcxr_repl # 安装 REPL"
    echo "  evcxr                    # 启动 REPL"
fi
echo ""
echo "详细指南: rust_repl_guide.md"