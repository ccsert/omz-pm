# Golang 插件

本插件为 [Go 编程语言](https://golang.org/)提供补全,以及一些常用 Golang 命令的别名。

✅ 启用方式:把「golang」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名   | 命令              | 说明                                                          |
| ------ | ----------------- | ------------------------------------------------------------- |
| gob    | `go build`        | 构建你的代码                                                   |
| goc    | `go clean`        | 从包源码目录中移除对象文件                                     |
| god    | `go doc`          | 打印文档注释                                                   |
| goe    | `go env`          | 打印 Go 环境信息                                               |
| gof    | `go fmt`          | Gofmt 对 Go 程序进行格式化(对齐与缩进)。                       |
| gofa   | `go fmt ./...`    | 递归地对当前目录下的所有包运行 go fmt                           |
| gofx   | `go fix`          | 把包更新为使用新的 API                                         |
| gog    | `go get`          | 下载包并安装到 $GOPATH                                         |
| goga   | `go get ./...`    | 递归安装当前目录下的所有依赖                                    |
| goi    | `go install`      | 编译包并把包安装到 $GOPATH                                      |
| gol    | `go list`         | 列出 Go 包                                                     |
| gom    | `go mod`          | 访问模块相关操作                                               |
| gomt   | `go mod tidy`     | 整理 go.mod 文件                                               |
| gopa   | `cd $GOPATH`      | 带你前往 `$GOPATH`                                             |
| gopb   | `cd $GOPATH/bin`  | 带你前往 `$GOPATH/bin`                                         |
| gops   | `cd $GOPATH/src`  | 带你前往 `$GOPATH/src`                                         |
| gor    | `go run`          | 编译并运行你的代码                                             |
| got    | `go test`         | 运行测试                                                       |
| gota   | `go test ./...`   | 运行所有子目录中的测试                                          |
| goto   | `go tool`         | 打印所有可用的工具                                             |
| gotoc  | `go tool compile` | 生成对象文件                                                   |
| gotod  | `go tool dist`    | 用于引导、构建和测试 go 运行时的工具                            |
| gotofx | `go tool fix`     | 修复应用以使用新特性                                           |
| gov    | `go vet`          | Vet 检查 Go 源代码并报告可疑的结构                              |
| gove   | `go version`      | 打印 Go 版本                                                   |
| gow    | `go work`         | Work 提供对工作区相关操作的访问                                 |
