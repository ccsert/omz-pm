# Git-Flow 插件

本插件为 [`git-flow` 命令](https://github.com/nvie/gitflow)提供补全和别名。

✅ 启用方式:把「git-flow」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名      | 命令                                      | 说明                                           |
| --------- | ----------------------------------------- | ---------------------------------------------- |
| `gcd`     | `git checkout develop`                    | 检出 develop 分支                              |
| `gch`     | `git checkout hotfix`                     | 检出 hotfix 分支                               |
| `gcr`     | `git checkout release`                    | 检出 release 分支                              |
| `gfl`     | `git flow`                                | Git-Flow 命令                                  |
| `gflf`    | `git flow feature`                        | 列出现有的 feature 分支                        |
| `gflff`   | `git flow feature finish`                 | 完成 feature:`gflff <name>`                    |
| `gflffc`  | `gflff ${$(git_current_branch)#feature/}` | 完成当前 feature                               |
| `gflfp`   | `git flow feature publish`                | 发布 feature:`gflfp <name>`                    |
| `gflfpc`  | `gflfp ${$(git_current_branch)#feature/}` | 发布当前 feature                               |
| `gflfpll` | `git flow feature pull`                   | 拉取远程 feature:`gflfpll <remote> <name>`     |
| `gflfs`   | `git flow feature start`                  | 开始一个新 feature:`gflfs <name>`              |
| `gflh`    | `git flow hotfix`                         | 列出现有的 hotfix 分支                         |
| `gflhf`   | `git flow hotfix finish`                  | 完成 hotfix:`gflhf <version>`                  |
| `gflhfc`  | `gflhf ${$(git_current_branch)#hotfix/}`  | 完成当前 hotfix                                |
| `gflhp`   | `git flow hotfix publish`                 | 发布 hostfix:`gflhp <version>`                 |
| `gflhpc`  | `gflhp ${$(git_current_branch)#hotfix/}`  | 完成当前 hotfix                                |
| `gflhs`   | `git flow hotfix start`                   | 开始一个新 hotfix:`gflhs <version>`            |
| `gfli`    | `git flow init`                           | 初始化 git-flow 仓库                           |
| `gflr`    | `git flow release`                        | 列出现有的 release 分支                        |
| `gflrf`   | `git flow release finish`                 | 完成 release:`gflrf <version>`                 |
| `gflrfc`  | `gflrf ${$(git_current_branch)#release/}` | 完成当前 release                               |
| `gflrp`   | `git flow release publish`                | 发布 release:`gflrp <version>`                 |
| `gflrpc`  | `gflrp ${$(git_current_branch)#release/}` | 发布当前 release                               |
| `gflrs`   | `git flow release start`                  | 开始一个新 release:`gflrs <version>`           |

[更多关于 `git-flow` 命令的信息](https://github.com/nvie/gitflow/wiki/Command-Line-Arguments)。
