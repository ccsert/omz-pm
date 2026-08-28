# `tig` 插件

本插件为经常使用 [`tig`](https://jonas.github.io/tig/)(Git 的文本模式界面)的用户添加了一些别名。

✅ 启用方式:把「tig」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 特性

| 别名  | 命令           | 说明                                            |
|-------|----------------|-------------------------------------------------|
| `tis` | `tig status`   | 显示 git 状态                                   |
| `til` | `tig log`      | 显示 git 日志                                   |
| `tib` | `tig blame -C` | 对文件执行 `git-blame`,并检测复制和重命名       |
