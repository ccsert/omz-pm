# repo 插件

本插件主要为 [repo 命令行工具](https://code.google.com/p/git-repo/)添加了一些别名和自动补全支持。

✅ 启用方式:把「repo」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名    | 命令                                   |
|---------|----------------------------------------|
| `r`     | `repo`                                 |
| `rra`   | `repo rebase --auto-stash`             |
| `rs`    | `repo sync`                            |
| `rsrra` | `repo sync ; repo rebase --auto-stash` |
| `ru`    | `repo upload`                          |
| `rst`   | `repo status`                          |
| `rsto`  | `repo status -o`                       |
| `rfa`   | `repo forall -c`                       |
| `rfap`  | `repo forall -p -c`                    |
| `rinf`  | `repo info`                            |
