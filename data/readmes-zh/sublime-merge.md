## sublime-merge

Sublime Merge 插件。Sublime Merge 是一个跨平台的文本与代码编辑器,可用于 Linux、Mac OS X 和 Windows。

### 依赖要求

 * [Sublime Merge](https://www.sublimemerge.com)

### 用法

 * 如果调用 `sm` 命令时不带参数,则启动 Sublime Merge

 * 如果给 `sm` 传入一个目录,则会 `cd` 到该目录并在 Sublime Merge 中打开其中已有的 git 仓库

 * 如果调用 `smt` 命令,效果等价于 `sm .`,即在 Sublime Merge 中打开当前文件夹里已有的 git 仓库

 * 如果调用 `ssm` 命令,效果类似 `sudo sm`,即在 Sublime Merge 中打开 git 仓库。适合编辑受系统保护的仓库。
