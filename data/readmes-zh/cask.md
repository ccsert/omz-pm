# Cask 插件

[Cask](https://github.com/cask/cask) 是一款 Emacs 项目管理工具,可以帮助自动化包开发的整个周期:开发、依赖、测试、构建、打包等。

本插件会从非标准位置加载 `cask` 补全,例如通过 Homebrew 或其他方式安装的情况。

✅ 启用方式:把「cask」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

请确保在加载 Oh My Zsh 之前,`cask` 所在目录已在你的 `$PATH` 中,否则会出现
"command not found" 错误。
