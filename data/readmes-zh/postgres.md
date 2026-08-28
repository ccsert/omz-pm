# Postgres 插件

本插件为一些实用的 Postgres 命令添加了别名。

:warning: 本插件只能配合在 OSX 上通过 Homebrew 安装的 Postgres 使用,
因为 Postgres 的路径被硬编码为 `/usr/local/var/postgres`。

✅ 启用方式:把「postgres」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名        | 命令                                                                            | 说明                                                         |
|-------------|---------------------------------------------------------------------------------|--------------------------------------------------------------|
| startpost   | `pg_ctl -D /usr/local/var/postgres -l /usr/local/var/postgres/server.log start` | 启动 postgres 服务器                                         |
| stoppost    | `pg_ctl -D /usr/local/var/postgres stop -s -m fast`                             | 停止 postgres 服务器                                         |
| restartpost | `stoppost && sleep 1 && startpost`                                              | 重启(先调用 stop,再调用 start)                             |
| reloadpost  | `pg_ctl reload -D /usr/local/var/postgres -s`                                   | 重新加载 postgres 配置(某些设置需要重启才能生效)            |
| statuspost  | `pg_ctl status -D /usr/local/var/postgres -s`                                   | 查看 postgres 服务器的状态(运行中、已停止)                  |
