# MicroK8s 插件

本插件为 [MicroK8s](https://microk8s.io/) 提供补全和实用的别名。

✅ 启用方式:把「microk8s」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名  | 命令             | 说明                                                                                              |
|-------|------------------|----------------------------------------------------------------------------------------------------------|
| mco   | microk8s.config  | 显示 Kubernetes 配置文件。                                                                                |
| mct   | microk8s.ctr     | 与 containerd CLI 交互。                                                                                  |
| mdi   | microk8s.disable | 禁用一个 addon。                                                                                          |
| me    | microk8s.enable  | 启用一个 addon。                                                                                          |
| mh    | microk8s.helm    | 与 Helm CLI 交互。                                                                                        |
| mis   | microk8s.istio   | 与 Istio CLI 交互。                                                                                       |
| mk    | microk8s.kubectl | 与 Kubernetes CLI 交互。                                                                                  |
| msp   | microk8s.stop    | 停止所有 Kubernetes 服务。                                                                                |
| mst   | microk8s.start   | 在 MicroK8s 被停止后重新启动它。                                                                          |
| msts  | microk8s.status  | 显示 MicroK8s 状态概览(运行中 / 未运行)以及已启用的 addon 集合。                                          |
