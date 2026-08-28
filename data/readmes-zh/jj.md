# jj - Jujutsu CLI

本插件为 [jj](https://martinvonz.github.io/jj) 提供自动补全。

✅ 启用方式:把「jj」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名   | 命令                          |
| ------ | ----------------------------- |
| jja    | `jj abandon`                  |
| jjb    | `jj bookmark`                 |
| jjba   | `jj bookmark advance`         |
| jjbc   | `jj bookmark create`          |
| jjbd   | `jj bookmark delete`          |
| jjbf   | `jj bookmark forget`          |
| jjbl   | `jj bookmark list`            |
| jjbm   | `jj bookmark move`            |
| jjbr   | `jj bookmark rename`          |
| jjbs   | `jj bookmark set`             |
| jjbt   | `jj bookmark track`           |
| jjbu   | `jj bookmark untrack`         |
| jjc    | `jj commit`                   |
| jjcmsg | `jj commit --message`         |
| jjd    | `jj diff`                     |
| jjdmsg | `jj desc --message`           |
| jjds   | `jj desc`                     |
| jje    | `jj edit`                     |
| jjgcl  | `jj git clone`                |
| jjgf   | `jj git fetch`                |
| jjgfa  | `jj git fetch --all-remotes`  |
| jjgp   | `jj git push`                 |
| jjgpa  | `jj git push --all`           |
| jjgpd  | `jj git push --deleted`       |
| jjgpt  | `jj git push --tracked`       |
| jjl    | `jj log`                      |
| jjla   | `jj log -r "all()"`           |
| jjn    | `jj new`                      |
| jjnt   | `jj new "trunk()"`            |
| jjrb   | `jj rebase`                   |
| jjrbm  | `jj rebase -d "trunk()"`      |
| jjrs   | `jj restore`                  |
| jjrt   | `cd "$(jj root \|\| echo .)"` |
| jjsp   | `jj split`                    |
| jjsq   | `jj squash`                   |
| jjst   | `jj status`                   |

## 提示符用法

由于 `jj` 拥有非常强大的[模板语法](https://martinvonz.github.io/jj/latest/templates/),本插件
只暴露了一个便捷函数 `jj_prompt_template`,用于读取当前 change 的信息。
它基本上等同于 `jj log --no-graph -r @ -T $1`:

```sh
_my_theme_jj_info() {
  jj_prompt_template 'self.change_id().shortest(3)'
}

PROMPT='$(_my_theme_jj_info) $'
```

`jj_prompt_template` 会对输出中的 `%` 符号做转义。如果不想转义(例如想给输出上色),
可以使用 `jj_prompt_template_raw`。

不过,由于 `jj` 可以在 Git 仓库中使用,某些主题可能会与它冲突。通常可以用一个包装函数解决:
先尝试 `jj`,如果不奏效再回退到 `git`:

```sh
_my_theme_vcs_info() {
  jj_prompt_template 'self.change_id().shortest(3)' \
  || git_prompt_info
}

PROMPT='$(_my_theme_vcs_info) $'
```

你可以在[这里](https://github.com/nasso/omzsh/blob/e439e494f22f4fd4ef1b6cb64626255f4b341c1b/themes/sunakayu.zsh-theme)
找到一个示例。

### 性能

有时 `jj` 会比 `git` 慢。

如果你感觉到变慢,可以考虑使用下面的配置:

```
zstyle :omz:plugins:jj ignore-working-copy yes
```

这会给提示符执行的所有 `jj` 命令加上 `--ignore-working-copy`。代价是,提示符可能会不同步,
直到下一次 `jj` 有机会_不_忽略工作副本为止(也就是你手动运行某条 `jj` 命令时)。

如果你希望提示符始终保持最新,但又不想_感觉_到变慢,可以把提示符改成异步的。
本插件不会自动这么做,所以你需要对自己的主题做一些改造。

### Git async-prompt 兼容性

如果你使用调用 `git_prompt_info` 的包装函数(如上所示),它在默认的 git async-prompt 模式下
不会生效。这是因为 async-prompt 只有在你的提示符变量中逐字检测到 `$(git_prompt_info)` 时,
才会注册它的后台 worker。像 `$(_my_theme_vcs_info)` 这样的包装不会匹配,于是异步输出一直是空的。

要解决这个问题,请在加载 Oh My Zsh **之前**把下面任意一项加入你的 `.zshrc`:

```zsh
# Option 1: force async handlers to always register (recommended, keeps async behavior)
zstyle ':omz:alpha:lib:git' async-prompt force

# Option 2: disable async-prompt entirely (simpler, but prompt may feel slower in large repos)
zstyle ':omz:alpha:lib:git' async-prompt no
```

详情见 [#13555](https://github.com/ohmyzsh/ohmyzsh/issues/13555)。

## 另见

- [martinvonz/jj](https://github.com/martinvonz/jj)

## 贡献者

- [nasso](https://github.com/nasso) - 插件作者
- [imp](https://github.com/imp) - 别名的偶尔贡献者
