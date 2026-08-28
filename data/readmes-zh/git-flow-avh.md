# git-flow (AVH Edition) 插件

本插件为 [git-flow (AVH Edition)](https://github.com/petervanderdoes/gitflow-avh) 提供补全。
该 AVH 版本的 git 扩展为 [Vincent Driessen 的分支模型](https://nvie.com/posts/a-successful-git-branching-model/)
提供高层次的仓库操作。

✅ 启用方式:把「git-flow-avh」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 依赖要求

1. git-flow 工具需要[单独安装](https://github.com/petervanderdoes/gitflow-avh#installing-git-flow)。

2. 你必须使用 zsh 的 git 补全,而不是 git 项目自带的 git 补全。这通常就是默认情况,
   所以你无需额外操作。但如果你是用 Homebrew 安装的 git,可能需要卸载它捆绑的 git 补全。
