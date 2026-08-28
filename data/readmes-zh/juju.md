# juju 插件

本插件为 [juju](https://juju.is/) 提供实用的别名和函数(TAB 补全请参考[官方仓库](https://github.com/juju/juju/blob/develop/etc/bash_completion.d/juju))。

✅ 启用方式:把「juju」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

命名约定:

- `!` 后缀:表示 `--force --no-wait -y`。
- `ds` 后缀:表示 `--destroy-storage`。
- `jsh` 前缀:表示 `juju show-*`。

### 通用

| 别名    | 命令                                        | 说明                                                   |
|---------|---------------------------------------------|--------------------------------------------------------|
| `j`     | `juju`                                      | juju 命令本身                                          |
| `jcld`  | `juju clouds`                               | 列出所有已登记凭据的云                                 |
| `jclda` | `juju clouds --all`                         | 列出 Juju 可用的所有云                                 |
| `jdl`   | `juju debug-log --ms`                       | 显示日志,精确到毫秒                                   |
| `jdlr`  | `juju debug-log --ms --replay`              | 完整回放日志                                           |
| `jh`    | `juju help`                                 | 显示某个命令或其他主题的帮助                           |
| `jshsl` | `juju show-status-log`                      | 输出指定实体的历史状态                                 |
| `jstj`  | `juju status --format=json`                 | 以 json 格式显示状态(更详细)                         |
| `jst`   | `juju status --relations --color`           | 以彩色显示状态(含关系)                               |
| `jsts`  | `juju status --relations --storage --color` | 以彩色显示状态(含关系与存储)                         |

### 引导(Bootstrap)

| 别名    | 命令                                | 说明                                                  |
|---------|-------------------------------------|-------------------------------------------------------|
| `jb`    | `juju bootstrap`                    | 初始化一个 Juju 云环境                                |
| `jbng`  | `juju bootstrap --no-gui`           | 初始化不带 GUI 的 Juju 云环境                         |
| `jbl`   | `juju bootstrap localhost`          | 初始化 lxd 云环境                                     |
| `jblng` | `juju bootstrap --no-gui localhost` | 初始化不带 GUI 的 lxd 云环境                          |
| `jbm`   | `juju bootstrap microk8s`           | 初始化 MicroK8s 云环境                                |
| `jbmng` | `juju bootstrap --no-gui microk8s`  | 初始化不带 GUI 的 MicroK8s 云环境                     |

### 控制器

| 别名     | 命令                                                                                  | 说明                                                              |
|----------|---------------------------------------------------------------------------------------|-------------------------------------------------------------------|
| `jctl`   | `juju controllers`                                                                    | 列出所有控制器                                                    |
| `jctlr`  | `juju controllers --refresh`                                                          | 列出所有控制器(下载最新详情)                                    |
| `jdc`    | `juju destroy-controller --destroy-all-models`                                        | 销毁一个控制器                                                    |
| `jdc!`   | `juju destroy-controller --destroy-all-models --force --no-wait -y`                   | 销毁一个控制器                                                    |
| `jdcds`  | `juju destroy-controller --destroy-all-models --destroy-storage`                      | 销毁控制器及其关联的存储                                          |
| `jdcds!` | `juju destroy-controller --destroy-all-models --destroy-storage --force --no-wait -y` | 销毁控制器及其关联的存储                                          |
| `jkc`    | `juju kill-controller -y -t 0`                                                        | 强制终止 Juju 控制器的所有关联资源                                |
| `jshc`   | `juju show-controller`                                                                | 显示某个控制器的详细信息                                          |
| `jsw`    | `juju switch`                                                                         | 选择或查看当前的控制器与模型                                      |

### 模型

| 别名     | 命令                                                        | 说明                                                  |
|----------|-------------------------------------------------------------|-------------------------------------------------------|
| `jam`    | `juju add-model`                                            | 添加一个托管模型                                      |
| `jdm`    | `juju destroy-model`                                        | 不可恢复地彻底删除一个模型                            |
| `jdm!`   | `juju destroy-model --force --no-wait -y`                   | 不可恢复地彻底删除一个模型                            |
| `jdmds`  | `juju destroy-model --destroy-storage`                      | 不可恢复地彻底删除一个模型                            |
| `jdmds!` | `juju destroy-model --destroy-storage --force --no-wait -y` | 不可恢复地彻底删除一个模型                            |
| `jmc`    | `juju model-config`                                         | 显示或设置模型的配置值                                |
| `jm`     | `juju models`                                               | 列出用户在控制器上可访问的模型                        |
| `jshm`   | `juju show-model`                                           | 显示当前或指定模型的信息                              |
| `jsw`    | `juju switch`                                               | 选择或查看当前的控制器与模型                          |

### 应用 / 单元

| 别名     | 命令                                                          | 说明                                                                      |
|----------|---------------------------------------------------------------|---------------------------------------------------------------------------|
| `jc`     | `juju config`                                                 | 获取、设置或重置已部署应用的配置                                          |
| `jde`    | `juju deploy --channel=edge`                                  | 从 edge 通道部署新的应用或 bundle                                         |
| `jd`     | `juju deploy`                                                 | 部署新的应用或 bundle                                                     |
| `jra`    | `juju run-action`                                             | 将一个 action 加入执行队列                                                |
| `jraw`   | `juju run-action --wait`                                      | 将 action 加入执行队列并等待结果,可选超时                                |
| `jrm`    | `juju remove-application`                                     | 移除应用                                                                  |
| `jrm!`   | `juju remove-application --force --no-wait`                   | 强制移除应用                                                              |
| `jrmds`  | `juju remove-application --destroy-storage`                   | 移除应用并销毁其挂载的存储                                                |
| `jrmds!` | `juju remove-application --destroy-storage --force --no-wait` | 强制移除应用,并销毁其挂载的存储                                          |
| `jrp`    | `juju refresh --path`                                         | 从本地 charm 文件升级 charm                                               |
| `jsa`    | `juju scale-application`                                      | 设置应用单元的期望数量                                                    |
| `jssh`   | `juju ssh`                                                    | 在 Juju 目标上发起 SSH 会话或执行命令                                     |
| `jsshc`  | `juju ssh --container`                                        | 在指定容器上发起 SSH 会话或执行命令                                       |
| `jshu`   | `juju show-unit`                                              | 显示某个单元的信息                                                        |

### 存储

| 别名   | 命令                          | 说明                                            |
|--------|-------------------------------|-------------------------------------------------|
| `jrs`  | `juju remove-storage`         | 移除存储                                        |
| `jrs!` | `juju remove-storage --force` | 即使存储当前已挂载也强制移除                    |

### 关系

| 别名      | 命令                           | 说明                                                              |
|-----------|--------------------------------|-------------------------------------------------------------------|
| `jrel`    | `juju relate`                  | 建立两个应用之间的关系                                            |
| `jrmrel`  | `juju remove-relation`         | 移除两个应用之间已有的关系。                                      |
| `jrmrel!` | `juju remove-relation --force` | 强制移除两个应用之间已有的关系。                                  |

### 跨模型关系(CMR)

| 别名     | 命令               | 说明                                                           |
|----------|--------------------|----------------------------------------------------------------|
| `jex`    | `juju expose`      | 将应用公开暴露到网络上                                         |
| `jof`    | `juju offer`       | 将应用端点提供给其他模型使用                                   |
| `jcon`   | `juju consume`     | 将远程 offer 添加到模型中                                      |
| `jrmsas` | `juju remove-saas` | 从模型中移除被消费的应用(SAAS)                               |
| `junex`  | `juju unexpose`    | 取消应用在网络上的公开暴露                                     |

### Bundle

| 别名  | 命令                 | 说明                                                        |
|-------|----------------------|-------------------------------------------------------------|
| `jeb` | `juju export-bundle` | 将当前模型配置导出为可复用的 bundle                         |

## 函数

- `jaddr <app_name> [unit_num]`:显示应用或单元的 IP 地址。
- `jreld <relation_name> <app_name> <unit_num>`:显示应用与单元的关系数据。
- `jclean`:销毁所有控制器
- `jcontroller`:显示你当前连接的控制器。
- `jmodel`:显示你当前连接的模型。
- `wjst [interval_secs] [args_for_watch]`:监视 juju 状态,间隔可选(默认:5s);还可以向 `watch` 传递额外参数。
