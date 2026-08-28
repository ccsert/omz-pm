# mise

本插件添加与 [mise](https://github.com/jdx/mise)(前身为 `rtx`)的集成。mise 是一个运行时执行器,
兼容 npm、nodenv、pyenv 等。mise 用 Rust 编写,速度非常快,比 asdf 快 20 到 200 倍。
话虽如此,mise 兼容 asdf 插件和 .tool-versions 文件,可以作为直接替代品(drop-in replacement)使用。

## 安装

1. 运行以下命令[下载并安装 mise](https://github.com/jdx/mise#installation):

```bash
curl https://mise.jdx.dev/install.sh | sh
```

2. 把 mise 加入 `~/.zshrc` 中的 `plugins` 定义,即可[启用 mise](https://github.com/jdx/mise#quickstart)。

```bash
plugins=(mise)
```

## 用法

关于如何使用 mise,请参阅 [mise readme](https://github.com/jdx/mise#table-of-contents)。下面是几个例子:

```bash
mise install node         Install the current version specified in .tool-versions/.mise.toml
mise use -g node@system   Use system node as global default
mise install node@20.0.0  Install a specific version number
mise use -g node@20       Use node-20.x as global default
```
