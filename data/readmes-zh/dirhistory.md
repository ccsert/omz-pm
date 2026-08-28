# Dirhistory 插件

本插件提供用于在目录历史和目录层级之间导航的键盘快捷键。

✅ 启用方式:把「dirhistory」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 键盘快捷键

| 快捷键                            | 说明                                                       |
|-----------------------------------|-----------------------------------------------------------|
| <kbd>Alt</kbd> + <kbd>Left</kbd>  | 回到上一个目录                                             |
| <kbd>Alt</kbd> + <kbd>Right</kbd> | 前往下一个目录                                             |
| <kbd>Alt</kbd> + <kbd>Up</kbd>    | 进入上级目录                                               |
| <kbd>Alt</kbd> + <kbd>Down</kbd>  | 进入按字母序排列的第一个子目录                             |

**macOS:请用 Option 键(<kbd>⌥</kbd>)代替 <kbd>Alt</kbd>**。

> 注意:某些终端可能会覆盖 <kbd>Alt</kbd> + 方向键的键位绑定(例如 Windows Terminal)。
> 如果这些快捷键不起作用,请检查你的终端设置,把它们改成别的键盘快捷键。

## 用法

本插件允许你使用 <kbd>Alt</kbd> + <kbd>Left</kbd> 和 <kbd>Alt</kbd> + <kbd>Right</kbd>
在过往工作目录的历史中导航。<kbd>Alt</kbd> + <kbd>Left</kbd> 跳转到更早的目录,
<kbd>Alt</kbd> + <kbd>Right</kbd> 则回到更近的目录。

**注意:目录历史的最大长度为 30。**

你还可以使用 <kbd>Alt</kbd> + <kbd>Up</kbd> 和 <kbd>Alt</kbd> + <kbd>Down</kbd> 在
**目录层级**中导航。<kbd>Alt</kbd> + <kbd>Up</kbd> 进入上级目录,而 <kbd>Alt</kbd> + <kbd>Down</kbd>
进入按字母序找到的第一个子目录(适合在很长的空目录里导航,例如 Java 包)。

举例来说,假设启动 shell 后依次输入了以下命令:

```shell
cd ~
cd /usr
cd share
cd doc
```

那么目录栈(`dirs -v`)会是这样:

```console
$ dirs -v
0       /usr/share/doc
1       /usr/share
2       /usr
3       ~
```

此时在提示符下按 <kbd>Alt</kbd> + <kbd>Left</kbd>,目录会从 `/usr/share/doc` 切换到 `/usr/share`,
再按一次切换到 `/usr`,然后是 `~`。如果按的是 <kbd>Alt</kbd> + <kbd>Right</kbd>,目录则又会
切换回 `/usr`。

之后,按 <kbd>Alt</kbd> + <kbd>Down</kbd> 很可能会进入 `/usr/bin`(前提是 `bin` 是按字母序
排列的第一个目录,取决于你的 `/usr` 目录结构)。<kbd>Alt</kbd> + <kbd>Up</kbd> 会回到 `/usr`,
再按一次就会到达根目录(`/`)。

### cde

本插件还提供了一个 `cde` 别名,让你在切换目录时不清空「未来」目录栈。这改变了 `dirhistory`
的默认行为——默认情况下,切换目录会清空下一个目录栈。

举例来说,假设启动 shell 后依次输入了以下命令:

```shell
cd ~
cd /usr
cd share
cd doc

# <Alt + Left>
# <Alt + Left>
```

目录栈会是这样:

```sh
➜  /usr typeset -pm dirhistory_\*
typeset -ax dirhistory_past=( /home/user /usr )
typeset -ax dirhistory_future=( /usr/share/doc /usr/share )
```

这意味着此时按 <kbd>Alt</kbd> + <kbd>Right</kbd>,你会依次前往 `/usr/share` 和 `/usr/share/doc`
(即「未来」目录)。

如果你运行 `cd /usr/bin`,「未来」目录会被清空,你就无法再用 <kbd>Alt</kbd> + <kbd>Right</kbd>
访问它们了:

```sh
➜  /u/bin typeset -pm dirhistory_\*
typeset -ax dirhistory_past=( /home/user /usr )
typeset -ax dirhistory_future=( /usr/bin )
```

如果你改用 `cde /usr/bin`,「未来」目录则会被保留:

```sh
➜  /u/bin typeset -pm dirhistory_\*
typeset -ax dirhistory_past=( /home/user /usr /usr/bin )
typeset -ax dirhistory_future=( /usr/share/doc /usr/share )
```
