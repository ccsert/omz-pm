# ssh 插件

本插件基于你的 `~/.ssh/config` 文件提供主机名补全,并添加了一些处理 SSH 密钥的实用函数。

✅ 启用方式:把「ssh」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 函数

- `ssh_rmhkey`:根据 `.ssh/config` 中的主机条目名称,从 known hosts 中移除对应的主机密钥。
- `ssh_load_key`:把 SSH 密钥加载进 agent。
- `ssh_unload_key`:把 SSH 密钥从 agent 中移除。
