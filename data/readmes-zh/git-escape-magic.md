# Git Escape Magic

本插件复制自原始项目
https://github.com/knu/zsh-git-escape-magic 。本插件所启用功能的全部
功劳应归于 @knu。

下面是该项目 README 中的一段摘录,解释了它的用途。

> 它免去了手动转义这些元字符的麻烦。它提供的 zle 函数能感知上下文,并能识别 git
> 每个子命令的特点。每当你在 git 命令行上输入这类元字符时,它都会在必要且合适之处
> 自动用反斜杠将其转义。

## 用法

✅ 启用方式:把「git-escape-magic」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

**注意**:如果你使用 url-quote-magic,它必须在本插件运行之前加载,以免发生冲突。
