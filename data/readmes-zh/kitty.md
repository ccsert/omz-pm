# Kitty 插件

本插件为 [Kitty](https://sw.kovidgoyal.net/kitty/) 终端的用户提供几个实用的别名和函数。

✅ 启用方式:把「kitty」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 插件命令

* `kssh`
  运行一个 kitten ssh 会话,确保你的 terminfo 设置被正确复制到远程主机上。
* `kssh-slow`
  `kssh` 的较慢版本,但应当总是可用。如果 `kssh` 未能为你在远程主机上正确设置
  terminfo,请改用这个。
* `kitty-theme`
  浏览并更换 Kitty 终端的主题。

## 贡献者

- [Ian Chesal](https://github.com/ianchesal)
