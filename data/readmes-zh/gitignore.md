# gitignore

本插件让你可以在命令行中使用 [gitignore.io](https://www.gitignore.io)。拉取模板需要可用的
网络连接。插件使用 gitignore.io 的 CDN 端点,以简化访问并提升可靠性。

✅ 启用方式:把「gitignore」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 插件命令

* `gi list`:列出当前支持的全部 gitignore.io 模板。

* `gi [TEMPLATENAME]`:在命令行上显示 git-ignore 输出,例如 `gi java` 用于排除 class 和
  package 文件。

* `gi [TEMPLATENAME] >> .gitignore`:把模板规则追加到你项目的 `.gitignore` 文件中。
