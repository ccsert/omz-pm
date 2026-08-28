# hcloud 插件

本插件为 [Hetzner Cloud CLI](https://github.com/hetznercloud/cli) 提供自动补全,
并附带一些常用 hcloud 命令的别名。

✅ 启用方式:把「hcloud」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名       | 命令                                      | 说明                                       |
| :--------- | :---------------------------------------- | :----------------------------------------- |
| hc         | `hcloud`                                  | hcloud 命令                                 |
|            |                                           | **上下文管理**                              |
| hcctx      | `hcloud context`                          | 管理上下文                                  |
| hcctxls    | `hcloud context list`                     | 列出所有上下文                              |
| hcctxu     | `hcloud context use`                      | 使用某个上下文                              |
| hcctxc     | `hcloud context create`                   | 创建新上下文                                |
| hcctxd     | `hcloud context delete`                   | 删除一个上下文                              |
| hcctxa     | `hcloud context active`                   | 显示当前活动的上下文                        |
|            |                                           | **服务器管理**                              |
| hcs        | `hcloud server`                           | 管理服务器                                  |
| hcsl       | `hcloud server list`                      | 列出所有服务器                              |
| hcsc       | `hcloud server create`                    | 创建服务器                                  |
| hcsd       | `hcloud server delete`                    | 删除服务器                                  |
| hcsdesc    | `hcloud server describe`                  | 查看服务器详情                              |
| hcspoff    | `hcloud server poweroff`                  | 关闭服务器电源                              |
| hcspon     | `hcloud server poweron`                   | 开启服务器电源                              |
| hcsr       | `hcloud server reboot`                    | 重启服务器                                  |
| hcsreset   | `hcloud server reset`                     | 重置服务器                                  |
| hcssh      | `hcloud server ssh`                       | 通过 SSH 连接到服务器                       |
| hcse       | `hcloud server enable-rescue`             | 为服务器启用救援模式                        |
| hcsdr      | `hcloud server disable-rescue`            | 为服务器禁用救援模式                        |
| hcsip      | `hcloud server ip`                        | 管理服务器 IP                               |
| hcsa       | `hcloud server attach-iso`                | 给服务器挂载 ISO                            |
| hcsda      | `hcloud server detach-iso`                | 从服务器卸载 ISO                            |
| hcscip     | `hcloud server change-type`               | 更改服务器类型                              |
|            |                                           | **卷(Volume)管理**                          |
| hcv        | `hcloud volume`                           | 管理卷                                      |
| hcvl       | `hcloud volume list`                      | 列出所有卷                                  |
| hcvc       | `hcloud volume create`                    | 创建卷                                      |
| hcvd       | `hcloud volume delete`                    | 删除卷                                      |
| hcvdesc    | `hcloud volume describe`                  | 查看卷详情                                  |
| hcva       | `hcloud volume attach`                    | 把卷挂载到服务器                            |
| hcvda      | `hcloud volume detach`                    | 把卷从服务器上卸载                          |
| hcvr       | `hcloud volume resize`                    | 调整卷的容量                                |
|            |                                           | **网络管理**                                |
| hcn        | `hcloud network`                          | 管理网络                                    |
| hcnl       | `hcloud network list`                     | 列出所有网络                                |
| hcnc       | `hcloud network create`                   | 创建网络                                    |
| hcnd       | `hcloud network delete`                   | 删除网络                                    |
| hcndesc    | `hcloud network describe`                 | 查看网络详情                                |
| hcnas      | `hcloud network add-subnet`               | 给网络添加子网                              |
| hcnds      | `hcloud network delete-subnet`            | 从网络中删除子网                            |
| hcnar      | `hcloud network add-route`                | 给网络添加路由                              |
| hcndr      | `hcloud network delete-route`             | 从网络中删除路由                            |
|            |                                           | **浮动 IP(Floating IP)管理**                |
| hcfip      | `hcloud floating-ip`                      | 管理浮动 IP                                 |
| hcfipl     | `hcloud floating-ip list`                 | 列出所有浮动 IP                             |
| hcfipc     | `hcloud floating-ip create`               | 创建浮动 IP                                 |
| hcfipd     | `hcloud floating-ip delete`               | 删除浮动 IP                                 |
| hcfipdesc  | `hcloud floating-ip describe`             | 查看浮动 IP 详情                            |
| hcfipa     | `hcloud floating-ip assign`               | 把浮动 IP 分配给服务器                      |
| hcfipua    | `hcloud floating-ip unassign`             | 把浮动 IP 从服务器上解绑                    |
|            |                                           | **SSH 密钥管理**                            |
| hcsk       | `hcloud ssh-key`                          | 管理 SSH 密钥                               |
| hcskl      | `hcloud ssh-key list`                     | 列出所有 SSH 密钥                           |
| hcskc      | `hcloud ssh-key create`                   | 创建 SSH 密钥                               |
| hcskd      | `hcloud ssh-key delete`                   | 删除 SSH 密钥                               |
| hcskdesc   | `hcloud ssh-key describe`                 | 查看 SSH 密钥详情                           |
| hcsku      | `hcloud ssh-key update`                   | 更新 SSH 密钥                               |
|            |                                           | **镜像管理**                                |
| hci        | `hcloud image`                            | 管理镜像                                    |
| hcil       | `hcloud image list`                       | 列出所有镜像                                |
| hcid       | `hcloud image delete`                     | 删除镜像                                    |
| hcidesc    | `hcloud image describe`                   | 查看镜像详情                                |
| hciu       | `hcloud image update`                     | 更新镜像                                    |
|            |                                           | **防火墙管理**                              |
| hcfw       | `hcloud firewall`                         | 管理防火墙                                  |
| hcfwl      | `hcloud firewall list`                    | 列出所有防火墙                              |
| hcfwc      | `hcloud firewall create`                  | 创建防火墙                                  |
| hcfwd      | `hcloud firewall delete`                  | 删除防火墙                                  |
| hcfwdesc   | `hcloud firewall describe`                | 查看防火墙详情                              |
| hcfwar     | `hcloud firewall add-rule`                | 给防火墙添加规则                            |
| hcfwdr     | `hcloud firewall delete-rule`             | 从防火墙中删除规则                          |
| hcfwas     | `hcloud firewall apply-to-resource`       | 把防火墙应用到某个资源                      |
| hcfwrs     | `hcloud firewall remove-from-resource`    | 把防火墙从某个资源上移除                    |
|            |                                           | **负载均衡器管理**                          |
| hclb       | `hcloud load-balancer`                    | 管理负载均衡器                              |
| hclbl      | `hcloud load-balancer list`               | 列出所有负载均衡器                          |
| hclbc      | `hcloud load-balancer create`             | 创建负载均衡器                              |
| hclbd      | `hcloud load-balancer delete`             | 删除负载均衡器                              |
| hclbdesc   | `hcloud load-balancer describe`           | 查看负载均衡器详情                          |
| hclbu      | `hcloud load-balancer update`             | 更新负载均衡器                              |
| hclbas     | `hcloud load-balancer add-service`        | 给负载均衡器添加服务                        |
| hclbds     | `hcloud load-balancer delete-service`     | 从负载均衡器中删除服务                      |
| hclbat     | `hcloud load-balancer add-target`         | 给负载均衡器添加目标                        |
| hclbdt     | `hcloud load-balancer delete-target`      | 从负载均衡器中删除目标                      |
|            |                                           | **证书管理**                                |
| hccert     | `hcloud certificate`                      | 管理证书                                    |
| hccertl    | `hcloud certificate list`                 | 列出所有证书                                |
| hccertc    | `hcloud certificate create`               | 创建证书                                    |
| hccertd    | `hcloud certificate delete`               | 删除证书                                    |
| hccertdesc | `hcloud certificate describe`             | 查看证书详情                                |
| hccertu    | `hcloud certificate update`               | 更新证书                                    |
|            |                                           | **数据中心与位置信息**                      |
| hcdc       | `hcloud datacenter list`                  | 列出所有数据中心                            |
| hcloc      | `hcloud location list`                    | 列出所有位置                                |
| hcst       | `hcloud server-type list`                 | 列出所有服务器类型                          |
| hcit       | `hcloud image list --type system`         | 列出所有系统镜像                            |

## 依赖要求

本插件需要安装 [Hetzner Cloud CLI](https://github.com/hetznercloud/cli)。

### 安装

用以下任意一种方式安装 Hetzner Cloud CLI:

**macOS(Homebrew):**
```bash
brew install hcloud
```

**Linux(从源码安装):**
```bash
go install github.com/hetznercloud/cli/cmd/hcloud@latest
```

**或者从[发布页](https://github.com/hetznercloud/cli/releases)下载预编译的二进制文件。**

### 配置

安装完成后,创建一个上下文并完成认证:

```bash
hcloud context create my-project
```

接下来会提示你输入 Hetzner Cloud API token,你可以在 [Hetzner Cloud Console](https://console.hetzner.cloud/) 中生成它。
