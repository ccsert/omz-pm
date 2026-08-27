#!/usr/bin/env bash
# omz-pm 安装脚本:构建 release 版本并链接到 ~/.local/bin
set -euo pipefail

cd "$(dirname "$0")"

command -v cargo >/dev/null 2>&1 || {
    # 尝试常见的 cargo 路径
    if [ -x "$HOME/.cargo/bin/cargo" ]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    else
        echo "错误: 未找到 cargo,请先安装 Rust (https://rustup.rs)" >&2
        exit 1
    fi
}

echo "==> 构建 release 版本..."
cargo build --release

BIN_DIR="${HOME}/.local/bin"
mkdir -p "$BIN_DIR"

echo "==> 链接到 ${BIN_DIR}/omz-pm ..."
ln -sf "$(pwd)/target/release/omz-pm" "${BIN_DIR}/omz-pm"

case ":$PATH:" in
    *":${BIN_DIR}:"*) ;;
    *)
        echo ""
        echo "提示: ${BIN_DIR} 不在 PATH 中,请把下面这行加进 ~/.zshrc:"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        ;;
esac

echo ""
echo "安装完成!运行 omz-pm 进入插件管理界面。"
