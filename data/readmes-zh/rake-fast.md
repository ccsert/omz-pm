# rake-fast

快速的 rake 自动补全插件。

本插件会把输出缓存起来供后续使用,从而显著提升速度。
它会在 Rakefile 旁边生成一个 `.rake_tasks` 缓存文件。它还会
检查文件的修改时间,以判断是否需要重新生成缓存文件。

这完全基于 [Ullrich Schäfer 的这个 pull request](https://github.com/robb/.dotfiles/pull/10/),
其灵感来自 [2006 年的这个 Ruby on Rails 技巧](https://weblog.rubyonrails.org/2006/3/9/fast-rake-task-completion-for-zsh/)。

想想看,那可是 2006 年。

----------

自 2016 年 8 月起,它还会检测自己是否处于 Rails 项目中,并查看
`lib/tasks` 里的 rake 文件及其修改时间,以判断是否需要重新生成缓存文件。

## 安装

✅ 启用方式:把「rake-fast」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

你可以考虑把 `.rake_tasks` 加入你的[全局 .gitignore](https://help.github.com/articles/ignoring-files#global-gitignore)。

## 用法

输入 `rake`,然后按 tab 键。

如果你想强制重新生成 `.rake_tasks` 文件,请运行 `rake_refresh`。
