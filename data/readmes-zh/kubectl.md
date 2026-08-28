# Kubectl 插件

本插件为 [Kubernetes 集群管理器](https://kubernetes.io/docs/reference/kubectl/kubectl/)
提供自动补全,并为常用的 kubectl 命令提供了一些别名。

✅ 启用方式:把「kubectl」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名     | 命令                                                    | 说明                                                                                             |
| :------- | :------------------------------------------------------ | :----------------------------------------------------------------------------------------------- |
| k        | `kubectl`                                               | kubectl 命令本身                                                                                 |
| kca      | `kubectl --all-namespaces`                              | 作用于所有命名空间(namespace)的 kubectl 命令                                                   |
| kaf      | `kubectl apply -f`                                      | 应用一个 YML 文件                                                                                |
| kapk     | `kubectl apply -k`                                      | 应用一个 kustomization 目录                                                                      |
| keti     | `kubectl exec -ti`                                      | 进入某个容器的交互式终端                                                                         |
|          |                                                         | **快速管理配置,在 local、dev 和 staging 之间切换上下文**                                        |
| kcuc     | `kubectl config use-context`                            | 在 kubeconfig 文件中设置 current-context                                                         |
| kcsc     | `kubectl config set-context`                            | 在 kubeconfig 中设置一个 context 条目                                                            |
| kcdc     | `kubectl config delete-context`                         | 从 kubeconfig 中删除指定的 context                                                               |
| kccc     | `kubectl config current-context`                        | 显示 current-context                                                                             |
| kcgc     | `kubectl config get-contexts`                           | 列出所有可用的 context                                                                           |
|          |                                                         | **通用别名**                                                                                     |
| kdel     | `kubectl delete`                                        | 按文件名、标准输入、资源与名称,或按资源与标签选择器删除资源                                     |
| kdelf    | `kubectl delete -f`                                     | 按 `-f` 参数指定的类型和名称删除 Pod                                                             |
| kdelk    | `kubectl delete -k`                                     | 删除 kustomization 目录中定义的所有资源                                                          |
| kge      | `kubectl get events --sort-by=".lastTimestamp"`         | 获取事件(按时间戳排序)                                                                         |
| kgew     | `kubectl get events --watch --sort-by=".lastTimestamp"` | 获取事件并持续监视新发生的事件(按时间戳排序)                                                   |
|          |                                                         | **Pod 管理**                                                                                     |
| kgp      | `kubectl get pods`                                      | 以 ps 输出格式列出所有 Pod                                                                       |
| kgpl     | `kgp -l`                                                | 按标签获取 Pod。示例:`kgpl "app=myapp" -n myns`                                                  |
| kgpn     | `kgp -n`                                                | 按命名空间获取 Pod。示例:`kgpn kube-system`                                                      |
| kgpsl    | `kubectl get pods --show-labels`                        | 以 ps 输出格式列出所有 Pod,并显示标签                                                           |
| kgpa     | `kubectl get pods --all-namespaces`                     | 以 ps 输出格式列出所有命名空间中的 Pod                                                           |
| kgpw     | `kgp --watch`                                           | 列出/获取请求的对象后,持续监视其变更                                                            |
| kgpwide  | `kgp -o wide`                                           | 以纯文本格式输出,并附带额外信息。对 Pod 而言,还会包含节点名称                                  |
| kgpall   | `kubectl get pods --all-namespaces -o wide`             | 以 wide 输出格式列出所有命名空间中的 Pod(包含节点名称)                                         |
| kep      | `kubectl edit pods`                                     | 用默认编辑器编辑 Pod                                                                             |
| kdp      | `kubectl describe pods`                                 | 显示所有 Pod 的详细信息                                                                          |
| kdelp    | `kubectl delete pods`                                   | 删除与传入参数匹配的所有 Pod                                                                     |
|          |                                                         | **Service 管理**                                                                                 |
| kgs      | `kubectl get svc`                                       | 以 ps 输出格式列出所有 Service                                                                   |
| kgsa     | `kubectl get svc --all-namespaces`                      | 列出所有命名空间中的 Service                                                                     |
| kgsw     | `kgs --watch`                                           | 列出所有 Service 后,持续监视其变更                                                              |
| kgswide  | `kgs -o wide`                                           | 列出所有 Service 后,以纯文本格式输出并附带额外信息                                              |
| kes      | `kubectl edit svc`                                      | 用默认编辑器编辑 Service(svc)                                                                  |
| kds      | `kubectl describe svc`                                  | 详细显示所有 Service 的信息                                                                      |
| kdels    | `kubectl delete svc`                                    | 删除与传入参数匹配的所有 Service                                                                 |
|          |                                                         | **Ingress 管理**                                                                                 |
| kgi      | `kubectl get ingress`                                   | 以 ps 输出格式列出 Ingress 资源                                                                  |
| kgia     | `kubectl get ingress --all-namespaces`                  | 列出所有命名空间中的 Ingress 资源                                                                |
| kei      | `kubectl edit ingress`                                  | 用默认编辑器编辑 Ingress 资源                                                                    |
| kdi      | `kubectl describe ingress`                              | 详细显示 Ingress 资源的信息                                                                      |
| kdeli    | `kubectl delete ingress`                                | 删除与传入参数匹配的 Ingress 资源                                                                |
|          |                                                         | **Namespace 管理**                                                                               |
| kgns     | `kubectl get namespaces`                                | 列出集群中当前的命名空间                                                                         |
| kcn      | `kubectl config set-context --current --namespace`      | 更改当前命名空间                                                                                 |
| kens     | `kubectl edit namespace`                                | 用默认编辑器编辑 namespace 资源                                                                  |
| kdns     | `kubectl describe namespace`                            | 详细显示 namespace 资源的信息                                                                    |
| kdelns   | `kubectl delete namespace`                              | 删除命名空间。警告!这会删除该命名空间中的所有内容                                               |
|          |                                                         | **ConfigMap 管理**                                                                               |
| kgcm     | `kubectl get configmaps`                                | 以 ps 输出格式列出 ConfigMap                                                                     |
| kgcma    | `kubectl get configmaps --all-namespaces`               | 列出所有命名空间中的 ConfigMap                                                                   |
| kecm     | `kubectl edit configmap`                                | 用默认编辑器编辑 configmap 资源                                                                  |
| kdcm     | `kubectl describe configmap`                            | 详细显示 configmap 资源的信息                                                                    |
| kdelcm   | `kubectl delete configmap`                              | 删除该 ConfigMap                                                                                 |
|          |                                                         | **Secret 管理**                                                                                  |
| kgsec    | `kubectl get secret`                                    | 获取 secret 以进行解码                                                                           |
| kgseca   | `kubectl get secret --all-namespaces`                   | 列出所有命名空间中的 secret                                                                      |
| kdsec    | `kubectl describe secret`                               | 详细显示 secret 资源的信息                                                                       |
| kdelsec  | `kubectl delete secret`                                 | 删除该 secret                                                                                    |
|          |                                                         | **Deployment 管理**                                                                              |
| kgd      | `kubectl get deployment`                                | 获取 deployment                                                                                  |
| kgda     | `kubectl get deployment --all-namespaces`               | 列出所有命名空间中的 deployment                                                                  |
| kgdw     | `kgd --watch`                                           | 获取 deployment 后,持续监视其变更                                                               |
| kgdwide  | `kgd -o wide`                                           | 获取 deployment 后,以纯文本格式输出并附带额外信息                                               |
| ked      | `kubectl edit deployment`                               | 用默认编辑器编辑 deployment 资源                                                                 |
| kdd      | `kubectl describe deployment`                           | 详细显示 deployment 资源的信息                                                                   |
| kdeld    | `kubectl delete deployment`                             | 删除该 deployment                                                                                |
| ksd      | `kubectl scale deployment`                              | 扩缩容一个 deployment                                                                            |
| krsd     | `kubectl rollout status deployment`                     | 检查 deployment 的发布(rollout)状态                                                            |
| krrd     | `kubectl rollout restart deployment`                    | 滚动重启一个 deployment                                                                          |
| kres     | `kubectl set env $@ REFRESHED_AT=...`                   | 以零停机方式重建 deployment 中的所有 Pod                                                         |
|          |                                                         | **Rollout 管理**                                                                                 |
| kgrs     | `kubectl get replicaset`                                | 列出由 deployment 创建的所有 ReplicaSet `rs`                                                     |
| kdrs     | `kubectl describe replicaset`                           | 详细显示 ReplicaSet 的信息                                                                       |
| kers     | `kubectl edit replicaset`                               | 用默认编辑器编辑 ReplicaSet                                                                      |
| krh      | `kubectl rollout history`                               | 查看该 deployment 的修订版本                                                                     |
| kru      | `kubectl rollout undo`                                  | 回滚到上一个修订版本                                                                             |
|          |                                                         | **端口转发**                                                                                     |
| kpf      | `kubectl port-forward`                                  | 把一个或多个本地端口转发到 Pod                                                                   |
|          |                                                         | **访问全部信息的工具**                                                                           |
| kga      | `kubectl get all`                                       | 以 ps 格式列出所有资源                                                                           |
| kgaa     | `kubectl get all --all-namespaces`                      | 列出所有命名空间中的目标对象                                                                     |
|          |                                                         | **日志**                                                                                         |
| kl       | `kubectl logs`                                          | 打印容器或资源的日志                                                                             |
| kl1h     | `kubectl logs --since 1h`                               | 打印容器或资源最近一小时内的日志                                                                 |
| kl1m     | `kubectl logs --since 1m`                               | 打印容器或资源最近一分钟内的日志                                                                 |
| kl1s     | `kubectl logs --since 1s`                               | 打印容器或资源最近一秒内的日志                                                                   |
| klf      | `kubectl logs -f`                                       | 流式输出容器或资源的日志(跟踪)                                                                 |
| klf1h    | `kubectl logs --since 1h -f`                            | 流式输出容器或资源最近一小时内的日志(跟踪)                                                     |
| klf1m    | `kubectl logs --since 1m -f`                            | 流式输出容器或资源最近一分钟内的日志(跟踪)                                                     |
| klf1s    | `kubectl logs --since 1s -f`                            | 流式输出容器或资源最近一秒内的日志(跟踪)                                                       |
|          |                                                         | **文件复制**                                                                                     |
| kcp      | `kubectl cp`                                            | 在容器与本机之间复制文件和目录                                                                   |
|          |                                                         | **Node 管理**                                                                                    |
| kgno     | `kubectl get nodes`                                     | 以 ps 输出格式列出节点                                                                           |
| kgnosl   | `kubectl get nodes --show-labels`                       | 以 ps 输出格式列出节点,并显示标签                                                               |
| keno     | `kubectl edit node`                                     | 用默认编辑器编辑 node 资源                                                                       |
| kdno     | `kubectl describe node`                                 | 详细显示 node 资源的信息                                                                         |
| kdelno   | `kubectl delete node`                                   | 删除该节点                                                                                       |
|          |                                                         | **Persistent Volume Claim 管理**                                                                 |
| kgpvc    | `kubectl get pvc`                                       | 列出所有 PVC                                                                                     |
| kgpvca   | `kubectl get pvc --all-namespaces`                      | 列出所有命名空间中的 PVC                                                                         |
| kgpvcw   | `kgpvc --watch`                                         | 列出/获取请求的对象后,持续监视其变更                                                            |
| kepvc    | `kubectl edit pvc`                                      | 用默认编辑器编辑 PVC                                                                             |
| kdpvc    | `kubectl describe pvc`                                  | 显示所有 PVC 的详细信息                                                                          |
| kdelpvc  | `kubectl delete pvc`                                    | 删除与传入参数匹配的所有 PVC                                                                     |
|          |                                                         | **StatefulSets 管理**                                                                            |
| kgss     | `kubectl get statefulset`                               | 以 ps 格式列出 statefulset                                                                       |
| kgssa    | `kubectl get statefulset --all-namespaces`              | 列出所有命名空间中的 statefulset                                                                 |
| kgssw    | `kgss --watch`                                          | 获取 statefulset 列表后,持续监视其变更                                                          |
| kgsswide | `kgss -o wide`                                          | 获取 statefulset 后,以纯文本格式输出并附带额外信息                                              |
| kess     | `kubectl edit statefulset`                              | 用默认编辑器编辑 statefulset 资源                                                                |
| kdss     | `kubectl describe statefulset`                          | 详细显示 statefulset 资源的信息                                                                  |
| kdelss   | `kubectl delete statefulset`                            | 删除该 statefulset                                                                               |
| ksss     | `kubectl scale statefulset`                             | 扩缩容一个 statefulset                                                                           |
| krsss    | `kubectl rollout status statefulset`                    | 检查 deployment 的发布(rollout)状态                                                            |
| krrss    | `kubectl rollout restart statefulset`                   | 滚动重启一个 statefulset                                                                         |
|          |                                                         | **Service Accounts 管理**                                                                        |
| kdsa     | `kubectl describe sa`                                   | 详细显示某个 Service Account 的信息                                                              |
| kdelsa   | `kubectl delete sa`                                     | 删除该 Service Account                                                                           |
|          |                                                         | **DaemonSet 管理**                                                                               |
| kgds     | `kubectl get daemonset`                                 | 以 ps 输出格式列出所有 DaemonSet                                                                 |
| kgdsa    | `kubectl get daemonset --all-namespaces`                | 列出所有命名空间中的所有 DaemonSet                                                               |
| kgdsw    | `kgds --watch`                                          | 列出所有 DaemonSet 后,持续监视其变更                                                            |
| keds     | `kubectl edit daemonset`                                | 用默认编辑器编辑 DaemonSet                                                                       |
| kdds     | `kubectl describe daemonset`                            | 详细显示所有 DaemonSet 的信息                                                                    |
| kdelds   | `kubectl delete daemonset`                              | 删除与传入参数匹配的所有 DaemonSet                                                               |
|          |                                                         | **CronJob 管理**                                                                                 |
| kgcj     | `kubectl get cronjob`                                   | 以 ps 输出格式列出所有 CronJob                                                                   |
| kecj     | `kubectl edit cronjob`                                  | 用默认编辑器编辑 CronJob                                                                         |
| kdcj     | `kubectl describe cronjob`                              | 详细显示某个 CronJob 的信息                                                                      |
| kdelcj   | `kubectl delete cronjob`                                | 删除该 CronJob                                                                                   |
|          |                                                         | **Job 管理**                                                                                     |
| kgj      | `kubectl get job`                                       | 以 ps 输出格式列出所有 Job                                                                       |
| kej      | `kubectl edit job`                                      | 编辑一个 Job 的详细信息                                                                          |
| kdj      | `kubectl describe job`                                  | 显示该 Job 的信息                                                                                |
| kdelj    | `kubectl delete job`                                    | 删除该 Job                                                                                       |

## 包装器

本插件提供 3 个包装器(wrapper),借助各种工具(必须已安装)为 kubectl 的
JSON 和 YAML 输出着色:

- `kj`:JSON,使用 [`jq`](https://stedolan.github.io/jq/) 着色。
- `kjx`:JSON,使用 [`fx`](https://github.com/antonmedv/fx) 着色。
- `ky`:YAML,使用 [`yh`](https://github.com/andreazorzetto/yh) 着色。
