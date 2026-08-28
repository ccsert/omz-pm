# Gem 插件

本插件为 [Gem](https://rubygems.org/) 提供补全和别名。补全涵盖常用的 `gem` 子命令,
以及当前目录下已安装的 gems。

✅ 启用方式:把「gem」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名                 | 命令                          | 说明                                       |
|----------------------|-------------------------------|--------------------------------------------|
| gemb                 | `gem build *.gemspec`         | 从 gemspec 构建 gem                        |
| gemp                 | `gem push *.gem`              | 把 gem 推送到 gem 服务器                   |
| gemy [gem] [version] | `gem yank [gem] -v [version]` | 从索引中移除已推送的某个 gem 版本          |
