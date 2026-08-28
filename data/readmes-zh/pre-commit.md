# Pre-commit 插件

本插件为 [pre-commit](https://pre-commit.com/) 的常用命令添加别名。
它还支持用 [prek](https://github.com/prek/prek) 作为直接替代品(drop-in replacement):
如果 `prek` 可用,就会使用它;否则回退使用 `pre-commit`。

✅ 启用方式:把「pre-commit」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名  | 命令                                                   | 说明                                          |
| ----- | ------------------------------------------------------ | --------------------------------------------- |
| prc   | `prek` or `pre-commit`                                 | pre-commit 命令                               |
| prcau | `prek auto-update` or `pre-commit autoupdate`          | 自动更新钩子                                  |
| prcr  | `prek run` or `pre-commit run`                         | pre-commit run 命令                           |
| prcra | `prek run --all-files` or `pre-commit run --all-files` | 对所有文件运行 pre-commit 钩子                |
| prcrf | `prek run --files` or `pre-commit run --files`         | 对给定的文件列表运行 pre-commit 钩子          |
