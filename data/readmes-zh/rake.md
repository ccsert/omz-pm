# Rake 插件

本插件为 [rake](https://ruby.github.io/rake/) 提供支持,rake 是 Ruby 的构建工具,即 Ruby 版的 Make。

✅ 启用方式:把「rake」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

本插件对 rake 命令做了别名处理,让你在调用 rake 任务时可以传入参数而不必转义方括号,也就是说,你可以直接运行

```sh
rake namespace:task['argument']
```

而不必写成

```sh
rake namespace:task\['argument'\]
```

| 别名       | 命令                           | 说明                                          |
| ---------- | ------------------------------ | --------------------------------------------- |
| `rake`     | `noglob rake`                  | 允许使用未转义的方括号                        |
| `bin/rake` | `noglob bin/rake`              | 同上,但使用 rake binstub                     |
| `brake`    | `noglob bundle exec rake`      | 同上,但通过 bundler 调用 rake                |
| `srake`    | `noglob sudo rake`             | 与 rake 相同,但使用 sudo                     |
| `sbrake`   | `noglob sudo bundle exec rake` | 同上,但同时使用 sudo 和 bundler              |

## Jim Weirich

本插件还将 `rake` 别名指向了 [`jimweirich`](https://github.com/jimweirich),他是 Rake 的作者,
也是 Ruby 开源社区的重要贡献者。他于 2014 年去世:

> 谢谢你,Jim,谢谢你这些年来为 Ruby 和开源社区贡献的一切。我们会深深地怀念你。 — [**@robbyrussell**](https://github.com/ohmyzsh/ohmyzsh/commit/598a9c6f990756386517d66b6bcf77e53791e905)
