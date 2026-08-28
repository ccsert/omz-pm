# TextMate 插件

本插件为 [TextMate](https://macromates.com) 编辑器添加一个函数。

✅ 启用方式:把「textmate」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 函数

`tm` 函数提供以下选项:

- 不带参数:在当前目录运行 `mate`。
- 参数是目录:在给定目录运行 `mate` 并 cd 到该目录。
- 其他参数:把所有参数传给 `mate`。这样可以轻松打开多个文件。
