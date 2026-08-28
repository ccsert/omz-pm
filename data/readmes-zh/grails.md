# Grails 插件

本插件为 [Grails 2 CLI](https://grails.github.io/grails2-doc/2.5.x/guide/commandLine.html) 提供补全。

✅ 启用方式:把「grails」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

它会在以下路径中查找脚本:

- `$GRAILS_HOME/scripts`
- `~/.grails/scripts`
- `./scripts`
- `./plugins/*/scripts`
