# composer

本插件为 [composer](https://getcomposer.org/) 提供补全,以及常用 composer 命令的别名。
它还会把 Composer 的全局可执行文件加入 PATH——在 Composer 可用的情况下。

✅ 启用方式:把「composer」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

原作者:Daniel Gomes <me@danielcsgomes.com>

## 别名

| 别名   | 命令                               | 说明                                                                                    |
| ------ | ---------------------------------- | --------------------------------------------------------------------------------------- |
| `c`    | `composer`                         | 启动 composer                                                                           |
| `ccp`  | `composer create-project`          | 从现有的包创建新项目                                                                     |
| `cdo`  | `composer dump-autoload -o`        | 把 PSR-0/4 自动加载转换为 classmap,以获得更快的自动加载器(适合生产环境)                 |
| `cdu`  | `composer dump-autoload`           | 更新自动加载器                                                                           |
| `cget` | `curl -s <installer> \| php`       | 在当前目录安装 composer                                                                  |
| `cgr`  | `composer global require`          | 允许 require 命令在 COMPOSER_HOME 目录下运行                                             |
| `cgrm` | `composer global remove`           | 允许 remove 命令在 COMPOSER_HOME 目录下运行                                              |
| `cgu`  | `composer global update`           | 允许 update 命令在 COMPOSER_HOME 目录下运行                                              |
| `ci`   | `composer install`                 | 从 `composer.json` 解析并安装依赖                                                        |
| `co`   | `composer outdated`                | 显示已安装且有可用更新的包列表                                                           |
| `cod`  | `composer outdated --direct`       | 显示已安装且有可用更新的直接依赖包列表                                                    |
| `cr`   | `composer require`                 | 向 `composer.json` 添加新包                                                              |
| `crm`  | `composer remove`                  | 从 `composer.json` 移除包                                                                |
| `cs`   | `composer show`                    | 列出可用的包,可按需过滤                                                                  |
| `csu`  | `composer self-update`             | 把 composer 更新到最新版本                                                               |
| `cu`   | `composer update`                  | 更新 composer 依赖和 `composer.lock` 文件                                                 |
| `cuh`  | `composer update -d <config-home>` | 更新全局安装的包                                                                         |
