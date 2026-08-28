# Vim 交互插件 #

这个脚本的设计初衷,是让你能和正在运行的 GVim 会话进行得体的交互。通常你会在文件系统里
四处奔走、做各种神奇的事情,期间免不了要把一些文件装进 GVim,用于编辑、检查、破坏,
或者其他种种折腾。这个脚本就能帮你做到。

## 用法

插件提供了一个名为 `callvim` 的函数,其用法是:

    usage: callvim [-b cmd] [-a cmd] [file ... fileN]

      -b cmd     Run this command in GVIM before editing the first file
      -a cmd     Run this command in GVIM after editing the first file
      file       The file to edit
      ... fileN  The other files to add to the argslist

## 别名 ##

另外还提供了几个别名:

* `v` 是 `callvim` 的简写
* `vvsp` 编辑传入的文件,但会先做一次垂直分割
* `vhsp` 编辑传入的文件,但会先做一次水平分割

## 调用后回调 ##

在 `callvim` 函数的末尾,如果 `postCallVim` 函数存在,我们就会调用它。
比如你在使用 MacVim,就可以定义一个函数,在文件加载完成后把窗口焦点切换到它上面:

    function postCallVim
    {
      osascript -e 'tell application "MacVim" to activate'
    }

具体做法因你的操作系统/窗口管理器而异。

## 示例 ##

这会把 `/tmp/myfile.scala` 加载进正在运行的 GVim 会话:

    > v /tmp/myfile.scala

这会先做一次垂直分割,然后加载文件:

    > vvsp /tmp/myfile.scala
    or
    > v -b':vsp' /tmp/myfile.scala

这会先做一次水平分割并跳到文件底部,然后加载文件:

    > vhsp -aG /tmp/myfile.scala
    or
    > v -b':sp' -aG /tmp/myfile.scala

这会加载文件,然后把第一行复制到末尾(你为什么会想这么做……我也不知道):

    > v -a':1t$' /tmp/myfile.scala

而这会把所有 `*.txt` 文件都装进参数列表:

    > v *.txt

如果你想把文件加载到已经分割好的区域,可以使用专门的别名:

    # Do a ':wincmd h' first
    > vh /tmp/myfile.scala

    # Do a ':wincmd j' first
    > vj /tmp/myfile.scala

    # Do a ':wincmd k' first
    > vk /tmp/myfile.scala

    # Do a ':wincmd l' first
    > vl /tmp/myfile.scala
