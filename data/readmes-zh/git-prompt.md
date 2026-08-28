# git-prompt 插件

一个用于显示当前 git 仓库信息的 `zsh` 提示符。具体包括:分支名、与远程分支的差异、已暂存或已更改的文件数量等。

✅ 启用方式:把「git-prompt」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

你还可能需要[自定义你的主题](https://github.com/ohmyzsh/ohmyzsh/issues/9395#issuecomment-1027130429),
来改变提示符的构建方式。参见
[OMZ wiki 上关于自定义主题的说明](https://github.com/ohmyzsh/ohmyzsh/wiki/Customization#overriding-and-adding-themes)。

另见[原始仓库](https://github.com/olivierverdier/zsh-git-prompt)。

## 依赖要求

本插件使用 `python3`,因此你的主机需要安装它。

## 示例

提示符可能形如下面这样:

- `(master↑3|✚1)`:位于 `master` 分支,领先远程 3 个提交,1 个文件已更改但未暂存
- `(status|●2)`:位于 `status` 分支,2 个文件已暂存
- `(master|✚7…)`:位于 `master` 分支,7 个文件已更改,还有若干未跟踪的文件
- `(master|✖2✚3)`:位于 `master` 分支,2 处冲突,3 个文件已更改
- `(experimental↓2↑3|✔)`:位于 `experimental` 分支;你的分支与远程已分叉:你领先 3 个提交,远程领先 2 个提交;除此之外仓库是干净的
- `(:70c2952|✔)`:不在任何分支上;父提交的哈希为 `70c2952`;除此之外仓库是干净的
- `(master|⚑2)`:位于 `master` 分支,有 2 条 stash 的改动

## 提示符结构

默认情况下,提示符的总体外观是:

```text
(<branch><branch tracking>|<local status>)
```

各符号含义如下:

### 本地状态符号

| 符号   | 含义                            |
|--------|--------------------------------|
| ✔      | 仓库干净                        |
| ●n     | 有 `n` 个已暂存的文件           |
| ✖n     | 有 `n` 个未合并的文件           |
| ✚n     | 有 `n` 个未暂存的文件           |
| -n     | 有 `n` 个已删除的文件           |
| ⚑n     | 有 `n` 条 stash 的改动          |
| …      | 有一些未跟踪的文件              |

### 分支跟踪符号

| 符号   | 含义                                                          |
|--------|---------------------------------------------------------------|
| ↑n     | 领先远程 `n` 个提交                                            |
| ↓n     | 落后远程 `n` 个提交                                            |
| ↓m↑n   | 分支已分叉:对方领先 `m` 个提交,你领先 `n` 个提交              |

## 自定义

- 把变量 `ZSH_THEME_GIT_PROMPT_CACHE` 设为任意值即可启用缓存。
- 把变量 `ZSH_THEME_GIT_SHOW_UPSTREAM` 设为任意值即可显示上游分支。
- 你还可以修改若干变量(名称以 `ZSH_THEME_GIT_PROMPT_` 开头)来改变提示符的外观。
  查看[插件文件](git-prompt.plugin.zsh)的末尾,即可了解有哪些变量可用。

**祝使用愉快!**
