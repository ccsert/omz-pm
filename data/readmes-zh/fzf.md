# fzf

本插件会根据安装位置自动查找 [junegunn 的 fzf](https://github.com/junegunn/fzf),并启用它的模糊自动补全和键位绑定。

✅ 启用方式:把「fzf」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 设置

所有这些设置都应写进你的 zshrc 文件,并且要在 source Oh My Zsh 之前。

### `FZF_BASE`

设置为 fzf 的安装目录路径:

```zsh
export FZF_BASE=/path/to/fzf/install/dir
```

### `FZF_DEFAULT_COMMAND`

设置当输入来自 tty 时使用的默认命令:

```zsh
export FZF_DEFAULT_COMMAND='<your fzf default command>'
```

如果没有设置,插件会按找到的先后顺序依次尝试下面这些命令:

- [`fd`](https://github.com/sharkdp/fd)
- [`rg`](https://github.com/BurntSushi/ripgrep)
- [`ag`](https://github.com/ggreer/the_silver_searcher)

### `DISABLE_FZF_AUTO_COMPLETION`

设置是否加载 fzf 自动补全:

```zsh
DISABLE_FZF_AUTO_COMPLETION="true"
```

### `DISABLE_FZF_KEY_BINDINGS`

设置是否禁用键位绑定(CTRL-T、CTRL-R、ALT-C):

```zsh
DISABLE_FZF_KEY_BINDINGS="true"
```
