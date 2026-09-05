#!/usr/bin/env bash
# omz-pm 安装脚本
#
# 两种安装方式,按运行场景自动选择:
#   · curl 管道运行(推荐):下载 Release 预编译二进制(自动识别平台,SHA-256 校验)
#   · 仓库克隆内运行:从当前源码编译并软链到 ~/.local/bin(开发者习惯,git pull 后重跑即更新)
#
# 国内加速:
#   curl -fsSL https://ghfast.top/https://raw.githubusercontent.com/ccsert/omz-pm/main/install.sh | bash -s -- --cn
#   · --cn:GitHub 下载走加速前缀;源码编译时 cargo 走 rsproxy(仅本次构建生效,不改全局配置)
#   · 公共加速前缀时有失效,可用 OMZ_PM_MIRROR=<前缀> 或 --mirror <前缀> 换成自建前缀
#
# 其他用法:
#   ./install.sh --prebuilt          强制下载预编译
#   ./install.sh --build             强制源码编译
#   ./install.sh --version v0.3.1    安装指定版本(默认最新 Release)
#   ./install.sh --no-verify         跳过 SHA-256 校验(不建议)
#   ./install.sh --uninstall         卸载
#   ./install.sh --help              帮助
#
# 环境变量:
#   OMZ_PM_MIRROR=<前缀>             GitHub 加速前缀,等价 --mirror
#   OMZ_PM_INSTALL_DIR=<目录>        安装目录,默认 ~/.local/bin
set -euo pipefail

REPO="ccsert/omz-pm"
BIN="omz-pm"
DEFAULT_MIRROR="https://ghfast.top"

VERSION=""
MIRROR="${OMZ_PM_MIRROR:-}"
CN=0
MODE="auto" # auto | prebuilt | build
VERIFY=1
UNINSTALL=0

INSTALL_DIR="${OMZ_PM_INSTALL_DIR:-$HOME/.local/bin}"

# 是否运行在含 Cargo.toml 的仓库克隆内(curl 管道运行时 BASH_SOURCE 不是脚本路径)
SCRIPT_PATH="${BASH_SOURCE[0]:-}"
IN_REPO=0
if [[ -f "$SCRIPT_PATH" && -f "$(dirname "$SCRIPT_PATH")/Cargo.toml" ]]; then
    IN_REPO=1
fi

usage() {
    sed -n '2,30p' "$SCRIPT_PATH" 2>/dev/null | sed 's/^# \{0,1\}//'
}

gh_url() {
    # $1 = 相对 github.com 的路径(owner/repo/...)
    if [[ -n "$MIRROR" ]]; then
        echo "${MIRROR%/}/https://github.com/$1"
    else
        echo "https://github.com/$1"
    fi
}

fetch() {
    curl -fsSL --retry 2 --connect-timeout 10 "$1" -o "$2"
}

# 非交互 shell 里 cargo 常不在 PATH,补一次常见位置
ensure_cargo_on_path() {
    command -v cargo >/dev/null 2>&1 && return 0
    if [ -x "$HOME/.cargo/bin/cargo" ]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
    command -v cargo >/dev/null 2>&1
}

detect_target() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"
    case "$os/$arch" in
        Darwin/arm64 | Darwin/aarch64) echo "aarch64-apple-darwin" ;;
        Darwin/x86_64) echo "x86_64-apple-darwin" ;;
        Linux/x86_64) echo "x86_64-unknown-linux-musl" ;;
        Linux/aarch64 | Linux/arm64) echo "aarch64-unknown-linux-gnu" ;;
        *) echo "" ;;
    esac
}

release_base() {
    # Release 资产路径前缀(不含资产名)
    if [[ -n "$VERSION" ]]; then
        echo "$REPO/releases/download/$VERSION"
    else
        echo "$REPO/releases/latest/download"
    fi
}

install_prebuilt() {
    local target
    target="$(detect_target)"
    if [[ -z "$target" ]]; then
        echo "未识别的平台 $(uname -s)/$(uname -m),请用 --build 从源码编译" >&2
        return 1
    fi
    local asset base url tmp
    asset="omz-pm-$target.tar.gz"
    base="$(release_base)"
    url="$(gh_url "$base/$asset")"
    tmp="$(mktemp -d)"

    echo "==> 下载预编译二进制($target)…"
    if [[ -n "$MIRROR" ]]; then
        echo "    走加速前缀: $MIRROR"
    fi
    if ! fetch "$url" "$tmp/$asset"; then
        rm -rf "$tmp"
        echo "下载失败: $url" >&2
        if [[ -z "$MIRROR" ]]; then
            echo "国内网络可加 --cn 走镜像重试(见脚本头部说明)" >&2
        fi
        return 1
    fi

    if [[ $VERIFY -eq 1 ]]; then
        echo "==> 校验 SHA-256…"
        fetch "$(gh_url "$base/$asset.sha256")" "$tmp/$asset.sha256"
        if ! (
            cd "$tmp" &&
                if command -v sha256sum >/dev/null 2>&1; then
                    sha256sum -c --status "$asset.sha256"
                else
                    shasum -a 256 -c -s "$asset.sha256" >/dev/null
                fi
        ); then
            rm -rf "$tmp"
            echo "SHA-256 校验失败(内容与官方 Release 不一致,可能是镜像过期)。" >&2
            echo "可换镜像(OMZ_PM_MIRROR=<前缀>)重试,或加 --no-verify 跳过(不建议)。" >&2
            return 1
        fi
    fi

    tar xzf "$tmp/$asset" -C "$tmp"
    mkdir -p "$INSTALL_DIR"
    install -m 0755 "$tmp/omz-pm-$target/$BIN" "$INSTALL_DIR/$BIN"
    rm -rf "$tmp"
    echo "✓ 已安装 $INSTALL_DIR/$BIN($("$INSTALL_DIR/$BIN" --version))"
}

