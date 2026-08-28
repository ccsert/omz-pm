# iTerm2 插件

本插件提供了一些在使用 [iTerm2](https://www.iterm2.com/) 时很实用的功能。


✅ 启用方式:把「iterm2」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

此外,插件还可以选择应用 [iTerm2 的 Shell Integration Script](https://iterm2.com/documentation-shell-integration.html)。
你可以通过 zstyle 启用该集成。注意这一行必须写在 source oh-my-zsh 那一行之前:

```
zstyle :omz:plugins:iterm2 shell-integration yes
```

## 插件命令

* `_iterm2_command <iterm2-command>`
  通过转义码序列执行任意的 iTerm2 命令。
  全部受支持的命令见 https://iterm2.com/documentation-escape-codes.html 。

* `iterm2_profile <profile-name>`
  更改当前终端窗口的 profile(配色、字体、设置等)。
  `profile-name` 是另一个 iTerm2 profile 的名称。profile 名称中可以包含空格。

* `iterm2_tab_color <red> <green> <blue>`
  更改 iTerm2 当前活动标签页的颜色。
  `red`/`green`/`blue` 的取值范围为 0-255。

* `iterm2_tab_color_reset`
  把 iTerm2 当前标签页的颜色重置回默认值。


shell 集成功能请参见[官方文档](https://iterm2.com/documentation-shell-integration.html)。

## 贡献者

- [Aviv Rosenberg](https://github.com/avivrosenberg)
