# zeus 插件

[Zeus](https://github.com/burke/zeus) 会预加载你的 Rails 环境,并在需要时 fork 该进程。
这实际上能把 Rails 的启动过程加速到 1 秒以内。本插件为 zeus 提供自动补全以及常用用法的别名。

✅ 启用方式:把「zeus」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

另外,你还需要安装 `zeus` gem。

| 别名         | 命令                                                               |
|:-------------|:-------------------------------------------------------------------|
| _zi_         | `zeus init`                                                        |
| _zinit_      | `zeus init`                                                        |
| _zs_         | `zeus start`                                                       |
| _ztart_      | `zeus start`                                                       |
| _zc_         | `zeus console`                                                     |
| _zonsole_    | `zeus console`                                                     |
| _zsr_        | `zeus server`                                                      |
| _zerver_     | `zeus server`                                                      |
| _zr_         | `noglob zeus rake`                                                 |
| _zake_       | `noglob zeus rake`                                                 |
| _zg_         | `zeus generate`                                                    |
| _zenerate_   | `zeus generate`                                                    |
| _zrn_        | `zeus runner`                                                      |
| _zunner_     | `zeus runner`                                                      |
| _zcu_        | `zeus cucumber`                                                    |
| _zucumber_   | `zeus cucumber`                                                    |
| _zwip_       | `zeus cucumber --profile wip`                                      |
| _zspec_      | `zeus rspec`                                                       |
| _zt_         | `zeus test`                                                        |
| _zest_       | `zeus test`                                                        |
| _zu_         | `zeus test test/unit/*`                                            |
| _zunits_     | `zeus test test/unit/*`                                            |
| _zf_         | `zeus test test/functional/*`                                      |
| _zunctional_ | `zeus test test/functional/*`                                      |
| _za_         | `zeus test test/unit/*; zeus test test/functional/; zeus cucumber` |
| _zall_       | `zeus test test/unit/*; zeus test test/functional/; zeus cucumber` |
| _zsw_        | `rm .zeus.sock`                                                    |
| _zweep_      | `rm .zeus.sock`                                                    |
| _zdbr_       | `zeus rake db:reset db:test:prepare`                               |
| _zdbreset_   | `zeus rake db:reset db:test:prepare`                               |
| _zdbm_       | `zeus rake db:migrate db:test:prepare`                             |
| _zdbmigrate_ | `zeus rake db:migrate db:test:prepare`                             |
| _zdbc_       | `zeus rake db:create`                                              |
| _zdbcm_      | `zeus rake db:create db:migrate db:test:prepare`                   |
