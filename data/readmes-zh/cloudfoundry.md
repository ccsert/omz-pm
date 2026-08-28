# Cloudfoundry 插件

本插件旨在为 [Cloud Foundry Cli][1] 的常用用户提供一些简单的别名。大多数只是能省一点输入量的简单别名,
另一些则包含小型函数或可以接受参数。详情请看下面的表格。

| 别名     | 命令                        | 说明                                                                     |
|----------|-----------------------------|--------------------------------------------------------------------------|
| cfl      | `cf login`                  | 登录 Cloud Foundry                                                       |
| cft      | `cf target`                 | 把 CLI 指向 Cloud Foundry 中特定的 Org/Space                                   |
| cfa      | `cf apps`                   | 列出当前 Org/Space 中的所有应用                                             |
| cfs      | `cf services`               | 列出当前 Org/Space 中的所有服务                                             |
| cfm      | `cf marketplace`            | 列出 Marketplace 中可用的服务                                               |
| cfp      | `cf push`                   | 把你的应用代码推送到 Cloud Foundry                                            |
| cfcs     | `cf create-service`         | 基于 Marketplace 的服务项创建服务                                            |
| cfbs     | `cf bind-service`           | 把应用绑定到你创建的服务                                                      |
| cfus     | `cf unbind-service`         | 把服务与应用解绑                                                          |
| cfds     | `cf delete-service`         | 删除不再被绑定的服务                                                        |
| cfup     | `cf cups`                   | 创建一个「用户自定义服务」(user-provided-service)                                  |
| cflg     | `cf logs`                   | 跟踪(tail)一个应用的日志(需要 <APP_NAME>)                                    |
| cfr      | `cf routes`                 | 列出当前 Space 中的所有路由                                                   |
| cfe      | `cf env`                    | 显示一个应用的环境变量(需要 <APP_NAME>)                                       |
| cfsh     | `cf ssh`                    | 连接到运行中的容器(需要 <APP_NAME> 等参数)                                     |
| cfsc     | `cf scale`                  | 对应用进行扩缩容(需要 <APP_NAME> 等参数)                                     |
| cfev     | `cf events`                 | 显示应用事件(需要 <APP_NAME>)                                             |
| cfdor    | `cf delete-orphaned-routes` | 删除不再绑定到应用的路由                                                      |
| cfbpk    | `cf buildpacks`             | 列出可用的 buildpack                                                      |
| cfdm     | `cf domains`                | 列出与该 Cloud Foundry foundation 关联的域名                                |
| cfsp     | `cf spaces`                 | 列出当前 Org 中的所有 Space                                                  |
| cfap     | `cf app`                    | 显示已部署应用的详细信息(需要 <APP_NAME>)                                     |
| cfh.     | `export CF_HOME=$PWD/.cf`   | 把当前目录设为 CF_HOME                                                       |
| cfh~     | `export CF_HOME=~/.cf`      | 把用户主目录设为 CF_HOME                                                      |
| cfhu     | `unset CF_HOME`             | 取消设置 CF_HOME                                                          |
| cfpm     | `cf push -f`                | 使用 manifest 推送应用(需要 <MANIFEST_FILE> 位置)                              |
| cflr     | `cf logs --recent`          | 显示最近的日志(需要 <APP_NAME>)                                           |
| cfsrt    | `cf start`                  | 启动一个应用(需要 <APP_NAME>)                                             |
| cfstp    | `cf stop`                   | 停止一个应用(需要 <APP_NAME>)                                             |
| cfstg    | `cf restage`                | 重新构建(restage)一个应用(需要 <APP_NAME>)                                 |
| cfdel    | `cf delete`                 | 删除一个应用(需要 <APP_NAME>)                                             |
| cfsrtall | -                           | 启动所有当前处于「Stopped」状态的应用                                            |
| cfstpall | -                           | 停止所有当前处于「Started」状态的应用                                             |

想了解某个命令的作用并获得帮助,可以按如下方式使用 `cf` 内置的帮助功能:

```bash
cf help # List the most popular and commonly used commands
cf help -a # Complete list of all possible commands
cf <COMMAND_NAME> --help # Help on a specific command including arguments and examples
```

也可以查阅[在线文档][3]。另外别忘了,cf-cli 命令行工具有很多优秀的[社区插件][4],
能大幅扩展它的能力和实用性。

## 贡献者

本插件由 [benwilcock][2] 贡献给 `oh_my_zsh`。

[1]: https://docs.cloudfoundry.org/cf-cli/install-go-cli.html
[2]: https://github.com/benwilcock
[3]: https://docs.cloudfoundry.org/cf-cli/getting-started.html
[4]: https://plugins.cloudfoundry.org/
