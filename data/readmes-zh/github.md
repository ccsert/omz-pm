# github 插件

本插件支持在命令行中使用 GitHub。它提供以下几项内容:

* 如果你安装了 [`hub`](https://github.com/github/hub),会为 `git` 命令设置 `hub` 包装器(wrapper)和补全。
* 为 [`github` Ruby gem](https://github.com/defunkt/github-gem) 提供补全。
* 提供一些用于处理仓库和 URL 的便捷函数。

### 函数

* `empty_gh` - 创建一个新的空仓库(带 `README.md`)并推送到 GitHub
* `new_gh` - 把现有目录初始化为仓库并推送到 GitHub
* `exist_gh` - 接收一个现有仓库并将其推送到 GitHub


## 安装

如果你想使用 [Hub](https://github.com/github/hub),需要先安装它。在装有 Homebrew 的 OS X 上,可以用 `brew install hub` 完成。`hub` 的补全定义需要在初始化 OMZ 之前加入你的 `$FPATH`。

如果你想使用 [`github` Ruby gem](https://github.com/defunkt/github-gem),需要先安装它。

### 配置

这些设置会影响 `github` 的行为。

#### 环境变量

* `$GITHUB_USER`
* `$GITHUB_PASSWORD`

#### Git 配置项

* `github.user` - 用于仓库操作的 GitHub 用户名

更多细节请参阅 `man hub`。

### Homebrew 安装说明

如果你是用 Homebrew 安装的 `hub`,并且你使用的是系统自带的 `zsh`,那么它的补全可能不在你的 `$FPATH` 上。Homebrew 会把 `zsh` 补全定义安装到 `/usr/local/share/zsh/site-functions`,该路径位于 Homebrew 安装的 `zsh` 的 `$FPATH` 中,但不在系统 `zsh` 的 `$FPATH` 中。如果你想让它配合系统 `zsh` 工作,请在 `~/.zshrc` 中 source `oh-my-zsh.sh` 之前加入下面的内容。

```zsh
if (( ! ${fpath[(I)/usr/local/share/zsh/site-functions]} )); then
  FPATH=/usr/local/share/zsh/site-functions:$FPATH
fi
```
