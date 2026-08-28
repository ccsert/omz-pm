# sublime

[Sublime Text](https://www.sublimetext.com/) 是一款跨平台的文本和代码编辑器,
可用于 Linux、macOS 和 Windows,本插件就是为它而作。

✅ 启用方式:把「sublime」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

使用本插件前必须先安装 Sublime Text。

## 用法

本插件定义了若干别名,例如:

- `st`:打开 Sublime Text。如果传入了文件或目录,Sublime Text 会打开它。

- `stt`:在当前目录打开 Sublime Text。

- `sst`:如果 `sudo` 可用,`sst` 会以 root 权限打开 Sublime Text,
  这样你就可以修改传给它的任何文件或目录。编辑系统文件时很有用。

还有一些可用的函数:

- `find_project`(或 `stp` 别名):调用时,该函数会在当前目录及其各级上级目录中
  查找 `.sublime-project` 文件,直到找不到为止。

  如果没有 `.sublime-project` 文件,但当前文件夹位于某个 Git 仓库中,它会在
  该仓库的根目录打开 Sublime Text。

  如果也不存在 Git 仓库,它就会在当前目录打开 Sublime Text。

- `create_project`(或 `stn` 别名):不带参数调用时,如果当前工作目录尚不存在
  `.sublime-project` 文件,就在其中创建一个存根(stub)`.sublime-project` 文件。
  如果传入一个目录,则在该目录中创建存根 `.sublime-project` 文件。
