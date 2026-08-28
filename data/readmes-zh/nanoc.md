# Nanoc 插件

本插件为常用的 [Nanoc](https://nanoc.ws) 命令添加了一些别名和自动补全。

✅ 启用方式:把「nanoc」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名 | 命令                  | 说明                                                                              |
| ---- | --------------------- | --------------------------------------------------------------------------------- |
| n    | `nanoc`               | Nanoc 主命令                                                                      |
| nco  | `nanoc compile`       | 编译当前站点的所有条目(item)                                                    |
| ncs  | `nanoc create-site`   | 在给定路径创建一个新站点。该站点将使用文件系统数据源                              |
| nd   | `nanoc deploy`        | 把已编译的站点部署到目标位置(用 `--target` 指定)                                |
| np   | `nanoc prune`         | 从输出目录中移除不受 Nanoc 管理的文件                                             |
| nv   | `nanoc view`          | 启动静态 Web 服务器(默认使用 3000 端口并监听所有 IP 地址,除非另行指定)          |
