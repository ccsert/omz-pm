# MySQL-Macports 插件

本插件为使用 [MacPorts](https://www.macports.org/) 安装在 macOS 上的 [MySQL](https://www.mysql.com/) 的常用命令添加了一些别名。

✅ 启用方式:把「mysql-macports」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

关于如何使用 MacPorts 安装 MySQL,请阅读 [MacPorts wiki](https://trac.macports.org/wiki/howto/MySQL/)。

## 别名

| 别名         | 命令                                 | 说明                          |
| ------------ | ------------------------------------ | ----------------------------- |
| mysqlstart   | `sudo /path/to/mysql.server start`   | 启动 MySQL 服务器。           |
| mysqlstop    | `sudo /path/to/mysql.server stop`    | 停止 MySQL 服务器。           |
| mysqlrestart | `sudo /path/to/mysql.server restart` | 重启 MySQL 服务器。           |
| mysqlstatus  | `mysqladmin5 -u root -p ping`        | 检查 MySQL 服务器是否在运行。 |
