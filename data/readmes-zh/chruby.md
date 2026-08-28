# chruby 插件

本插件加载 [chruby](https://github.com/postmodern/chruby)(一个切换当前 Ruby 版本的工具),
并提供自动补全和一个用于显示 Ruby 版本的提示符函数。
支持通过 brew 安装或手动安装的 chruby。

✅ 启用方式:把「chruby」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

如果你希望指定一个明确的路径来加载 chruby,可以像下面这样设置变量:

```zsh
zstyle :omz:plugins:chruby path /local/path/to/chruby.sh
zstyle :omz:plugins:chruby auto /local/path/to/auto.sh
```
