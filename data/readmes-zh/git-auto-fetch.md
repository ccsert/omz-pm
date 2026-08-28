# Git auto-fetch

当你在一个 git 仓库目录中工作时,自动从所有远程抓取全部变更。

✅ 启用方式:把「git-auto-fetch」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

每次显示命令提示符时,都会在后台从所有远程抓取。默认情况下,只有当距离上次自动抓取至少过去了 60 秒,`git-auto-fetch` 才会触发。
你可以在 .zshrc 中修改抓取间隔:

```sh
GIT_AUTO_FETCH_INTERVAL=1200 # in seconds
```

`git-fetch-all` 的日志会保存在 `.git/FETCH_LOG` 中。

## 按文件夹切换自动抓取

如果你正在使用移动网络,或出于其他任何原因,你可以针对任意文件夹禁用 git-auto-fetch:

```shell
$ cd to/your/project
$ git-auto-fetch
disabled
$ git-auto-fetch
enabled
```

## 注意事项

自动抓取所有变更会让 `git push --force-with-lease` 失去原本的意义,并在某些情况下使它表现得像 `git push --force`。例如:

假设你做了一些修改,还可能 rebase 了一些内容,这意味着你需要使用 `--force-with-lease` 来覆盖某个分支的远程历史。从你做出修改(也许还运行了 `git log`)到你执行 `git push` 之间,别人可能已经更新了你正在工作的分支。

如果此时 `git-auto-fetch` 触发了,你就会在不知情的情况下抓取到远程变更;即便你执行 push 时带着 `--force-with-lease`,git 也会覆盖最近的这些变更,因为它们已经存在于你的本地仓库中。
[`git push --force-with-lease` 文档](https://git-scm.com/docs/git-push)讨论了解决这一问题的几种可行方案。
