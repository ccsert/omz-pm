# Singlechar 插件

本插件为一些命令添加了单字符快捷方式(及其组合)。

✅ 启用方式:把「singlechar」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

### CAT, GREP, CURL, WGET

| 别名 | 命令 | 说明 |
|------|------|------|
| y | `grep -Ri` | 递归地在所有文件和目录中查找不区分大小写的字符串。会跟随符号链接。 |
| n | `grep -Rvi` | 同上,但只显示不匹配该字符串的行。 |
| f | `grep -Rli` | 与 'y' 相同,但只打印找到该字符串的文件名。 |
| fn | `grep -Rlvi` | 同上,但只显示不包含该字符串的文件。 |
| f. | `find . \| grep` | 对当前目录的文件列表执行 grep |
| f: | `find` | 'find' 命令 |
| p | `less` | 'less' 命令 |
| m | `man` | 'man' 命令 |
| d | `wget` | 'wget' 命令 |
| u | `curl` | 'curl' 命令 |
| c | `cat` | 'cat' 命令 |
| w | `echo >` | 把参数写入文件,若文件已存在则覆盖。 |
| a | `echo >>` | 把参数写入文件,若文件已存在则追加。 |
| w: | `cat >` | 把 stdin 写入文件,若文件已存在则覆盖。 |
| a: | `cat >>` | 把 stdin 写入文件,若文件已存在则追加。 |

### XARGS

以下别名是上面那些别名的 xargs 版本。把参数通过管道传给这些 xargs 别名即可使用。

| 别名 | 命令 | 说明 |
|------|------|------|
| x | `xargs` | 'xargs' 命令 |
| xy | `xargs grep -Ri` | 与 'y' 别名相同,但使用 xargs。 |
| xn | `xargs grep -Rvi` | 与 'n' 别名相同,但使用 xargs。 |
| xf | `xargs grep -Rli` | 与 'f' 别名相同,但使用 xargs。 |
| xfn | `xargs grep -Rlvi` | 与 'fn' 别名相同,但使用 xargs。 |
| xf. | `xargs find \| grep` | 与 'f.' 别名相同,但使用 xargs。 |
| xf: | `xargs find` | 与 'f:' 别名相同,但使用 xargs。 |
| xc | `xargs cat` | 与 'c' 别名相同,但使用 xargs。 |
| xp | `xargs less` | 与 'p' 别名相同,但使用 xargs。 |
| xm | `xargs man` | 与 'm' 别名相同,但使用 xargs。 |
| xd | `xargs wget` | 与 'd' 别名相同,但使用 xargs。 |
| xu | `xargs curl` | 与 'u' 别名相同,但使用 xargs。 |
| xw | `xargs echo >` | 与 'w' 别名相同,但使用 xargs。 |
| xa | `xargs echo >>` | 与 'a' 别名相同,但使用 xargs。 |
| xw: | `xargs cat >` | 与 'w:' 别名相同,但使用 xargs。 |
| xa: | `xargs >>` | 与 'a:' 别名相同,但使用 xargs。 |

### SUDO

以下别名是上文 [CAT, GREP, CURL, WGET](#cat-grep-curl-wget) 中那些别名的版本,使用 sudo 以 root 权限运行它们。

| 别名 | 命令 | 说明 |
|------|------|------|
| s | `sudo` | 'sudo' 命令 |
| sy | `sudo grep -Ri` | 与 'y' 别名相同,但使用 sudo。 |
| sn | `sudo grep -Riv` | 与 'n' 别名相同,但使用 sudo。 |
| sf | `sudo grep -Rli` | 与 'f' 别名相同,但使用 sudo。 |
| sfn | `sudo grep -Rlvi` | 与 'fn' 别名相同,但使用 sudo。 |
| sf. | `sudo find . \| grep` | 与 'f.' 别名相同,但使用 sudo。 |
| sf: | `sudo find` | 与 'f:' 别名相同,但使用 sudo。 |
| sp | `sudo less` | 与 'p' 别名相同,但使用 sudo。 |
| sm | `sudo man` | 与 'm' 别名相同,但使用 sudo。 |
| sd | `sudo wget` | 与 'd' 别名相同,但使用 sudo。 |
| sc | `sudo cat` | 与 'c' 别名相同,但使用 sudo。 |
| sw | `sudo echo >` | 与 'w' 别名相同,但使用 sudo。 |
| sa | `sudo echo >>` | 与 'a' 别名相同,但使用 sudo。 |
| sw: | `sudo cat >` | 与 'w:' 别名相同,但使用 sudo。 |
| sa: | `sudo cat >>` | 与 'a:' 别名相同,但使用 sudo。 |

### SUDO-XARGS

与上一节相同,但同时使用 sudo 和 xargs。

| 别名 | 命令 | 说明 |
|------|------|------|
| sx | `sudo xargs` | 'sudo xargs' 命令 |
| sxy | `sudo xargs grep -Ri` | 与 'xy' 别名相同,但使用 sudo。 |
| sxn | `sudo xargs grep -Riv` | 与 'xn' 别名相同,但使用 sudo。 |
| sxf | `sudo xargs grep -li` | 与 'xf' 别名相同,但使用 sudo。 |
| sxfn | `sudo xargs grep -lvi` | 与 'xfn' 别名相同,但使用 sudo。 |
| sxf. | `sudo xargs find \| grep` | 与 'xf.' 别名相同,但使用 sudo。 |
| sxf: | `sudo xargs find` | 与 'xf:' 别名相同,但使用 sudo。 |
| sxp | `sudo xargs less` | 与 'xp' 别名相同,但使用 sudo。 |
| sxm | `sudo xargs man` | 与 'xm' 别名相同,但使用 sudo。 |
| sxd | `sudo xargs wget` | 与 'xd' 别名相同,但使用 sudo。 |
| sxu | `sudo xargs curl` | 与 'xu' 别名相同,但使用 sudo。 |
| sxc | `sudo xargs cat` | 与 'xc' 别名相同,但使用 sudo。 |
| sxw | `sudo xargs echo >` | 与 'xw' 别名相同,但使用 sudo。 |
| sxa | `sudo xargs echo >>` | 与 'xa' 别名相同,但使用 sudo。 |
| sxw: | `sudo xargs cat >` | 与 'xw:' 别名相同,但使用 sudo。 |
| sxa: | `sudo xargs cat >>` | 与 'xa:' 别名相同,但使用 sudo。 |

## 选项

在 source Oh My Zsh 之前,可以通过下面的设置变量,把 `grep`、`sudo`、`wget`、`curl` 和 `less` 命令配置为使用其他命令。如果尚未设置,它们将使用各自的默认值:

| 设置变量 | 默认值 |
|----------|--------|
| GREP | `grep` |
| ROOT | `sudo` |
| WGET | `wget` |
| CURL | `curl` |
| PAGER | `less` |

## 作者

- [Karolin Varner](https://github.com/koraa)
