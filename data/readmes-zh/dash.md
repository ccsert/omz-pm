# Dash 插件

本插件为 macOS 上的 API 文档浏览器 [Dash](https://kapeli.com/dash) 提供命令行功能。
插件需要先安装 Dash 才能工作。

✅ 启用方式:把「dash」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

- 打开并切换到 Dash 应用。
```
dash
```

- 在 Dash 应用中查询某样东西:`dash query`
```
dash golang
```

- 还可以选择性提供一个关键字:`dash [keyword:]query`
```
dash python:tuple
```
