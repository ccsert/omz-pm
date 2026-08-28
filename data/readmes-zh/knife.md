# knife 插件

本插件为 [knife](https://docs.chef.io/knife.html) 提供补全。knife 是一个命令行工具,
用于与 [Chef](https://chef.io) 交互——Chef 是一个通过代码实现基础架构自动化与管理的平台。

✅ 启用方式:把「knife」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 选项

- `KNIFE_RELATIVE_PATH`:如果设为 `true`,补全脚本会在 chef 根目录下的 `cookbooks`
  文件夹中查找本地 cookbook。它的优先级高于下面另外两个选项。**默认值:** 空。

- `KNIFE_COOKBOOK_PATH`:如果设置,它指向包含本地 cookbook 的文件夹,
  例如:`/path/to/my/chef/cookbooks`。**默认值:** `knife.rb` 中的 `cookbook_path`
  字段(见下文)。

- `KNIFE_CONF_PATH`:指向 `knife.rb` 配置文件的变量,例如
  `/path/to/my/.chef/knife.rb`。仅在未设置 `$KNIFE_COOKBOOK_PATH` 时使用。
  如果 `$PWD/.chef/knife.rb` 存在,则会改用它;否则,若设置了该变量,就使用它的值。
  **默认值**: `$HOME/.chef/knife.rb`。
