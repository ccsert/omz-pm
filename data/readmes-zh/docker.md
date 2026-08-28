# Docker 插件

本插件为 [docker](https://www.docker.com/) 提供自动补全和一组别名。

✅ 启用方式:把「docker」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

补全脚本复制自 docker/cli 官方仓库:
https://github.com/docker/cli/blob/master/contrib/completion/zsh/_docker

## 设置

默认情况下,补全不支持「选项连写」(option-stacking):如果你尝试补全
`docker run -it <TAB>`,是不会生效的,因为你把 `-i` 和 `-t` 两个选项_连_在了一起。

[你可以启用它](https://github.com/docker/cli/commit/b10fb43048)——**把下面几行加进你的 zshrc**,
但要注意副作用:

> 启用后 Zsh 能理解 `docker run -it ubuntu` 这类命令。但与此同时,Zsh 也会把
> `docker run -u<tab>` 补全成 `docker run -uapprox` 这样的无效结果。用户必须自己先敲好
> 空格或等号,再尝试补全。
>
> 因此该行为默认关闭。要启用的话:

```sh
zstyle ':completion:*:*:docker:*' option-stacking yes
zstyle ':completion:*:*:docker-*:*' option-stacking yes
```

### 使用旧版补全

如果当前补全在你那里表现不佳,可以用下面的设置改用旧版(legacy)补全。
更多信息见 https://github.com/ohmyzsh/ohmyzsh/issues/11789 。

```zsh
zstyle ':omz:plugins:docker' legacy-completion yes
```

### Podman 的 Docker 包装(wrapper)用户

如果你用的是 Podman 的 Docker wrapper,必须启用旧版补全,见上一节。

## 别名

| 别名    | 命令                          | 说明                                                                                     |
| :------ | :---------------------------- | :--------------------------------------------------------------------------------------- |
| dbl     | `docker build`                | 用 Dockerfile 构建镜像                                                                    |
| dcin    | `docker container inspect`    | 显示一个或多个容器的详细信息                                                              |
| dcls    | `docker container ls`         | 列出所有运行中的 docker 容器                                                              |
| dclsa   | `docker container ls -a`      | 列出所有运行中和已停止的容器                                                              |
| dcprune | `docker container prune`      | 清除所有已停止的容器                                                                      |
| dib     | `docker image build`          | 用 Dockerfile 构建镜像(同 docker build)                                                  |
| dii     | `docker image inspect`        | 显示一个或多个镜像的详细信息                                                              |
| dils    | `docker image ls`            | 列出 docker 镜像                                                                          |
| dipu    | `docker image push`           | 把镜像或仓库推送到远程 registry                                                           |
| dipru   | `docker image prune -a`       | 删除没有被任何容器引用的镜像                                                              |
| dirm    | `docker image rm`             | 删除一个或多个镜像                                                                        |
| dit     | `docker image tag`            | 给指定镜像添加名称和标签                                                                  |
| dlo     | `docker container logs`       | 抓取 docker 容器的日志                                                                    |
| dnc     | `docker network create`       | 创建新网络                                                                                |
| dncn    | `docker network connect`      | 把容器接入某个网络                                                                        |
| dndcn   | `docker network disconnect`   | 把容器从某个网络断开                                                                      |
| dni     | `docker network inspect`      | 返回一个或多个网络的信息                                                                  |
| dnls    | `docker network ls`           | 列出引擎守护进程已知的全部网络,包括跨主机的网络                                          |
| dnprune | `docker network prune`        | 清除所有未使用的网络                                                                      |
| dnrm    | `docker network rm`           | 删除一个或多个网络                                                                        |
| dpo     | `docker container port`       | 列出容器的端口映射,或查看某个指定映射                                                    |
| dps     | `docker ps`                   | 列出所有运行中的 docker 容器                                                              |
| dpsa    | `docker ps -a`                | 列出所有运行中和已停止的容器                                                              |
| dpu     | `docker pull`                 | 从 registry 拉取镜像或仓库                                                                |
| dr      | `docker container run`        | 创建新容器并用指定命令启动它                                                              |
| drit    | `docker container run -it`    | 创建新容器并在交互式 shell 中启动                                                         |
| drm     | `docker container rm`         | 删除指定容器                                                                              |
| drm!    | `docker container rm -f`      | 强制删除运行中的容器(发送 SIGKILL)                                                       |
| dsprune | `docker system prune`         | 清除未使用的数据                                                                          |
| dst     | `docker container start`      | 启动一个或多个已停止的容器                                                                |
| drs     | `docker container restart`    | 重启一个或多个容器                                                                        |
| dsta    | `docker stop $(docker ps -q)` | 停止所有运行中的容器                                                                      |
| dstp    | `docker container stop`       | 停止一个或多个运行中的容器                                                                |
| dsts    | `docker stats`                | 实时显示容器的资源统计                                                                    |
| dtop    | `docker top`                  | 显示容器内正在运行的进程                                                                  |
| dvi     | `docker volume inspect`       | 显示一个或多个数据卷(volume)的详细信息                                                   |
| dvls    | `docker volume ls`           | 列出 docker 已知的所有数据卷                                                              |
| dvprune | `docker volume prune`         | 清理悬空(dangling)数据卷                                                                 |
| dxc     | `docker container exec`       | 在运行中的容器里执行新命令                                                                |
| dxcit   | `docker container exec -it`   | 在运行中的容器里以交互式 shell 执行新命令                                                 |
