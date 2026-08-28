# profiles 插件

本插件让你可以依据完整主机名(含域名)为 zsh 创建各自独立的配置文件。

✅ 启用方式:把「profiles」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

它会读取你的 `$HOST` 变量,并在 `$ZSH_CUSTOM/profiles/` 目录中查找按域名的各部分命名的文件。

例如,对于 `HOST=host.domain.com`,它会尝试按以下顺序加载下列文件:

```text
$ZSH_CUSTOM/profiles/com
$ZSH_CUSTOM/profiles/domain.com
$ZSH_CUSTOM/profiles/host.domain.com
```

这意味着如果这些文件之间存在相互冲突的设置,生效的将是最后应用的那个,
也就是 host.domain.com 里的设置。
