# Colored man pages 插件

本插件为 man 手册页添加颜色。

✅ 启用方式:把「colored-man-pages」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

它还会自动为 `dman` 或 `debman` 显示的手册页着色,这两个工具来自 [`debian-goodies`](https://packages.debian.org/stable/debian-goodies)。

你还可以在相应命令前加上 `colored`,尝试为其他页面着色:

```zsh
colored git help clone
```

## 自定义

插件声明了全局关联数组 `less_termcap`,它把 termcap 能力映射为 `less` 分页器使用的转义序列。
插件加载之后,你可以进一步自定义这个映射。详情请查看源码。

例如:`less_termcap[md]` 对应 `LESS_TERMCAP_md`,它是告诉 `less` 如何以粗体打印内容的转义序列。
当前它显示为粗体红色;如果你想修改,可以在 OMZ 被 source 之后,在你的 zshrc 文件中重新定义 `less_termcap[md]`:

```zsh
less_termcap[md]="${fg_bold[blue]}" # this tells less to print bold text in bold blue
```
