[![License (GPL version 3)](https://img.shields.io/badge/license-GNU%20GPL%20version%203-blue.svg?style=flat-square)](./LICENSE)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](./LICENSE)
![ZSH 5.0.0](https://img.shields.io/badge/zsh-v5.0.0-orange.svg?style=flat-square)
[![Gitter][gitter-image]][gitter-link]

![znt logo](https://imageshack.com/a/img905/2629/WK9qjN.png)

# Zsh Navigation Tools

一组工具,包括 `n-history`——多关键词历史搜索器、`n-cd`——目录书签管理器、
`n-kill`——类似 `htop` 的 kill 工具,等等。它们都基于 `n-list`:一个生成可选中、
基于 curses 的元素列表的工具,并且能访问当前 `Zsh` 会话,也就是说具备与 Zsh
协同工作的强大能力。特性亮点包括:增量多关键词搜索、近似匹配、ANSI 着色、
主题、去重模式、横向滚动、grep 过滤、高级历史管理,以及与 `Zsh` 的多种集成。

✅ 启用方式:把「zsh-navigation-tools」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

另请查看 [![ZCA](https://imageshack.com/a/img911/8084/qSpO8a.png) Zsh Command Architect](https://github.com/zdharma-continuum/zsh-cmd-architect)
和 [Zconvey](https://github.com/zdharma-continuum/zconvey)



视频:

- [https://youtu.be/QwZ8IJEgXRE](https://youtu.be/QwZ8IJEgXRE)
- [https://youtu.be/DN9QqssAYB8](https://youtu.be/DN9QqssAYB8)

截图:

![n-history](https://imageshack.com/a/img921/5046/bqr0mk.png)

![n-history](https://imageshack.com/a/img633/9905/WzfSdl.gif)


## 历史记录 Widget

要把 `n-history` 作为增量搜索器绑定到 `Ctrl-R`,请把 `znt-*` 文件复制到
`*/site-functions` 目录(除非你采用单文件安装方式),然后在 `.zshrc` 中添加:

```zsh
autoload znt-history-widget
zle -N znt-history-widget
bindkey "^R" znt-history-widget
```

如果使用安装脚本、zgen、antigen 或单文件安装,这一步会自动完成。另外还有两个
widget:`znt-cd-widget` 和 `znt-kill-widget`,它们同样可以绑定到按键组合
(`autoload` 已在 `.zshrc` 中完成,无需再做):

```zsh
zle -N znt-cd-widget
bindkey "^B" znt-cd-widget
zle -N znt-kill-widget
bindkey "^Y" znt-kill-widget
```

## 简介

这些工具是:

- `n-aliases`——浏览别名,编辑交给 `vared` 完成
- `n-cd`——浏览目录栈和书签目录,可进入选中的目录
- `n-functions`——浏览函数,编辑交给 `zed` 或 `vared` 完成
- `n-history`——浏览历史记录,可从中编辑并运行命令
- `n-kill`——浏览进程列表,可向选中的进程发送信号
- `n-env`——浏览环境变量,编辑交给 `vared` 完成
- `n-options`——浏览选项,可切换其开关状态
- `n-panelize`——把给定命令的输出载入列表以供浏览

所有工具都支持横向滚动,按键为 `<`、`>`、`{`、`}`、`h`、`l` 或左右方向键。其他按键有:

- `H`、`?`(来自 n-history)——运行 n-help
- `Ctrl-R`——启动 n-history,即增量式多关键词历史搜索器(Zsh 绑定)
- `Ctrl-A`——轮换已输入的词(1+2+3 -> 3+1+2)
- `Ctrl-F`——修正模式(近似匹配)
- `Ctrl-L`——重绘整个显示
- `Ctrl-T`——浏览主题(下一个主题)
- `Ctrl-G`——浏览主题(上一个主题)
- `Ctrl-U`——向上翻半页
- `Ctrl-D`——向下翻半页
- `Ctrl-P`——上一个元素(也可用 vim 的 k)
- `Ctrl-N`——下一个元素(也可用 vim 的 j)
- `[`、`]`——在 n-cd 中跳转目录书签,在 n-kill 中跳转常见信号
- `g`、`G`——列表开头和结尾
- `/`——显示增量搜索
- `F3`——显示/隐藏增量搜索
- `Esc`——退出增量搜索并清空过滤条件
- `Ctrl-W`(增量搜索中)——删除整个词
- `Ctrl-K`(增量搜索中)——删除整行
- `Ctrl-O`、`o`——进入去重模式(不显示重复行)
- `Ctrl-E`、`e`——编辑私有历史(处于私有历史视图时)
- `F1`——(n-history 中)切换视图
- `F2`、`Ctrl-X`、`Ctrl-/`——搜索预定义关键词(在配置文件中定义)

## 配置

`ZNT` 的配置文件位于 `~/.config/znt`。这些文件是:

```
n-aliases.conf
n-cd.conf
n-env.conf
n-functions.conf
n-history.conf
n-kill.conf
n-list.conf
n-options.conf
n-panelize.conf
```

`n-list.conf` 包含主要配置变量:

```zsh
# Should the list (text, borders) be drawn in bold
local bold=0

# Main color pair (foreground/background)
local colorpair="white/black"

# Should draw the border?
local border=1

# Combinations of colors to try out with Ctrl-T and Ctrl-G
# The last number is the bold option, 0 or 1
local -a themes
themes=( "white/black/1" "green/black/0" "green/black/1" "white/blue/0" "white/blue/1"
         "magenta/black/0" "magenta/black/1" )
```

其余配置文件的内容可自行阅读。此外,配置也可以在 `zshrc` 中设置。有 `5` 个
标准的 `zshrc` 配置变量:

```
znt_history_active_text - underline or reverse - how should be active element highlighted
znt_history_nlist_coloring_pattern - pattern that can be used to colorize elements
znt_history_nlist_coloring_color - color with which to colorize
znt_history_nlist_coloring_match_multiple - should multiple matches be colorized (0 or 1)
znt_history_keywords (array) - search keywords activated with `Ctrl-X`, `F2` or `Ctrl-/`, e.g. ( "git" "vim" )
```

以上变量对 `n-history` 工具生效。对其他工具,把 `_history_` 换掉即可,
比如 `n-cd` 工具用 `_cd_`。全部 `8` 个工具都是如此。

工具的通用配置使用带 `_list_` 的变量:

```
znt_list_bold - should draw text in bold (0 or 1)
znt_list_colorpair - main pair of colors to be used, e.g "green/black"
znt_list_border - should draw borders around windows (0 or 1)
znt_list_themes (array) - list of themes to try out with Ctrl-T, e.g. ( "white/black/1" "green/black/0" )
znt_list_instant_select - should pressing enter in search mode leave tool (0 or 1)
```

如果你使用的是 `v2.1.12` 之前的 `ZNT`,请删除旧的配置文件 `~/.config/znt/*.conf`,
以便 `ZNT` 把它们更新为支持与 `Zshrc` 集成的最新版本。如果你用的是安装脚本,
请在删除配置文件后重新运行一次。

## 编程

`n-list` 函数的用法如下:

```zsh
n-list {element1} [element2] ... [elementN]
```

只需这一步,就能获得 ANSI 着色、增量多关键词搜索、去重模式、横向滚动、
不可选元素等特性(grep 过滤在 `n-list` 之外完成,具体做法可参考各个工具)。
要设置不可选中的条目,把它们的索引加入数组 `NLIST_NONSELECTABLE_ELEMENTS`:

```zsh
typeset -a NLIST_NONSELECTABLE_ELEMENTS
NLIST_NONSELECTABLE_ELEMENTS=( 1 )
```

结果存储在 `$reply[REPLY]` 中(`REPLY` 前不需要 `$`,因为 `[]` 内是算术上下文)。
返回的数组可能与输入参数不同,因为 `n-list` 可能已通过增量搜索或去重模式对它们
做过处理。`$REPLY` 就是这个可能被处理过的数组中的索引。如果 `$REPLY` 等于
`-1`,表示没有做出选择(用户按 `q` 键退出)。

要设置可用 `[`、`]` 键跳转的条目,把它们的索引加入 `NLIST_HOP_INDEXES` 数组:

```zsh
typeset -a NLIST_HOP_INDEXES
NLIST_HOP_INDEXES=( 1 10 )
```

`n-list` 可以按照 `Zsh` 模式自动为条目着色。下面的例子会把所有数字着成蓝色:

```zsh
local NLIST_COLORING_PATTERN="[0-9]##"
local NLIST_COLORING_COLOR=$'\x1b[00;34m'
local NLIST_COLORING_END_COLOR=$'\x1b[0m'
local NLIST_COLORING_MATCH_MULTIPLE=1

n-list "This is a number 123" "This line too has a number: 456"
```

蓝色是默认颜色,不必显式设置。更多关于 `Zsh` 模式的信息见 `zshexpn` 手册页。
简单来说,与正则表达式对比:`(#s)` 相当于 `^`,`(#e)` 相当于 `$`,`#` 相当于
`*`,`##` 相当于 `+`。或选项要放在圆括号内才生效,即 `(a|b)`。顺带一提,借助
这个方法,你还可以通过各工具的配置文件给工具的输出着色(例如可以看看
n-cd.conf,它就用到了这一点)。

## 性能

`ZNT` 在 `5.0.6` 之前以及 `5.2` 及以上的 `Zsh` 上运行最快

## 一点提示

Zsh 插件可能看起来很吓人,似乎有一套「架构」。实际上,插件的真实面目不过是:

1. 它把自己的目录加入了 `fpath`
2. 它的第一个 `*.plugin.zsh` 文件会被 source

就是这么简单。无论是给 Oh-My-Zsh 贡献代码,还是为任何插件管理器创建插件,
都只需要考虑这两点。任何非典型的 Zsh Navigation Tools 安装方式也是如此。

## 更多

- 请留意[这个](https://github.com/zdharma-continuum/zsh-navigation-tools/blob/f49f910d239ae5bc6e1a5bb34930307b4f4e3ffe/zsh-navigation-tools.plugin.zsh#L35-L49)

# 修复 tmux、screen 和 Linux 虚拟终端

如果 `TERM=screen-256color`(`tmux` 和 `screen` 会话中经常如此),那么 terminfo
的 `ncv` 能力中第 `2` 位会被置位。这通常意味着下划线无法使用。要修复这个问题,
可以自己生成一个带 `ncv=0` 的 terminfo 文件,运行:

```zsh
{ infocmp -x screen-256color; printf '\t%s\n' 'ncv@,'; } > /tmp/t && tic -x /tmp/t
```

文件会生成在 `~/.terminfo` 目录中并被自动使用,`tmux` 和 `screen` 就能正常
工作了。Linux 虚拟终端的做法类似:

```zsh
{ infocmp -x linux; printf '\t%s\n' 'ncv@,'; } > /tmp/t && tic -x /tmp/t
```

它无法正确显示下划线,但会改用颜色来高亮,效果也不错。同样的方法对 FreeBSD 的
vt 行不通,`ZNT` 会检测当前是否在使用这种 vt,并回退为以 `reverse` 模式高亮元素。

[gitter-image]: https://badges.gitter.im/zdharma-continuum/community.svg
[gitter-link]: https://gitter.im/zdharma-continuum/community
