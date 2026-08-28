# suse

依照 Zypper 官方别名整理的 [Zypper](https://en.opensuse.org/Portal:Zypper) 别名。

✅ 启用方式:把「suse」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

**维护者**: [r-darwish](https://github.com/r-darwish)

## 主要命令

| 别名             | 命令                          | 说明                                                           |
| ---------------- | ----------------------------- | -------------------------------------------------------------- |
| z                | `sudo zypper`                 | 调用 zypper                                                    |
| zh               | `zypper -h`                   | 打印帮助                                                       |
| zhse             | `zypper -h se`                | 打印 search 命令的帮助                                         |
| zlicenses        | `zypper licenses`             | 打印已安装软件包的许可证与 EULA 报告                           |
| zps              | `sudo zypper ps`              | 列出正在使用已删除文件的进程                                   |
| zshell           | `sudo zypper shell`           | 打开一个 zypper shell 会话                                     |
| zsource-download | `sudo zypper source-download` | 为所有已安装软件包下载源码 rpm                                 |
| ztos             | `zypper tos`                  | 显示目标操作系统的 ID 字符串                                   |
| zvcmp            | `zypper vcmp`                 | 判断 version1 比 version2 旧还是新                             |

## 软件包命令

| 别名  | 命令              | 说明                                                               |
| ----- | ----------------- | ------------------------------------------------------------------ |
| zin   | `sudo zypper in`  | 安装软件包                                                         |
| zinr  | `sudo zypper inr` | 安装已安装软件包新推荐的软件包                                     |
| zrm   | `sudo zypper rm`  | 删除软件包                                                         |
| zsi   | `sudo zypper si`  | 安装某个软件包的源码                                               |
| zve   | `sudo zypper ve`  | 校验已安装软件包的依赖                                             |

## 更新命令

| 别名   | 命令                | 说明                   |
| ------ | ------------------- | ---------------------- |
| zdup   | `sudo zypper dup`   | 升级软件包             |
| zlp    | `zypper lp`         | 列出必要的补丁         |
| zlu    | `zypper lu`         | 列出可用更新           |
| zpchk  | `sudo zypper pchk`  | 检查补丁               |
| zup    | `sudo zypper up`    | 更新软件包             |
| zpatch | `sudo zypper patch` | 安装补丁               |

## 查询命令

| 别名          | 命令                       | 说明                                                 |
| ------------- | -------------------------- | ---------------------------------------------------- |
| zif           | `zypper if`                | 显示软件包信息                                       |
| zpa           | `zypper pa`                | 列出软件包                                           |
| zpatch-info   | `zypper patch-info`        | 显示补丁信息                                         |
| zpattern-info | `zypper pattern-info`      | 显示 pattern 信息                                    |
| zproduct-info | `zypper product-info`      | 显示产品信息                                         |
| zpch          | `zypper pch`               | 列出所有补丁                                         |
| zpd           | `zypper pd`                | 列出产品                                             |
| zpt           | `zypper pt`                | 列出 pattern                                         |
| zse           | `zypper se`                | 搜索软件包                                           |
| zwp           | `zypper wp`                | 列出提供指定能力的所有软件包                         |

注意:这些别名会向 zypper 传入 `--no-refresh`,以加快调用速度,并避免因缺少
root 权限而出错。如果需要刷新软件仓库,请先运行 `sudo zypper ref`(`zref` 别名),
再使用这些别名。

相关:[#9798](https://github.com/ohmyzsh/ohmyzsh/pull/9798)。

## 软件仓库命令

| 别名  | 命令                | 说明                                     |
| ----- | ------------------- | ---------------------------------------- |
| zar   | `sudo zypper ar`    | 添加软件仓库                             |
| zcl   | `sudo zypper clean` | 清理缓存                                 |
| zlr   | `zypper lr`         | 列出软件仓库                             |
| zmr   | `sudo zypper mr`    | 修改软件仓库                             |
| znr   | `sudo zypper nr`    | 重命名软件仓库(仅对该别名生效)         |
| zref  | `sudo zypper ref`   | 刷新软件仓库                             |
| zrr   | `sudo zypper rr`    | 删除软件仓库                             |

## 服务命令

| 别名  | 命令               | 说明                                                           |
| ----- | ------------------ | -------------------------------------------------------------- |
| zas   | `sudo zypper as`   | 把 URI 指定的服务添加到系统                                    |
| zms   | `sudo zypper ms`   | 修改指定服务的属性                                             |
| zrefs | `sudo zypper refs` | 刷新服务即执行该服务的特定任务                                 |
| zrs   | `sudo zypper rs`   | 从系统中移除指定的仓库索引服务                                 |
| zls   | `zypper ls`        | 列出系统中定义的服务                                           |

## 软件包锁定管理命令

| 别名  | 命令             | 说明                                |
| ----- | ---------------- | ----------------------------------- |
| zal   | `sudo zypper al` | 添加软件包锁定                      |
| zcl   | `sudo zypper cl` | 删除未使用的锁定                    |
| zll   | `zypper ll`      | 列出当前生效的软件包锁定            |
| zrl   | `sudo zypper rl` | 删除指定的软件包锁定                |
