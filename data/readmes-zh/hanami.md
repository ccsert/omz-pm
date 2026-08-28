# Hanami 插件

本插件为在控制台中使用 [Hanami](https://hanamirb.org/) 提供了一组方便的别名。
它受 Rails 插件启发,如果你用过后者,会有宾至如归的感觉。

✅ 启用方式:把「hanami」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

例如,在 Hanami 项目目录中,在控制台里输入 `hc` 即可运行应用控制台。可用的快捷方式请看下面的列表。你可以在[官方网站](https://hanamirb.org/guides/command-line/applications/)上进一步了解这些命令。

## 别名

| 别名  | 命令                        | 说明                                                    |
|-------|-----------------------------|---------------------------------------------------------|
| HED\* | `HANAMI_ENV=development`    | 把环境变量 HANAMI_ENV 设为 development                  |
| HEP\* | `HANAMI_ENV=production`     | 把环境变量 HANAMI_ENV 设为 production                   |
| HET\* | `HANAMI_ENV=test`           | 把环境变量 HANAMI_ENV 设为 test                         |
| hc    | `hanami console`            | 运行应用控制台                                          |
| hd    | `hanami destroy`            | 删除指定的 hanami 资源                                  |
| hg    | `hanami generate`           | 创建指定的 hanami 资源                                  |
| hgm   | `hanami generate migration` | 创建迁移文件                                            |
| hs    | `hanami server`             | 启动 hanami 应用的服务器                                |
| hsp   | `hanami server -p`          | 用指定端口启动服务器                                    |
| hr    | `hanami routes`             | 列出应用路由                                            |
| hdc   | `hanami db create`          | 创建应用数据库                                          |
| hdd   | `hanami db drop`            | 删除应用数据库                                          |
| hdp   | `hanami db prepare`         | 为当前环境准备数据库                                    |
| hda   | `hanami db apply`           | 在迁移之后重建全新的 schema(具有破坏性)                 |
| hdv   | `hanami db version`         | 打印当前数据库版本                                      |
| hdrs  | `hdd && hdp`                | 删除并重建应用数据库                                    |
| hdtp  | `HET hdp`                   | 更新测试环境的数据库                                    |
| hrg   | `hr | grep`                 | 用指定模式过滤 hanami 路由                              |

\* 这几个别名应放在命令的开头使用,例如:

```console
$ HED hdd # equivalent to 'HANAMI_ENV=development hanami db drop'
```
