# Globalias 插件

展开所有 glob 表达式、子命令和别名(包括全局别名)。

想法来自:https://blog.patshead.com/2012/11/automatically-expaning-zsh-global-aliases---simplified.html 。

## 用法

✅ 启用方式:把「globalias」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

然后只需按下 `SPACE`,即可触发对你已写命令的展开。

如果你只想插入一个空格而不展开命令行,请按 `CTRL`+`SPACE`。

如果你想把某些值排除在展开之外,请把 `GLOBALIAS_FILTER_VALUES` 设置为由这些值组成的数组。
参见[过滤值](#filtered-values)。

## 示例

#### Glob 表达式

```
$ touch {1..10}<space>
# expands to
$ touch 1 2 3 4 5 6 7 8 9 10

$ ls **/*.json<space>
# expands to
$ ls folder/file.json anotherfolder/another.json
```

#### 子命令

```
$ mkdir "`date -R`"
# expands to
$ mkdir Tue,\ 04\ Oct\ 2016\ 13:54:03\ +0300
```

#### 别名

```
# .zshrc:
alias -g G="| grep --color=auto -P"
alias l='ls --color=auto -lah'

$ l<space>G<space>
# expands to
$ ls --color=auto -lah | grep --color=auto -P
```

```
# .zsrc:
alias S="sudo systemctl"

$ S<space>
# expands to:
$ sudo systemctl
```

#### 过滤值

```
# .zshrc
alias l='ls -lh'
alias la='ls --color=auto -lah'
GLOBALIAS_FILTER_VALUES=(l)

$ l<space>
# does not expand
$ la<space>
# expands to:
$ ls --color=auto -lah
```
