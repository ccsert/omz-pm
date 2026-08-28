# fnm 插件

本插件为 [fnm](https://github.com/Schniz/fnm)(一个 Node.js 版本管理器)提供自动补全。

✅ 启用方式:把「fnm」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 配置

这些设置应写在 `.zshrc` 文件中、加载 Oh My Zsh 之前。

### 自动启动(Autostart)

设置后,插件会在会话中自动启动 fnm,即运行 `fnm env`:

```zsh
zstyle ':omz:plugins:fnm' autostart yes
```

默认值:`no`(关闭)

### cd 时自动切换(Use on cd)

设置后,Node.js 版本会根据当前目录的要求自动切换(推荐):

```zsh
zstyle ':omz:plugins:fnm' use-on-cd yes
```

默认值:`yes`(开启)

可用的 fnm 变量请参阅[官方文档](https://github.com/Schniz/fnm/blob/master/docs/commands.md)。
