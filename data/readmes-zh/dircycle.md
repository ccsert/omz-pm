# dircycle

用于在目录栈中循环切换的插件

本插件提供的目录导航体验,类似于在浏览器或 Finder、Nautilus 等常见文件管理器中使用「后退/前进」。
它利用了一个小的 zle 技巧,让你可以用 <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Left</kbd> / <kbd>Right</kbd>
在目录栈中左右循环切换。在开发环境中来回切换目录时非常好用,可以把它理解为一种无破坏性的 pushd/popd。

## 启用插件

1. ✅ 启用方式:把「dircycle」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

2. 重启 shell 或重启你的终端会话:

   ```console
   $ exec zsh
   $
   ```

## 用法示例

假设你在终端里依次打开了这些目录:

```console
~$ cd Projects
~/Projects$ cd Hacktoberfest
~/Projects/Hacktoberfest$ cd oh-my-zsh
~/Projects/Hacktoberfest/oh-my-zsh$ dirs -v
0       ~/Projects/Hacktoberfest/oh-my-zsh
1       ~/Projects/Hacktoberfest
2       ~/Projects
3       ~
```

按下 <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Left</kbd> 后,当前工作目录(即 `$PWD`)会从
`oh-my-zsh` 变为 `Hacktoberfest`。再按一次,就会变成 `Projects`。

而按下 <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Right</kbd> 时,`$PWD` 会从 `Projects`
变为 `Hacktoberfest`。再按一次,就会变成 `oh-my-zsh`。

下面是以上述同样目录为例的操作历史表:

| 当前 `$PWD`      | 按键                                                    | 新 `$PWD`        |
| --------------- | ----------------------------------------------------- | --------------- |
| `oh-my-zsh`     | <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Left</kbd>  | `Hacktoberfest` |
| `Hacktoberfest` | <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Left</kbd>  | `Projects`      |
| `Projects`      | <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Left</kbd>  | `~`             |
| `~`             | <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Right</kbd> | `Projects`      |
| `Projects`      | <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Right</kbd> | `Hacktoberfest` |
| `Hacktoberfest` | <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Right</kbd> | `oh-my-zsh`     |
| `oh-my-zsh`     | <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Right</kbd> | `~`             |

注意最后一次切换:当在最后一个已知的 `$PWD` 上按下 <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Right</kbd> 时,
会绕回到第一个已知的 `$PWD`,在本例中即 `~`。

下面是一段演示上述示例的 asciinema 录屏:

[![asciicast](https://asciinema.org/a/204406.png)](https://asciinema.org/a/204406)

## 函数

| 函数                 | 说明                                                                                                                |
| -------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `insert-cycledleft`  | 把 `$PWD` 切换到上一个已知的栈项,绑定到 <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Left</kbd>            |
| `insert-cycledright` | 把 `$PWD` 切换到下一个已知的栈项,绑定到 <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Right</kbd>               |
| `insert-cycledup`    | 把 `$PWD` 切换到父目录,绑定到 <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Up</kbd>                     |
| `insert-cycleddown`  | 把 `$PWD` 切换到按字母序排列的第一个子目录,绑定到 <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Down</kbd> |

## 重新绑定按键

你可以把这些函数绑定到其他按键序列,只要知道对应的 bindkey 序列即可。例如,下面的命令在
`xterm-256color` 中把它们绑定到 <kbd>Alt</kbd> + <kbd>Shift</kbd> + <kbd>key</kbd>:

```zsh
bindkey '^[[1;4D' insert-cycledleft
bindkey '^[[1;4C' insert-cycledright
bindkey "\e[1;4A" insert-cycledup
bindkey "\e[1;4B" insert-cycleddown
```

先按 <kbd>Ctrl</kbd> + <kbd>V</kbd>,再按下你想使用的快捷键,就能得到对应的 bindkey 序列。
