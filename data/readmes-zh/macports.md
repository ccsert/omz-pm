# Macports 插件

本插件为包管理器 [Macports](https://macports.com/) 提供自动补全,
并为常用的 Macports 命令提供了一些别名。

✅ 启用方式:把「macports」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名  | 命令                               | 说明                                                          |
|-------|------------------------------------|---------------------------------------------------------------|
| pc    | `sudo port clean --all installed`  | 清理已安装 port 的中间安装文件                                |
| pi    | `sudo port install`                | 安装作为参数给出的包                                          |
| pli   | `port livecheck installed`         | 检查已安装的 port 是否有更新                                  |
| plm   | `port-livecheck-maintainer`        | 检查指定维护者所维护的 port 是否有更新                        |
| psu   | `sudo port selfupdate`             | 用 MacPorts 仓库更新 ports 树                                 |
| puni  | `sudo port uninstall inactive`     | 卸载处于 inactive 状态的 port                                 |
| puo   | `sudo port upgrade outdated`        | 升级有更新版本可用的 port                                     |
| pup   | `psu && puo`                       | 先更新 ports 树,再把 port 升级到最新版本                     |

## 命令

### port-livecheck-maintainer

```text
Usage:
  port-livecheck-maintainer
  port-livecheck-maintainer (maintainer)+
  port-livecheck-maintainer -h|--help

Check

Options:
  maintainer  maintainer id
  -h          print this help message and exit
```

检查当前用户所维护的 port,或指定的维护者表达式列表所维护的 port 是否有可用更新。
当前用户的维护者 id 按以下方式获取:

* `MACPORTS_MAINTAINER` 变量的值(如果已设置且不为空)。
* `USER` 变量的值。
