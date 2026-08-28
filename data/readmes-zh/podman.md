# Podman 插件

本插件为 [podman](https://podman.io/) 提供自动补全和一组别名。

✅ 启用方式:把「podman」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名    | 命令                                          | 说明                                                                                     |
| :------ | :-------------------------------------------- | :--------------------------------------------------------------------------------------- |
| pbl     | `podman build`                                | 用 Dockerfile 构建镜像                                                                   |
| pcin    | `podman container inspect`                    | 显示一个或多个容器的详细信息                                                             |
| pcls    | `podman container ls`                         | 列出所有运行中的 podman 容器                                                             |
| pclsa   | `podman container ls --all`                   | 列出所有运行中和已停止的容器                                                             |
| pib     | `podman image build`                          | 用 Dockerfile 构建镜像(同 podman build)                                                 |
| pii     | `podman image inspect`                        | 显示一个或多个镜像的详细信息                                                             |
| pils    | `podman image ls`                             | 列出 podman 镜像                                                                         |
| pipu    | `podman image push`                           | 把镜像或仓库推送到远程 registry                                                          |
| pirm    | `podman image rm`                             | 删除一个或多个镜像                                                                       |
| pit     | `podman image tag`                            | 给指定镜像添加名称和标签                                                                 |
| plo     | `podman container logs`                       | 抓取 podman 容器的日志                                                                   |
| pnc     | `podman network create`                       | 创建新网络                                                                               |
| pncn    | `podman network connect`                      | 把容器接入某个网络                                                                       |
| pndcn   | `podman network disconnect`                   | 把容器从某个网络断开                                                                     |
| pni     | `podman network inspect`                      | 返回一个或多个网络的信息                                                                 |
| pnls    | `podman network ls`                           | 列出引擎守护进程已知的全部网络,包括跨主机的网络                                         |
| pnrm    | `podman network rm`                           | 删除一个或多个网络                                                                       |
| ppo     | `podman container port`                       | 列出容器的端口映射,或查看某个指定映射                                                   |
| ppu     | `podman pull`                                 | 从 registry 拉取镜像或仓库                                                               |
| pr      | `podman container run`                        | 创建新容器并用指定命令启动它                                                             |
| prit    | `podman container run --interactive --tty`    | 创建新容器并在交互式 shell 中启动                                                        |
| prm     | `podman container rm`                         | 删除指定容器                                                                             |
| prm!    | `podman container rm --force`                 | 强制删除运行中的容器(发送 SIGKILL)                                                      |
| pst     | `podman container start`                      | 启动一个或多个已停止的容器                                                               |
| prs     | `podman container restart`                    | 重启一个或多个容器                                                                       |
| psta    | `podman stop $(podman ps -q)`                 | 停止所有运行中的容器                                                                     |
| pstp    | `podman container stop`                       | 停止一个或多个运行中的容器                                                               |
| ptop    | `podman top`                                  | 显示容器内正在运行的进程                                                                 |
| pvi     | `podman volume inspect`                       | 显示一个或多个数据卷(volume)的详细信息                                                  |
| pvls    | `podman volume ls`                            | 列出 podman 已知的所有数据卷                                                             |
| pvprune | `podman volume prune`                         | 清理悬空(dangling)数据卷                                                                |
| pxc     | `podman container exec`                       | 在运行中的容器里执行新命令                                                               |
| pxcit   | `podman container exec --interactive --tty`   | 在运行中的容器里以交互式 shell 执行新命令                                                |
