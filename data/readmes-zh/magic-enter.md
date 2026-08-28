## Magic Enter 插件

本插件让你的回车键变得神奇:把一些常用命令绑定到它上面。

✅ 启用方式:把「magic-enter」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。你可以在 .zshrc 中、包含 plugins 的那一行之前设置要运行的命令。在 git 目录中,如果未指定命令,则执行 `git status`;在其他目录中,则执行 `ls`。

```zsh
# defaults
MAGIC_ENTER_GIT_COMMAND='git status -u .'
MAGIC_ENTER_OTHER_COMMAND='ls -lh .'

plugins=(... magic-enter)
```

**维护者:**[@dufferzafar](https://github.com/dufferzafar)
