# FirewallD 插件

本插件为 FirewallD 提供了一些别名和函数,基于 `firewalld-cmd` 命令。

✅ 启用方式:把「firewalld」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名  | 命令                                       | 说明             |
| :---- | :----------------------------------------- | :--------------- |
| fw    | `sudo firewall-cmd`                        | 简写形式         |
| fwr   | `sudo firewall-cmd --reload`               | 重新加载当前配置 |
| fwp   | `sudo firewall-cmd --permanent`            | 创建永久规则     |
| fwrp  | `sudo firewall-cmd --runtime-to-permanent` | 保存当前配置     |

## 函数

| 函数 | 说明                                                         |
| :--- | :----------------------------------------------------------- |
| fwl  | 列出所有活动区域(active zone)的配置以及直接规则(direct rule) |
