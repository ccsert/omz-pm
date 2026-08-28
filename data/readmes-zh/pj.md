# pj

`pj` 插件(`Project Jump` 的缩写)允许你定义多个存放项目的文件夹,
这样只需使用项目目录的名称,就能直接跳转过去。

最初的创意和代码来自 Jan De Poorter([@DefV](https://github.com/DefV))
来源:https://gist.github.com/pjaspers/368394#gistcomment-1016

## 用法

1. ✅ 启用方式:把「pj」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

2. 在你的 ~/.zshrc 中设置 `$PROJECT_PATHS`:

   ```zsh
   PROJECT_PATHS=(~/src ~/work ~/"dir with spaces")
   ```

现在你可以使用以下命令之一:

##### `pj my-project`:

`cd` 到在某个 `$PROJECT_PATHS` 目录中找到的名为 "my-project" 的目录。
如果存在多个同名目录,则以 `$PROJECT_PATHS` 中最先出现的那个为准。

例如:

```zsh
PROJECT_PATHS=(~/code ~/work)
$ ls ~/code    # ~/code/blog ~/code/react
$ ls ~/work    # ~/work/blog ~/work/project
$ pj blog      # <-- will cd to ~/code/blog
```

##### `pjo my-project`

用你定义的 `$EDITOR` 打开项目目录。它遵循与上面的 `pj` 命令相同的目录规则。

注意:`pjo` 是 `pj open` 的别名。
