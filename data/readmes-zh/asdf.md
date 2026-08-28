# asdf

本插件提供与 [asdf](https://github.com/asdf-vm/asdf) 的集成。asdf 是一个可扩展的版本管理器,支持 Ruby、Node.js、Elixir、Erlang 等多种运行时。

## 安装

1. [安装](https://asdf-vm.com/guide/getting-started.html#_1-install-asdf) asdf,并确保它能在 `$PATH` 中被找到;

✅ 启用方式:把「asdf」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

关于如何添加插件以及为其安装各类运行时版本,请参阅 [asdf 插件文档](https://asdf-vm.com/guide/getting-started.html#_4-install-a-plugin)。

下面以安装 nodejs 插件及其多种运行时为例:

```sh
# Add plugin to asdf
asdf plugin add nodejs

# Install the latest available version
asdf install nodejs latest

# Uninstall the latest version
asdf uninstall nodejs latest

# Install a specific version
asdf install nodejs 16.5.0

# Set the latest version in .tool-versions of the `current directory`
asdf set nodejs latest

# Set a specific version in the `parent directory`
asdf set -p nodejs 16.5.0   # -p is shorthand for --parent

# Set a global version under `$HOME`
asdf set -u nodejs 16.5.0   # -u is shorthand for --home
```

更多命令请运行 `asdf help`,或参阅
[asdf CLI 文档](https://asdf-vm.com/manage/commands.html#all-commands)。

## 维护者

- [@RobLoach](https://github.com/RobLoach)
