# Oh My Zsh 的 NestJS 插件

本插件为常用的 [NestJS CLI](https://docs.nestjs.com/cli/overview) 命令提供别名。

## 依赖要求

- 全局安装 [NestJS CLI](https://docs.nestjs.com/cli/overview#installation):
  `npm install -g @nestjs/cli`

## 别名

| 别名    | 命令                         | 说明                                        |
| :------ | :--------------------------- | :------------------------------------------ |
| `nnew`  | `nest new`                   | 创建新的 NestJS 项目                        |
| `nb`    | `nest build`                 | 构建 NestJS 应用                            |
| `ns`    | `nest start`                 | 启动应用                                    |
| `nsw`   | `nest start --watch`         | 以 watch 模式启动应用                       |
| `nsd`   | `nest start --dev`           | 以 dev 模式启动应用                         |
| `nsdbg` | `nest start --debug --watch` | 以 debug 和 watch 模式启动应用              |
| `ng`    | `nest generate`              | 生成一个 NestJS 元素                        |
| `ngm`   | `nest generate module`       | 生成模块                                    |
| `ngc`   | `nest generate controller`   | 生成控制器                                  |
| `ngs`   | `nest generate service`      | 生成服务                                    |
| `ngg`   | `nest generate guard`        | 生成守卫                                    |
| `ngp`   | `nest generate pipe`         | 生成管道                                    |
| `ngf`   | `nest generate filter`       | 生成过滤器                                  |
| `ngr`   | `nest generate resolver`     | 生成 GraphQL resolver                       |
| `ngcl`  | `nest generate class`        | 生成类                                      |
| `ngi`   | `nest generate interface`    | 生成接口                                    |
| `ngit`  | `nest generate interceptor`  | 生成拦截器                                  |
| `ngmi`  | `nest generate middleware`   | 生成中间件                                  |
| `ngd`   | `nest generate decorator`    | 生成自定义装饰器                            |
| `ngres` | `nest generate resource`     | 生成 CRUD 资源                              |
| `nglib` | `nest generate library`      | 生成新的库                                  |
| `ngsub` | `nest generate sub-app`      | 生成新的子应用(monorepo)                   |
| `na`    | `nest add`                   | 向项目添加一个库                            |
| `ni`    | `nest info`                  | 显示 NestJS 项目信息                        |
| `nu`    | `nest update`                | 更新 NestJS 依赖                            |

## 用法

✅ 启用方式:把「nestjs」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

2. 重启终端,或者重新加载你的 `~/.zshrc` 文件:

```zsh
source ~/.zshrc
```
