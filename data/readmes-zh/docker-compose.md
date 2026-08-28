# Docker-compose 插件

本插件为 [docker-compose](https://docs.docker.com/compose/) 提供自动补全,以及一些常用
docker-compose 命令的别名。
插件会在旧版 `docker-compose` 命令和现代 `docker compose` 子命令之间自动选择,
当两者都可用时优先使用 `docker-compose`。

✅ 启用方式:把「docker-compose」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名      | 命令                             | 说明                                                                     |
|-----------|----------------------------------|--------------------------------------------------------------------------|
| dco       | `docker-compose`                 | Docker-compose 主命令                                                    |
| dcb       | `docker-compose build`           | 构建容器                                                                 |
| dce       | `docker-compose exec`            | 在容器内执行命令                                                         |
| dcps      | `docker-compose ps`              | 列出容器                                                                 |
| dcrestart | `docker-compose restart`         | 重启容器                                                                 |
| dcrm      | `docker-compose rm`              | 移除容器                                                                 |
| dcr       | `docker-compose run`             | 在容器中运行命令                                                         |
| dcstop    | `docker-compose stop`            | 停止容器                                                                 |
| dcup      | `docker-compose up`              | 为服务构建、(重新)创建、启动并连接容器                                   |
| dcupb     | `docker-compose up --build`      | 同 `dcup`,但会在启动容器前先构建镜像                                     |
| dcupd     | `docker-compose up -d`           | 同 `dcup`,但以守护进程方式启动                                           |
| dcupdb    | `docker-compose up -d --build`   | 同 `dcup`,但会在启动容器前先构建镜像,并以守护进程方式启动               |
| dcdn      | `docker-compose down`            | 停止并移除容器                                                           |
| dcl       | `docker-compose logs`            | 显示容器日志                                                             |
| dclf      | `docker-compose logs -f`         | 显示日志并持续跟踪输出                                                   |
| dclF      | `docker-compose logs -f --tail0` | 仅跟踪最近的日志                                                         |
| dcpull    | `docker-compose pull`            | 拉取服务的镜像                                                           |
| dcstart   | `docker-compose start`           | 启动容器                                                                 |
| dck       | `docker-compose kill`            | 强制终止容器                                                             |
