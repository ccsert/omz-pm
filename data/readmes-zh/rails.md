# Rails 插件

本插件为 [Ruby On Rails Framework](https://rubyonrails.org/) 和 [Rake](https://ruby.github.io/rake/) 命令提供补全,并附带一些日志和环境变量相关的别名。

✅ 启用方式:把「rails」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名列表

### Rails 别名

| 别名    | 命令                             | 说明                                                   |
| ------- | -------------------------------- | ------------------------------------------------------ |
| `rc`    | `rails console`                  | 在命令行中与 Rails 应用交互                            |
| `rcs`   | `rails console --sandbox`        | 在沙盒中测试代码,不会改动任何数据                     |
| `rd`    | `rails destroy`                  | 撤销一次 generate 操作                                 |
| `rdb`   | `rails dbconsole`                | 在控制台中操作数据库                                   |
| `rdc`   | `rails db:create`                | 创建数据库                                             |
| `rdd`   | `rails db:drop`                  | 删除数据库                                             |
| `rdm`   | `rails db:migrate`               | 执行待处理的数据库迁移                                 |
| `rdmd`  | `rails db:migrate:down`          | 回滚指定的数据库迁移                                   |
| `rdmr`  | `rails db:migrate:redo`          | 重做指定的数据库迁移                                   |
| `rdmrs` | `rails db:migrate:reset`         | 删除数据库并从零重新建立。                             |
| `rdms`  | `rails db:migrate:status`        | 显示当前数据库迁移状态                                 |
| `rdmtc` | `rails db:migrate db:test:clone` | 执行待处理的迁移,并把数据库克隆到测试数据库           |
| `rdmu`  | `rails db:migrate:up`            | 执行指定的数据库迁移                                   |
| `rdr`   | `rails db:rollback`              | 回滚最近一次迁移                                       |
| `rdrs`  | `rails db:reset`                 | 删除数据库并根据 schema 重新建立。                     |
| `rds`   | `rails db:seed`                  | 填充数据库种子数据                                     |
| `rdsl`  | `rails db:schema:load`           | 加载数据库 schema                                      |
| `rdtc`  | `rails db:test:clone`            | 把数据库克隆到测试数据库                               |
| `rdtp`  | `rails db:test:prepare`          | 把数据库 schema 复制到测试数据库                       |
| `rgen`  | `rails generate`                 | 生成样板代码                                           |
| `rgm`   | `rails generate migration`       | 生成一次数据库迁移                                     |
| `rlc`   | `rails log:clear`                | 清空 Rails 日志                                        |
| `rmd`   | `rails middleware`               | 查看 Rails 中间件                                      |
| `rn`    | `rails notes`                    | 在代码注释中搜索笔记(`FIXME`、`TODO`)                |
| `rp`    | `rails plugin`                   | 执行 Rails 插件命令                                    |
| `rr`    | `rails routes`                   | 列出所有已定义的路由                                   |
| `rrc`   | `rails routes --controller`      | 列出并过滤映射到指定控制器的路由                       |
| `rre`   | `rails routes --expanded`        | 以展开表格模式列出所有已定义的路由                     |
| `rrg`   | `rails routes --grep`            | 列出并过滤已定义的路由                                 |
| `rru`   | `rails routes --unused`          | 列出未使用的路由                                       |
| `rs`    | `rails server`                   | 启动一个 web 服务器                                    |
| `rsb`   | `rails server --bind`            | 启动 web 服务器并绑定到指定 IP                         |
| `rsd`   | `rails server --debugger`        | 启动带调试器的 web 服务器                              |
| `rsp`   | `rails server --port`            | 启动 web 服务器并指定监听端口                          |
| `rsts`  | `rails stats`                    | 输出代码统计信息                                       |
| `rt`    | `rails test`                     | 运行 Rails 测试                                        |
| `rta`   | `rails test:all`                 | 运行所有 Rails 测试,包括系统测试                      |
| `ru`    | `rails runner`                   | 在 Rails 环境中运行 Ruby 代码                          |

### Foreman

| 别名   | 命令            | 说明                                      |
| ------ | --------------- | ----------------------------------------- |
| `fmns` | `foreman start` | 在命令行中与 Rails 应用交互               |

### 实用别名

| 别名      | 命令                          | 说明                                           |
| --------- | ----------------------------- | ---------------------------------------------- |
| `devlog`  | `tail -f log/development.log` | 显示开发日志并持续跟踪其变化                   |
| `prodlog` | `tail -f log/production.log`  | 显示生产日志并持续跟踪其变化                   |
| `testlog` | `tail -f log/test.log`        | 显示测试日志并持续跟踪其变化                   |

### 环境变量设置

| 别名  | 命令                    | 说明                            |
| ----- | ----------------------- | ------------------------------- |
| `RED` | `RAILS_ENV=development` | 把 `RAILS_ENV` 设为 development |
| `REP` | `RAILS_ENV=production`  | 把 `RAILS_ENV` 设为 production  |
| `RET` | `RAILS_ENV=test`        | 把 `RAILS_ENV` 设为 test        |

这些是全局别名。既可以与命令组合使用,也可以单独执行。例如:`REP rake db:migrate` 会对生产数据库执行迁移。

## 旧版

### Rake 别名

下列命令自 Rails v5 起[已改用 `rails` 而非 `rake` 来执行][1],但为了向后兼容,仍以 `rk` 前缀保留至今。

[1]: https://guides.rubyonrails.org/v5.2/command_line.html#bin-rails

| 别名     | 命令                            | 说明                                                   |
| -------- | ------------------------------- | ------------------------------------------------------ |
| `rkdc`   | `rake db:create`                | 创建数据库                                             |
| `rkdd`   | `rake db:drop`                  | 删除数据库                                             |
| `rkdm`   | `rake db:migrate`               | 执行待处理的数据库迁移                                 |
| `rkdms`  | `rake db:migrate:status`        | 显示当前数据库迁移状态                                 |
| `rkdmtc` | `rake db:migrate db:test:clone` | 执行待处理的迁移,并把数据库克隆到测试数据库           |
| `rkdr`   | `rake db:rollback`              | 回滚最近一次迁移                                       |
| `rkdrs`  | `rake db:reset`                 | 删除数据库并重新建立                                   |
| `rkds`   | `rake db:seed`                  | 填充数据库种子数据                                     |
| `rkdsl`  | `rake db:schema:load`           | 加载数据库 schema                                      |
| `rkdtc`  | `rake db:test:clone`            | 把数据库克隆到测试数据库                               |
| `rkdtp`  | `rake db:test:prepare`          | 把数据库 schema 复制到测试数据库                       |
| `rklc`   | `rake log:clear`                | 清空 Rails 日志                                        |
| `rkmd`   | `rake middleware`               | 查看 Rails 中间件                                      |
| `rkn`    | `rake notes`                    | 在代码注释中搜索笔记(`FIXME`、`TODO`)                |
| `rksts`  | `rake stats`                    | 输出代码统计信息                                       |
| `rkt`    | `rake test`                     | 运行 Rails 测试                                        |

### 其他

| 别名    | 命令                               |
| ------- | ---------------------------------- |
| `sc`    | `ruby script/console`              |
| `sd`    | `ruby script/destroy`              |
| `sd`    | `ruby script/server --debugger`    |
| `sg`    | `ruby script/generate`             |
| `sp`    | `ruby script/plugin`               |
| `sr`    | `ruby script/runner`               |
| `ssp`   | `ruby script/spec`                 |
| `sstat` | `thin --stats "/thin/stats" start` |

- `remote_console <server> <directory>`:在远程服务器上运行 `ruby script/console production`。
