# Pipenv

本插件提供一些特性,简化你在 ZSH 中使用 [Pipenv](https://pipenv.pypa.io/) 的过程。

✅ 启用方式:把「pipenv」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 特性

- 为 pipenv 提供补全([要让它支持 pipenv >= 2026.5.0,需安装 `argcomplete` 包](https://pipenv.pypa.io/en/latest/shell.html#shell-completion))
- 自动激活和停用 pipenv shell
- 为常用的 pipenv 命令提供简短别名
  - `pch` 是 `pipenv check` 的别名
  - `pcl` 是 `pipenv clean` 的别名
  - `pgr` 是 `pipenv graph` 的别名
  - `pi` 是 `pipenv install` 的别名
  - `pidev` 是 `pipenv install --dev` 的别名
  - `pl` 是 `pipenv lock` 的别名
  - `po` 是 `pipenv open` 的别名
  - `prun` 是 `pipenv run` 的别名
  - `psh` 是 `pipenv shell` 的别名
  - `psy` 是 `pipenv sync` 的别名
  - `pu` 是 `pipenv uninstall` 的别名
  - `pupd` 是 `pipenv update` 的别名
  - `pwh` 是 `pipenv --where` 的别名
  - `pvenv` 是 `pipenv --venv` 的别名
  - `ppy` 是 `pipenv --py` 的别名

## 缓存

本插件会缓存 Pipenv 的版本号,以避免每次 shell 启动时都运行 `pipenv --version`。插件加载时会异步地自动刷新缓存,而插件加载通常发生在你开启新终端会话的时候。

对于旧版 Pipenv,生成的补全脚本也会被缓存。

缓存保存在:

- `$ZSH_CACHE_DIR/pipenv_version` Pipenv 的版本号,用于在基于 argcomplete 的补全与旧版基于 Click 的补全之间做选择。

- `$ZSH_CACHE_DIR/completions/_pipenv` 旧版基于 Click 的补全脚本。

## 配置

### Shell 激活

如果你想禁用 shell 的自动激活和停用特性,请在 source `oh-my-zsh.sh` 之前,把下面的 style 加入你的 `.zshrc`:

```zsh
zstyle ':omz:plugins:pipenv' auto-shell no
```
