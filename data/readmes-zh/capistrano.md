# Capistrano

本插件为 [Capistrano](https://capistranorb.com/) 提供自动补全。

✅ 启用方式:把「capistrano」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

要让补全正常工作,请使用 `capit` 命令而不是 `cap`,因为 cap 是
[zsh 的保留字](http://zsh.sourceforge.net/Doc/Release/Zsh-Modules.html#The-zsh_002fcap-Module)。

如果找到了 Gemfile,`capit` 会自动通过 bundler 运行 cap。
