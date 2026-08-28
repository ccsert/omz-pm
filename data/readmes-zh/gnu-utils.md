# gnu-utils 插件

本插件把 GNU coreutils 绑定到它们的默认名称上,这样你就不必用带 `g` 前缀的名称来调用它们。在默认不安装 GNU coreutils 的系统上(主要是 macOS 和 FreeBSD,它们使用 BSD coreutils),这会很有用。

✅ 启用方式:把「gnu-utils」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

本插件的工作原理是修改命令哈希所指向的路径:于是 `ls` 不再指向 `/bin/ls`,而是指向 `gls` 的安装位置。

由于 `hash -rf` 或 `rehash` 会刷新命令哈希,插件还包装了 `hash` 和 `rehash`,以便在调用这两个命令之后,coreutils 的绑定总能重新完成。

请查看插件的源代码,了解尝试重新绑定的有哪些 GNU coreutils。如有遗漏,欢迎提交 issue。

## 其他方法

插件还记录了另外两种实现方式:

1. 使用函数包装:例如存在一个名为 `ls` 的函数,它实际调用的是 `gls`。由于函数的优先级高于命令,最终调用的就是 GNU coreutils。函数的优先级也高于 shell 内建命令(会调用 `gecho` 而不是内建的 `echo`)。

2. 使用别名。别名的优先级比函数更高,但它们可能因为用户自己的设置而被覆盖。

## 作者

- [Sorin Ionescu](https://github.com/sorin-ionescu)。
