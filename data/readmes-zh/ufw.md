# UFW 插件

本插件为管理大家最喜爱的简单防火墙 UFW(Uncomplicated Firewall)提供自动补全。
UFW 是一个管理 iptables 的简单接口。想了解更多,请查阅 [`UFW`](https://wiki.ubuntu.com/UncomplicatedFirewall)。

✅ 启用方式:把「ufw」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

其中一些命令包括:

* `allow <port>/<optional: protocol>` 添加一条允许(allow)规则
* `default` 设置默认策略
* `delete <port>/<optional: protocol>` 删除规则(RULE)
* `deny <port>/<optional: protocol>` 添加一条拒绝(deny)规则
* `disable` 关闭防火墙
* `enable` 启用防火墙
* `route` 添加路由(route)规则
