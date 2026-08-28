# Bitwarden(非官方)CLI 插件

本插件为 [rbw](https://github.com/doy/rbw) 提供自动补全。rbw 是
[Bitwarden](https://bitwarden.com) 的一个非官方 CLI。

✅ 启用方式:把「rbw」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## `rbwpw`

`rbwpw` 函数是 `rbw` 的一个包装。它会把你所请求服务的密码复制到剪贴板,
并在 20 秒后清空剪贴板。用法如下:

```zsh
rbwpw <service>
```

本插件不添加任何别名。
