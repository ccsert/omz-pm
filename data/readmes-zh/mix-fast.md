# mix-fast

快速的 mix 自动补全插件。

本脚本会把输出缓存起来供后续使用,从而显著提速。
它会在当前项目下生成一个 .mix_tasks 缓存文件。目前如果你想更新缓存,
应当删除 .mix_tasks 文件。

灵感来自 rake-fast zsh 插件,并以其为基础实现。

它完全基于 [Ullrich Schäfer 的这个 pull request](https://github.com/robb/.dotfiles/pull/10/),
而后者又受 [2006 年的这个 Ruby on Rails 小技巧](https://weblog.rubyonrails.org/2006/3/9/fast-rake-task-completion-for-zsh/)启发。


## 安装

✅ 启用方式:把「mix-fast」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

你可以考虑把 `.mix_tasks` 加入你的[全局 .gitignore](https://help.github.com/articles/ignoring-files#global-gitignore)

## 用法

输入 `mix`,然后按 tab 键

当前由 [styx](https://github.com/styx/) 维护
