# shrink-path

一个用于缩短目录路径、让显示更简洁美观的插件。

✅ 启用方式:把「shrink-path」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 示例

对于下面这棵目录树:
```
/home/
  me/
    f o o/     # The prefix f is ambiguous between "f o o" and "f i g".
      bar/
        quux/
      biz/     # The prefix b is ambiguous between bar and biz.
    f i g/
      baz/
```
以下是调用 `shrink_path <option> /home/me/foo/bar/quux` 的结果:
```
Option        Result
<none>        /h/m/f o/ba/q
-l|--last     /h/m/f o/ba/q
-s|--short    /h/m/f/b/q
-t|--tilde    ~/f o/ba/q
-f|--fish     ~/f/b/quux
-g|--glob     /h*/m*/f o*/ba*/q*
-3            /hom/me/f o/bar/quu
-e '$' -3     /hom$/me/f o$/bar/quu$
-q            /h/m/f\ o/ba/q
-g -q         /h*/m*/f\ o*/ba*/q*
-x            /home/me/foo/bar/quux
```

## 用法

如果想让命令提示符显示 fish 风格的工作目录,请把下面几行加进你的主题或 zshrc:

```zsh
setopt prompt_subst
PS1='%n@%m $(shrink_path -f)>'
```

可用选项如下:

```
    -f, --fish       fish simulation, equivalent to -l -s -t.
    -g, --glob       Add asterisk to allow globbing of shrunk path (equivalent to -e "*")
    -l, --last       Print the last directory's full name.
    -s, --short      Truncate directory names to the number of characters given by -. Without
                     -s, names are truncated without making them ambiguous.
    -t, --tilde      Substitute ~ for the home directory.
    -T, --nameddirs  Substitute named directories as well.
    -#               Truncate each directly to at least this many characters inclusive of the
                     ellipsis character(s) (defaulting to 1).
    -e SYMBOL        Postfix symbol(s) to indicate that a directory name had been truncated.
    -q, --quote      Quote special characters in the shrunk path
    -x, --expand     Print the full path. This takes precedence over the other options
```

长选项也可以通过 zstyle 来设置,例如:
```zsh
zstyle :prompt:shrink_path fish yes
```

注意:尚不支持包含两个及以上连续空格的目录名。


## 技巧:用键盘快捷键切换路径缩短

你可以用 `expand` 选项来关闭路径缩短。再把它和一个键绑定 widget 结合起来,就可以随时开关路径缩短。

```zsh
# Toggle off path shrinking
zstyle ':prompt:shrink_path' expand true
# Toggle on path shrinking
zstyle -d ':prompt:shrink_path' expand
```

与 widget 结合使用:

```zsh
# Widget definition
shrink-path-toggle() {
  zstyle -t ':prompt:shrink_path' expand \
    && zstyle -d ':prompt:shrink_path' expand \
    || zstyle ':prompt:shrink_path' expand true
  zle reset-prompt
}
zle -N shrink-path-toggle
# Key binding to ALT+SHIFT+S
bindkey "^[S" shrink-path-toggle
```

## 许可证

Copyright (C) 2008 by Daniel Friesel <derf@xxxxxxxxxxxxxxxxxx>
Copyright (C) 2018-2020 by Pavel N. Krivitsky

许可证:WTFPL <http://www.wtfpl.net>

参考:https://www.zsh.org/mla/workers/2009/msg00415.html
     https://www.zsh.org/mla/workers/2009/msg00419.html


## 其他

关键词:prompt directory truncate shrink collapse fish
