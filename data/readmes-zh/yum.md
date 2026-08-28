# Yum 插件

本插件为常用的 [Yum](http://yum.baseurl.org/) 命令添加了一些实用别名。

✅ 启用方式:把「yum」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名 | 命令                              | 说明                   |
| ---- | --------------------------------- | ---------------------- |
| ys   | `yum search`                      | 搜索软件包             |
| yp   | `yum info`                        | 显示软件包信息         |
| yl   | `yum list`                        | 列出软件包             |
| ygl  | `yum grouplist`                   | 列出软件包组           |
| yli  | `yum list installed`              | 打印所有已安装的软件包 |
| ymc  | `yum makecache`                   | 重建 yum 软件包列表    |
| yu   | `sudo yum update`                 | 升级软件包             |
| yi   | `sudo yum install`                | 安装软件包             |
| ygi  | `sudo yum groupinstall`           | 安装软件包组           |
| yr   | `sudo yum remove`                 | 删除软件包             |
| ygr  | `sudo yum groupremove`            | 删除软件包组           |
| yrl  | `sudo yum remove --remove-leaves` | 删除软件包及其叶子依赖 |
| yc   | `sudo yum clean all`              | 清理 yum 缓存          |
