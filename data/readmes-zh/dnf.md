# dnf 插件

本插件为最常用的命令添加了别名,让 `dnf` 更容易使用。

`dnf` 是基于 RPM 的发行版的新一代包管理器,用于取代 `yum`。

✅ 启用方式:把「dnf」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

经典版 `dnf` 正在逐渐被 `dnf5` 取代;本插件会检测 `dnf5` 是否存在,若存在则用它作为较慢的 `dnf` 的直接替代品。

## 别名

| 别名  | 命令                          | 说明                                     |
|-------|-------------------------------|------------------------------------------|
| dnfl  | `dnf list`                    | 列出软件包                               |
| dnfli | `dnf list --installed`        | 列出已安装的软件包                       |
| dnfgl | `dnf grouplist`               | 列出软件包组                             |
| dnfmc | `dnf makecache`               | 生成元数据缓存                           |
| dnfp  | `dnf info`                    | 显示软件包信息                           |
| dnfs  | `dnf search`                  | 搜索软件包                               |
| **使用 `sudo`**                                                                     |
| dnfu  | `sudo dnf upgrade`            | 升级软件包                               |
| dnfur | `sudo dnf upgrade --refresh`  | 升级软件包(强制刷新元数据)              |
| dnfi  | `sudo dnf install`            | 安装软件包                               |
| dnfgi | `sudo dnf groupinstall`       | 安装软件包组                             |
| dnfr  | `sudo dnf remove`             | 移除软件包                               |
| dnfgr | `sudo dnf groupremove`        | 移除软件包组                             |
| dnfc  | `sudo dnf clean all`          | 清理缓存                                 |
