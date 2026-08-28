# sbt 插件

本插件为 [sbt,交互式构建工具](https://scala-sbt.org/)提供自动补全,
并为常用的 sbt 命令提供了一些别名。

✅ 启用方式:把「sbt」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名  | 命令                  | 说明                                                         |
|-------|-----------------------|--------------------------------------------------------------|
| sbc   | `sbt compile`         | 编译主要源码                                                 |
| sbcln | `sbt clean`           | 删除所有生成的文件                                           |
| sbcc  | `sbt clean compile`   | 删除生成的文件,并编译主要源码                               |
| sbco  | `sbt console`         | 启动 Scala,带上已编译的源码和全部依赖                       |
| sbcq  | `sbt consoleQuick`    | 启动 Scala,带上全部依赖                                     |
| sbcp  | `sbt consoleProject`  | 启动 Scala,带上 sbt 和构建定义                              |
| sbd   | `sbt doc`             | 为 Scala 源文件生成 API 文档                                 |
| sbdc  | `sbt dist:clean`      | 删除分发包                                                   |
| sbdi  | `sbt dist`            | 创建分发包                                                   |
| sbgi  | `sbt genIdea`         | 创建 Idea 项目文件                                           |
| sbp   | `sbt publish`         | 把构件发布到仓库                                             |
| sbpl  | `sbt publishLocal`    | 把构件发布到本地 Ivy 仓库                                    |
| sbr   | `sbt run`             | 运行项目的主类                                               |
| sbrm  | `sbt runMain`         | 运行项目的指定主类                                           |
| sbu   | `sbt update`          | 解析并拉取外部依赖                                           |
| sbx   | `sbt test`            | 编译并运行所有测试                                           |
| sba   | `sbt assembly`        | 创建包含全部依赖的 fat JAR                                   |
