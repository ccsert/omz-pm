# git-hubflow 插件

本插件为 [HubFlow](https://datasift.github.io/gitflow/)(即面向 GitHub 的 GitFlow)提供补全,
以及一些常用命令的别名。HubFlow 是一个 git 扩展,让在 GitHub 上使用 GitFlow 变得简单。
它基于最初的 gitflow git 扩展。

hubflow 工具需要单独[安装](https://github.com/datasift/gitflow#installation)。

✅ 启用方式:把「git-hubflow」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名  | 命令             | 说明                                                             |
| ----- | ---------------- | ---------------------------------------------------------------- |
| ghf   | `git hf`         | 打印命令总览                                                     |
| ghff  | `git hf feature` | 管理你的功能分支(feature)                                       |
| ghfr  | `git hf release` | 管理你的发布分支(release)                                       |
| ghfh  | `git hf hotfix`  | 管理你的热修复分支(hotfix)                                      |
| ghfs  | `git hf support` | 管理你的支持分支(support)                                       |
| ghfu  | `git hf update`  | 把上游变更拉取到你的 master 和 develop 分支                      |
