# meteor 插件

[meteor 插件](https://github.com/ohmyzsh/ohmyzsh/tree/master/plugins/meteor)提供了许多
[实用别名](#aliases),并为 `meteor` 命令提供自动补全。

✅ 启用方式:把「meteor」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名    | 命令                       | 说明                                                             |
|---------|----------------------------|------------------------------------------------------------------|
| `ma`    | `meteor add`               | 向本项目添加一个包                                               |
| `map`   | `meteor add-platform`      | 向本项目添加一个平台                                             |
| `mad`   | `meteor admin`             | 管理类命令                                                       |
| `mau`   | `meteor authorized`        | 查看或修改某个站点授权的用户与组织                               |
| `mb`    | `meteor build`             | 为所有平台构建本项目                                             |
| `mcl`   | `meteor claim`             | 认领用旧版 Meteor 部署的站点                                     |
| `mca`   | `meteor configure-android` | 在 Meteor 的 ADK 环境中运行 Android 配置工具                     |
| `mc`    | `meteor create`            | 创建新项目                                                       |
| `mdb`   | `meteor debug`             | 运行项目,但挂起服务端进程以便调试                               |
| `mde`   | `meteor deploy`            | 将本项目部署到 Meteor                                            |
| `mis`   | `meteor install-sdk`       | 为某个平台安装 SDK                                               |
| `ml`    | `meteor list`              | 列出项目显式使用的包                                             |
| `mlp`   | `meteor list-platforms`    | 列出项目中已添加的平台                                           |
| `mls`   | `meteor list-sites`        | 列出你已获授权的站点                                             |
| `mli`   | `meteor login`             | 登录你的 Meteor 开发者账户                                       |
| `mlo`   | `meteor logout`            | 退出你的 Meteor 开发者账户                                       |
| `mlog`  | `meteor logs`              | 显示指定站点的日志                                               |
| `mm`    | `meteor mongo`             | 连接指定站点的 Mongo 数据库                                      |
| `mp`    | `meteor publish`           | 将包的新版本发布到包服务器                                       |
| `mpa`   | `meteor publish-for-arch`  | 为新平台构建一个已发布的包                                       |
| `mpr`   | `meteor publish-release`   | 向包服务器发布一个新的 meteor 发行版                             |
| `mr`    | `meteor remove`            | 从本项目移除一个包                                               |
| `mrp`   | `meteor remove-platform`   | 从本项目移除一个平台                                             |
| `mre`   | `meteor reset`             | 重置项目状态,会清空本地数据库                                   |
| `m`     | `meteor run`               | **[默认]** 以本地开发模式运行本项目                              |
| `ms`    | `meteor search`            | 在包服务器数据库中搜索                                           |
| `msh`   | `meteor shell`             | 启动一个 Node REPL,交互式地执行服务端代码                       |
| `msw`   | `meteor show`              | 显示某个发行版或包的详细信息                                     |
| `mt`    | `meteor test-packages`     | 测试一个或多个包                                                 |
| `mu`    | `meteor update`            | 将本项目的依赖升级到最新版本                                     |
| `mw`    | `meteor whoami`            | 输出你的 Meteor 开发者账户用户名                                 |
