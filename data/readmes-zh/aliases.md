# Aliases 速查表

在安装了大量出色的第三方别名之后,这个插件可以帮你列出当前可用的快捷方式——根据你已启用的插件来生成。

✅ 启用方式:把「aliases」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

依赖要求:需要安装 Python。

**维护者:** [@hqingyi](https://github.com/hqingyi)

## 用法

- `als`:按分组显示所有别名

- `als -h/--help`:打印帮助信息

- `als <keyword(s)>`:按 `<keyword>` 过滤并高亮别名

- `als -g <group>/--group <group>`:只显示分组 `<group>` 的别名。多次使用该标志则显示所有分组

- `als --groups`:只显示分组名称

  ![screenshot](https://github.com/ohmyzsh/ohmyzsh/assets/66907184/5bfa00ea-5fc3-4e97-8b22-2f74f6b948c7)
