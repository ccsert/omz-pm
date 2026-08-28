# JRuby 插件

本插件为 [JRuby](https://www.jruby.org/) 提供别名。

✅ 启用方式:把「jruby」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 依赖要求

本插件假定你已经安装了 jruby,并且它在你的 [path](https://www.jruby.org/getting-started) 中可用。

## 别名

| 别名         | 命令                                                             |
| ------------ | ---------------------------------------------------------------- |
| `jrspec`     | `jruby --debug -S rspec --debug`                                 |
| `jprofile`   | `jruby --profile.api -S rspec`                                   |
| `jexec`      | `jruby -S`                                                       |
