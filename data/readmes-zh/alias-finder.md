# alias-finder 插件

本插件会在已定义的别名中搜索,并输出所有与所输入命令匹配的别名。这让学习新别名变得更容易。

## 设置

✅ 启用方式:把「alias-finder」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

要让它在每一条命令上都生效,请在你的 `~/.zshrc` 中设置 zstyle。

如果用户安装了 `rg`([ripgrep](https://github.com/BurntSushi/ripgrep)),插件会优先使用它,因为它速度更快;否则会使用 `grep` 命令。

```zsh
# ~/.zshrc

zstyle ':omz:plugins:alias-finder' autoload yes # disabled by default
zstyle ':omz:plugins:alias-finder' longer yes # disabled by default
zstyle ':omz:plugins:alias-finder' exact yes # disabled by default
zstyle ':omz:plugins:alias-finder' cheaper yes # disabled by default
```

如上所示,这些选项同样可以通过 zstyle 来设置。

## 用法

当你执行一条命令时,alias finder 会查看你已定义的别名,并提示你本可以使用的更短别名,例如:

执行未设置别名的 `git status` 命令:
```sh
╭─tim@fox ~/repo/gitopolis ‹main›
╰─$ git status

gst='git status'         # <=== shorter suggestion from alias-finder

On branch main
Your branch is up-to-date with 'origin/main'.
nothing to commit, working tree clean
```

执行它建议的、来自 `.gitconfig` 的更短别名 `git st`:
```sh
╭─tim@fox ~/repo/gitopolis ‹main›
╰─$ git st
gs='git st'         # <=== shorter suggestion from alias-finder
## main...origin/main
```

执行它找到的最短的 `gs` shell 别名:
```sh
╭─tim@fox ~/repo/gitopolis ‹main›
╰─$ gs
         # <=== no suggestions alias-finder because this is the shortest
## main...origin/main
```

![image](https://github.com/ohmyzsh/ohmyzsh/assets/19378/39642750-fb10-4f1a-b7f9-f36789eeb01b)


### 选项

> 为了便于说明,假设 `alias a=abc` 中源(source)是 'abc',目标(destination)是 'a'。

- 使用 `--longer` 或 `-l`:包含源比输入更长的别名(换句话说,源可以包含整个输入)。
- 使用 `--exact` 或 `-e`:排除源比输入更短的别名(换句话说,源必须与输入完全相同)。
- 使用 `--cheaper` 或 `-c`:排除目标比输入更长的别名(换句话说,目标必须比输入更短)。
