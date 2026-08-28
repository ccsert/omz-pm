# Symfony2

本插件为 [Symfony 2](https://symfony.com/) 提供补全,并为常用的 Symfony 命令提供别名。

✅ 启用方式:把「symfony2」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名          | 命令                         | 说明                          |
|---------------|------------------------------|-------------------------------|
| `sf`          | php app/console              | 启动 symfony 控制台           |
| `sfcl`        | sf cache:clear               | 清除缓存                      |
| `sfsr`        | sf server:run                | 运行开发服务器                |
| `sfcw`        | sf cache:warmup              | 使用 Bundle 的 warmer         |
| `sfroute`     | sf debug:router              | 显示各个路由                  |
| `sfcontainer` | sf debug:container           | 列出各个服务                  |
| `sfgb`        | sf generate:bundle           | 生成一个 bundle               |
| `sfgc`        | sf generate:controller       | 生成一个 controller           |
| `sfgcom`      | sf generate:command          | 生成一个 command              |
| `sfge`        | sf doctrine:generate:entity  | 生成一个 entity               |
| `sfsu`        | sf doctrine:schema:update    | 更新数据库中的 schema         |
| `sfdc`        | sf doctrine:database:create  | 创建数据库                    |
| `sfdev`       | sf --env=dev                 | 把环境切换为 `dev`            |
| `sfprod`      | sf --env=prod                | 把环境切换为 `prod`           |
