# git-extras

本插件为 [git-extras](https://github.com/tj/git-extras) 定义的部分命令提供补全。git-extras 必须已经安装。

✅ 启用方式:把「git-extras」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 设置说明

这些补全的实现方式是对 `zsh` 提供的 `_git` 补全进行增强。它只对 `zsh` 自带的 `_git` 有效,
对 `git` 本身自带的 `_git` 无效。如果你同时安装了 `zsh` 和 `git`,需要确保 `zsh` 提供的
`_git` 优先级更高。

### OS X Homebrew 设置

**注意:** 这条办法在当前 Homebrew 发行版的 git 上已不再适用。~~在 OS X 上如果用 Homebrew,
你需要通过 `brew install git --without-completions` 来安装 `git`。否则 `git` 自带的 `_git`
会占据优先,你将看不到 `git-extras` 命令的补全。~~