build_from_source() {
    ensure_cargo_on_path || {
        echo "错误: 未找到 cargo,请先安装 Rust (https://rustup.rs)" >&2
        return 1
    }
    local srcdir tmp
    if [[ $IN_REPO -eq 1 ]]; then
        srcdir="$(cd "$(dirname "$SCRIPT_PATH")" && pwd)"
        echo "==> 使用当前仓库源码: $srcdir"
    else
        local src_url
        if [[ -n "$VERSION" ]]; then
            src_url="$(gh_url "$REPO/archive/refs/tags/$VERSION.tar.gz")"
        else
            src_url="$(gh_url "$REPO/archive/refs/heads/main.tar.gz")"
        fi
        echo "==> 下载源码包…"
        tmp="$(mktemp -d)"
        fetch "$src_url" "$tmp/src.tar.gz"
        tar xzf "$tmp/src.tar.gz" -C "$tmp"
        srcdir="$(echo "$tmp"/omz-pm-*)"
    fi

    echo "==> 编译 release 版本…"
    if [[ $CN -eq 1 ]]; then
        echo "    cargo 走 rsproxy 镜像(--config 注入,不改动你的全局 cargo 配置)"
        (cd "$srcdir" &&
            cargo build --release \
                --config 'source.crates-io.replace-with="rsproxy-sparse"' \
                --config 'source.rsproxy-sparse.registry="sparse+https://rsproxy.cn/index/"')
    else
        (cd "$srcdir" && cargo build --release)
    fi

    mkdir -p "$INSTALL_DIR"
    if [[ $IN_REPO -eq 1 ]]; then
        # 仓库内安装用软链:git pull 后重跑本脚本即可原地更新
        ln -sf "$srcdir/target/release/$BIN" "$INSTALL_DIR/$BIN"
    else
        install -m 0755 "$srcdir/target/release/$BIN" "$INSTALL_DIR/$BIN"
        [[ -n "${tmp:-}" ]] && rm -rf "$tmp"
    fi
    echo "✓ 已安装 $INSTALL_DIR/$BIN($("$INSTALL_DIR/$BIN" --version))"
}

uninstall() {
    if [[ -f "$INSTALL_DIR/$BIN" ]]; then
        rm -f "$INSTALL_DIR/$BIN"
        echo "✓ 已删除 $INSTALL_DIR/$BIN"
    else
        echo "$INSTALL_DIR/$BIN 不存在,无需卸载"
    fi
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --cn)
            CN=1
            [[ -z "$MIRROR" ]] && MIRROR="$DEFAULT_MIRROR"
            shift
            ;;
        --mirror)
            [[ $# -lt 2 ]] && {
                echo "--mirror 需要一个 URL 前缀" >&2
                exit 1
            }
            MIRROR="$2"
            shift 2
            ;;
        --mirror=*)
            MIRROR="${1#*=}"
            shift
            ;;
        --version)
            [[ $# -lt 2 ]] && {
                echo "--version 需要一个 tag(如 v0.3.1)" >&2
                exit 1
            }
            VERSION="$2"
            shift 2
            ;;
        --version=*)
            VERSION="${1#*=}"
            shift
            ;;
        --prebuilt)
            MODE="prebuilt"
            shift
            ;;
        --build)
            MODE="build"
            shift
            ;;
        --no-verify)
            VERIFY=0
            shift
            ;;
        --uninstall)
            UNINSTALL=1
            shift
            ;;
        -h | --help)
            usage
            exit 0
            ;;
        *)
            echo "未知参数: $1(见 --help)" >&2
            exit 1
            ;;
    esac
done

if [[ $UNINSTALL -eq 1 ]]; then
    uninstall
    exit 0
fi

case "$MODE" in
    prebuilt)
        install_prebuilt || exit 1
        ;;
    build)
        build_from_source || exit 1
        ;;
    auto)
        if [[ $IN_REPO -eq 1 ]]; then
            build_from_source || exit 1
        elif ! install_prebuilt; then
            if ensure_cargo_on_path; then
                echo "==> 回退到源码编译…"
                build_from_source || exit 1
            else
                exit 1
            fi
        fi
        ;;
esac

case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        echo ""
        echo "提示: $INSTALL_DIR 不在 PATH 中,请把下面这行加进 ~/.zshrc:"
        echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
        ;;
esac

echo ""
echo "安装完成!运行 omz-pm 进入插件管理界面。"
