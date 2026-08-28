# kube-ps1:适用于 bash、zsh 和 fish 的 Kubernetes 提示符

![GitHub Release](https://img.shields.io/github/v/release/jonmosco/kube-ps1)
[![CI](https://github.com/jonmosco/kube-ps1/actions/workflows/ci.yml/badge.svg)](https://github.com/jonmosco/kube-ps1/actions/workflows/ci.yml)

一个脚本,可以把 `kubectl` 当前配置的 Kubernetes 上下文(context)和命名空间(namespace)添加到你的 Bash、Zsh 或 Fish 提示符里。

灵感来自若干旨在简化 `kubectl` 使用的工具。

![提示符演示](img/kube-ps1-demo.gif)

## 安装

### 软件包

### macOS Brew 包

Homebrew 包管理器:

```sh
brew update
brew install kube-ps1
```

### Arch Linux

AUR 软件包见 [https://aur.archlinux.org/packages/kube-ps1/](https://aur.archlinux.org/packages/kube-ps1/)。

### Oh My Zsh

https://github.com/ohmyzsh/ohmyzsh

kube-ps1 已作为插件包含在 oh-my-zsh 项目中。要启用它,请编辑你的 `~/.zshrc` 并添加该插件:

```bash
plugins=(
  kube-ps1
)
PROMPT='$(kube_ps1)'$PROMPT # or RPROMPT='$(kube_ps1)'
```

## Zsh zinit 插件

### 使用 [zinit](https://github.com/zdharma-continuum/zinit)

在 `.zshrc` 中加入:

```sh
zinit light jonmosco/kube-ps1
PROMPT='$(kube_ps1)'$PROMPT # or RPROMPT='$(kube_ps1)'
```

### Fig

一键即可在 zsh、bash 或 fish 中安装 `kube-ps1`。

<a href="https://fig.io/plugins/other/kube-ps1" target="_blank"><img src="https://fig.io/badges/install-with-fig.svg" width="120" /></a>

### 从源码(git clone)

1. 克隆本仓库
2. 在你的 `~/.zshrc` 或 `~/.bashrc` 中 source kube-ps1.sh

#### Zsh

```sh
source /path/to/kube-ps1.sh
PROMPT='$(kube_ps1)'$PROMPT # or RPROMPT='$(kube_ps1)'
```

#### Bash

```sh
source /path/to/kube-ps1.sh
PS1='[\u@\h \W $(kube_ps1)]\$ '
```

#### Fish

把下面的内容加进 `~/.config/fish/config.fish`:

```fish
source /path/to/kube-ps1.fish

function fish_prompt
    echo -n (kube_ps1) ' '
    # your existing prompt here
end
```

> 注意:Fish 用户应 source `kube-ps1.fish`,而不是 `kube-ps1.sh`。

## 依赖要求

默认的提示符假定你已经安装了 `kubectl` 命令行工具。官方安装说明和二进制文件如下:

[安装和配置 kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl/)

如果要配合 OpenShift 使用,则需要安装 `oc` 工具。可以从 brew ports 获取:

```sh
brew install openshift-cli
```

也可以下载源码:

[OC 客户端工具](https://github.com/okd-project/okd/releases)

用下面的变量把二进制文件设为 `oc`:

```sh
KUBE_PS1_BINARY=oc
```

如果两个二进制文件都不可用,提示符会打印如下内容:

```sh
(<symbol>|BINARY-N/A:N/A)
```

## 辅助工具

有一些非常出色的工具能让 kubectl 的使用体验更加愉快:

- [`kubectx` 和 `kubens`](https://github.com/ahmetb/kubectx) 非常适合在集群和命名空间之间快速切换。

## Tmux 移植版

我已经开始把 kube-ps1 移植为 tmux 的状态栏插件。如果你更偏好 tmux,又喜欢 kube-ps1 提供的功能,可以看看 [kube-tmux](https://github.com/jonmosco/kube-tmux) 项目

## 提示符结构

默认的提示符布局如下:

```sh
(<symbol>|<context>:<namespace>)
```

如果未设置 current-context,kube-ps1 会返回如下内容:

```sh
(<symbol>|N/A:N/A)
```

## 启用/禁用

如果想临时不在提示符里显示 Kubernetes 状态,运行 `kubeoff` 即可。要在所有 shell 会话中都禁用该提示符,运行 `kubeoff -g`。可以在当前 shell 中运行 `kubeon` 重新启用,或运行 `kubeon -g` 全局启用。

```sh
kubeon     : turn on kube-ps1 status for this shell.  Takes precedence over
             global setting for current session
kubeon -g  : turn on kube-ps1 status globally
kubeoff    : turn off kube-ps1 status for this shell. Takes precedence over
             global setting for current session
kubeoff -g : turn off kube-ps1 status globally
```

## 符号

默认符号是 UTF8 字符,在大多数字体下都能正常显示。如果你想使用 Kubernetes 和 OpenShift 的字形图标,需要安装包含相应字形的补丁字体。[Nerd Fonts](https://www.nerdfonts.com/) 同时提供这两个字形。按照其安装说明安装补丁字体即可。

`KUBE_PS1_SYMBOL_CUSTOM` 的选项

| 选项 | 符号 | 说明 |
| ------------- | ------ | ----------- |
| default(空字符串) | ⎈ | 默认符号(Unicode `\u2388`) |
| img | ☸️ | 常用来代表 Kubernetes 的符号(Unicode `\u2638`) |
| oc | ![openshift-glyph](img/openshift-glyph.png) | 代表 OpenShift 的符号(Unicode `\ue7b7`) |
| k8s | ![k8s-glyph](img/k8s-glyph.png) | 代表 Kubernetes 的符号(Unicode `\ue7b7`) |

要把符号设为某个自定义字形,请把下面这行加进你的 `~/.bashrc` 或 `~/.zshrc`:

```sh
KUBE_PS1_SYMBOL_CUSTOM=img
```

要恢复默认符号,把 `KUBE_PS1_SYMBOL` 设为空字符串即可。

下面是各符号实际效果的演示:
![kube-ps1-symbols](img/kube-ps1-symbol-demo.gif)

如果字体没有正确安装、字形不可用,则会显示一对空括号或类似内容:

```sh
 echo -n "\ue7b7"
 
```

## 自定义

默认的设置可以在 `~/.bashrc`、`~/.zshrc` 或 `~/.config/fish/config.fish` 中通过设置以下变量来覆盖:

| 变量 | 默认值 | 含义 |
| :------- | :-----: | ------- |
| `KUBE_PS1_BINARY` | `kubectl` | 默认的 Kubernetes 二进制文件 |
| `KUBE_PS1_NS_ENABLE` | `true` | 显示命名空间。设为 `false` 时,也会一并禁用 `KUBE_PS1_DIVIDER` |
| `KUBE_PS1_PREFIX` | `(` | 提示符的起始字符 |
| `KUBE_PS1_SYMBOL_ENABLE` | `true` | 显示提示符符号。设为 `false` 时,也会一并禁用 `KUBE_PS1_SEPARATOR` |
| `KUBE_PS1_SYMBOL_PADDING` | `false` | 在符号后面加一个空格(内边距),避免挤坏提示符中的其他字符 |
| `KUBE_PS1_SYMBOL_CUSTOM` | `⎈` | 更改默认的提示符符号。Unicode `\u2388`。可选值为 `k8s`、`img`、`oc` |
| `KUBE_PS1_SYMBOL_COLOR` | `blue` | 更改默认的符号颜色。 |
| `KUBE_PS1_SEPARATOR` | &#124; | 符号与上下文名称之间的分隔符 |
| `KUBE_PS1_DIVIDER` | `:` | 上下文与命名空间之间的分隔符 |
| `KUBE_PS1_SUFFIX` | `)` | 提示符的结束字符 |
| `KUBE_PS1_CLUSTER_FUNCTION` | 无默认值,必须由用户提供 | 用于自定义集群(cluster)显示方式的函数 |
| `KUBE_PS1_NAMESPACE_FUNCTION` | 无默认值,必须由用户提供 | 用于自定义命名空间显示方式的函数 |
| `KUBE_PS1_CTX_COLOR_FUNCTION` | 无默认值,必须由用户提供 | 根据上下文名称自定义其颜色的函数 |
| `KUBE_PS1_HIDE_IF_NOCONTEXT` | `false` | 未设置上下文时隐藏 kube-ps1 提示符 |

要禁用某个特性,把它设为空字符串即可:

```sh
KUBE_PS1_SEPARATOR=''
```

## 颜色

默认颜色由以下变量设置:

| 变量 | 默认值 | 含义 |
| :------- | :-----: | ------- |
| `KUBE_PS1_PREFIX_COLOR` | `null` | 设置提示符前缀的默认颜色 |
| `KUBE_PS1_SYMBOL_COLOR` | `blue` | 设置 Kubernetes 符号的默认颜色 |
| `KUBE_PS1_CTX_COLOR` | `red` | 设置上下文的默认颜色 |
| `KUBE_PS1_SUFFIX_COLOR` | `null` | 设置提示符后缀的默认颜色 |
| `KUBE_PS1_NS_COLOR` | `cyan` | 设置命名空间的默认颜色 |
| `KUBE_PS1_BG_COLOR` | `null` | 设置提示符背景的默认颜色 |

默认符号选用蓝色,是为了尽可能贴近 Kubernetes 的品牌色。上下文名称选用红色以示醒目,命名空间则用青色。

如果不想让提示符的某个部分带颜色,把对应变量设为空字符串即可:

```sh
KUBE_PS1_CTX_COLOR=''
```

下列颜色可以直接使用颜色名称:

```text
black, red, green, yellow, blue, magenta, cyan
```

把数字值作为变量参数传入,即可使用 256 色。

## 自定义集群名称和命名空间的显示

你可以分别通过 `KUBE_PS1_CLUSTER_FUNCTION` 和 `KUBE_PS1_NAMESPACE_FUNCTION` 变量,改变集群名称和命名空间的显示方式。

在下面的示例中,我们假设:

集群名称:`sandbox.k8s.example.com`
命名空间:`alpha`

如果你的集群名称是域名风格,提示符很快就会变得很长。假设你只想显示集群名称的第一段(`sandbox`),可以这样实现:

```sh
function get_cluster_short() {
  echo "$1" | cut -d . -f1
}

KUBE_PS1_CLUSTER_FUNCTION=get_cluster_short
```

用同样的办法也可以自定义命名空间的显示。假设你希望命名空间全部以大写显示(`ALPHA`),下面是一种做法:

```sh
function get_namespace_upper() {
    echo "$1" | tr '[:lower:]' '[:upper:]'
}

export KUBE_PS1_NAMESPACE_FUNCTION=get_namespace_upper
```

以上两种情况中,变量都被设为函数名,而且你必须先在 shell 配置中定义好该函数,kube_ps1 被调用之前尤须如此。该函数必须接受单个参数,并把最终值 echo 出来。

## 动态上下文颜色

你可以用 `KUBE_PS1_CTX_COLOR_FUNCTION` 变量为不同的上下文设置不同的颜色。这对用颜色区分上下文、让生产环境在视觉上一眼可辨非常有用。

例如,让生产环境的上下文显示为红色、开发环境的显示为绿色:

```sh
function kube_ps1_ctx_color() {
  local context="$1"

  case "$context" in
    *prod*)
      echo "red"
      ;;
    *dev*)
      echo "green"
      ;;
    *staging*|*stg*)
      echo "yellow"
      ;;
    *)
      echo "cyan"  # default color for other contexts
      ;;
  esac
}

export KUBE_PS1_CTX_COLOR_FUNCTION=kube_ps1_ctx_color
```

该函数接收上下文名称作为第一个参数,并应 echo 出期望的颜色名称。`KUBE_PS1_CTX_COLOR` 支持的所有颜色选项都可用,包括命名颜色(black、red、green、yellow、blue、magenta、cyan、white)和 256 色代码(0-256)。

如果未设置 `KUBE_PS1_CTX_COLOR_FUNCTION`,kube-ps1 会使用 `KUBE_PS1_CTX_COLOR` 的值(默认:red)。

### Bug 报告与 shell 配置

由于 shell 的自定义方式千差万别,提交 bug 报告之前,请先用最小化配置测试该提示符。

在加载 kube-ps1 之前,各 shell 可以按如下方式操作:

Bash:

```sh
bash --norc
```

Zsh:

```sh
zsh -f
or
zsh --no-rcs
```

提示符符号需要安装包含相应字形的补丁字体。[Nerd Fonts Downloads](https://www.nerdfonts.com/font-downloads) 提供包含这些字形的补丁字体。具体请查阅其文档,相关支持不在本项目的范围内。

### 贡献者

感谢社区里每一位为 kube-ps1 做出贡献的人!

<a href="https://github.com/jonmosco/kube-ps1/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=jonmosco/kube-ps1" />
</a>
