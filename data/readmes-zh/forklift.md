# forklift

面向 ForkLift 的插件。ForkLift 是 OS X 上的一款 FTP 应用。

✅ 启用方式:把「forklift」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 依赖要求

* [ForkLift](https://binarynights.com/)

## 用法

`fl [<file_or_folder>]`

* 如果不带参数调用 `fl`,则会在 ForkLift 中打开当前文件夹。这等价于 `fl .`。

* 如果以目录作为参数调用 `fl`,则会在 ForkLift 中打开该目录

* 如果以非目录的文件作为参数调用 `fl`,则会打开该文件所在的父目录。
