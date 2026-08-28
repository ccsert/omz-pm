# Colemak 插件

本插件把 `zsh` 的 [`vi` 风格导航模式](http://zsh.sourceforge.net/Doc/Release/Zsh-Line-Editor.html#Keymaps)中的按键,针对 [Colemak](https://colemak.com/) 键盘布局重新映射,使其对应 QWERTY 键盘上的位置:

![Colemak layout on a US keyboard](https://colemak.com/wiki/images/6/6c/Colemak2.png)

✅ 启用方式:把「colemak」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

你还需要启用 `vi` 模式,因此请在 `~/.zshrc` 中再加一行:
```
bindkey -v
```

重启你的 shell,按下 `<ESC>` 键激活 `vicmd`(导航)模式,然后用新的键位绑定来操作 `zsh` 吧!

## vicmd 的键位绑定

| 原按键     | 新按键     | 绑定                      | 说明                                               |
|------------|------------|---------------------------|----------------------------------------------------|
| `CTRL`+`j` | `CTRL`+`n` | accept-line               | 插入新行                                           |
| `j`        | `n`        | down-line-or-history      | 下移一行,或在命令历史中向后一条移动                |
| `k`        | `e`        | up-line-or-history        | 上移一行,或在命令历史中向前一条移动                |
| `l`        | `i`        | vi-forward-char           | 向右移动一个字符                                    |
| `n`        | `k`        | vi-repeat-search          | 向前重复命令搜索                                    |
| `N`        | `K`        | vi-rev-repeat-search      | 向后重复命令搜索                                    |
| `i`        | `u`        | vi-insert                 | 进入插入模式                                        |
| `I`        | `U`        | vi-insert-bol             | 移动到第一个非空白字符并进入插入模式                 |
| `<none>`   | `l`        | vi-undo-change            | 撤销更改                                            |
| `J`        | `N`        | vi-join                   | 把当前行与下一行合并                                |
| `e`        | `j`        | vi-forward-word-end       | 移动到下一个词的末尾                                |
| `E`        | `J`        | vi-forward-blank-word-end | 移动到当前或下一个词的末尾                          |

## less 的键位绑定

| 快捷键            | `less` 键位绑定    |
|-------------------|--------------------|
| `n`               | forw-line          |
| `e`               | back-line          |
| `k`               | repeat-search      |
| `ESC`+`k`         | repeat-search-all  |
| `K`               | reverse-search     |
| `ESC`+`K`         | reverse-search-all |
