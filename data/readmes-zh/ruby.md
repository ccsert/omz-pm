# Ruby 插件

本插件为处理 [Ruby](https://www.ruby-lang.org/en/) 和 [gem 包](https://rubygems.org/)的常用命令添加了别名。

✅ 启用方式:把「ruby」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名    | 命令                                   | 说明                                                 |
| ------- | -------------------------------------- | ---------------------------------------------------- |
| rb      | `ruby`                                 | Ruby 命令                                            |
| sgem    | `sudo gem`                             | 在系统 ruby 上运行 sudo gem,而不是当前激活的 ruby   |
| rfind   | `find . -name "*.rb" \| xargs grep -n` | 查找 ruby 文件                                       |
| rrun    | `ruby -e`                              | 执行一段代码:例如 `rrun "puts 'Hello world!'"`       |
| rserver | `ruby -e httpd . -p 8080`              | 启动 HTTP Webrick 来伺服本地目录/文件                |
| gein    | `gem install`                          | 把 gem 安装到本地仓库                                |
| geun    | `gem uninstall`                        | 从本地仓库卸载 gem                                   |
| geli    | `gem list`                             | 显示本地已安装的 gem                                 |
| gei     | `gem info`                             | 显示指定 gem 的信息                                  |
| geiall  | `gem info --all`                       | 显示 gem 的所有版本                                  |
| geca    | `gem cert --add`                       | 添加受信任的证书                                     |
| gecr    | `gem cert --remove`                    | 移除受信任的证书                                     |
| gecb    | `gem cert --build`                     | 生成私钥和自签名证书                                 |
| geclup  | `gem cleanup -n`                       | 不卸载 gem                                           |
| gegi    | `gem generate_index`                   | 为 gem 服务器生成索引文件                            |
| geh     | `gem help`                             | 提供更多帮助信息                                     |
| gel     | `gem lock`                             | 生成 gem 锁定列表                                    |
| geo     | `gem open`                             | 在默认编辑器中打开 gem 源码                          |
| geoe    | `gem open -e`                          | 在首选编辑器中打开 gem 源码                          |
