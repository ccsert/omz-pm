# JetBrains Qodana CLI 插件

本插件为 [JetBrains Qodana CLI](https://github.com/JetBrains/qodana-cli) 提供自动补全。

✅ 启用方式:把「qodana」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

本插件不添加任何别名。

## 缓存

本插件会缓存补全脚本,并在插件加载时自动更新——通常就是启动新终端模拟器的时候。

缓存存储于:

- `$ZSH_CACHE_DIR/completions/_qodana` 补全脚本
