# knife_ssh 插件

本插件添加了一个 `knife_ssh` 函数及其自动补全,用于通过 ssh 连接到由
[Chef](https://www.chef.io/) 管理的服务器。

✅ 启用方式:把「knife_ssh」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

首次触发 knife_ssh 补全时,本插件会通过 `knife` 创建 Chef 节点列表的缓存,
并把它存储在 `$HOME/.knife_comp~` 中。

**依赖要求:**必须已安装 `knife`。
