# Ruby Version Manager 插件

本插件为 [Ruby Version Manager](https://rvm.io/) 添加了一些实用函数和补全。

✅ 启用方式:把「rvm」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名         | 命令                 |
| ------------ | -------------------- |
| `rb18`       | `rvm use ruby-1.8.7` |
| `rb19`       | `rvm use ruby-1.9.3` |
| `rb20`       | `rvm use ruby-2.0.0` |
| `rb21`       | `rvm use ruby-2.1`   |
| `rb22`       | `rvm use ruby-2.2`   |
| `rb23`       | `rvm use ruby-2.3`   |
| `rb24`       | `rvm use ruby-2.4`   |
| `rb25`       | `rvm use ruby-2.5`   |
| `rb26`       | `rvm use ruby-2.6`   |
| `rb27`       | `rvm use ruby-2.7`   |
| `rb30`       | `rvm use ruby-3.0`   |
| `rb31`       | `rvm use ruby-3.1`   |
| `rb32`       | `rvm use ruby-3.2`   |
| `rvm-update` | `rvm get head`       |
| `gems`       | `gem list`           |
| `rvms`       | `rvm gemset`         |

## 已弃用的版本

在撰写本文时(2021-12-28),2.5 及以下的 Ruby 版本[均已 EOL][1],
未来将被移除。

[1]: https://endoflife.date/ruby
