# Drush

本插件为 [Drush](https://www.drush.org) 提供一组别名和函数。Drush 是 Drupal 的命令行 shell
和 Unix 脚本接口。本插件还为 `drush` 命令提供补全。

✅ 启用方式:把「drush」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名    | 命令                                                        | 说明                                                                 |
| ------- | ----------------------------------------------------------- | -------------------------------------------------------------------- |
| `dr`    | `drush`                                                     | 显示 drush 帮助                                                      |
| `drca`  | `drush cc all`                                              | _(在 Drush 8 中已弃用)_ 清除所有 Drupal 缓存                         |
| `drcb`  | `drush cc block`                                            | _(在 Drush 8 中已弃用)_ 清除 block 缓存                              |
| `drcex` | `drush config:export -y`                                    | 把 Drupal 配置导出到一个目录                                         |
| `drcg`  | `drush cc registry`                                         | _(在 Drush 8 中已弃用)_ 清除 registry 缓存                           |
| `drcim` | `drush config:import -y`                                    | 从配置目录导入配置                                                   |
| `drcj`  | `drush cc css-js`                                           | 清除 css-js 缓存                                                     |
| `drcm`  | `drush cc menu`                                             | 清除 menu 缓存                                                       |
| `drcml` | `drush cc module-list`                                      | 清除 module-list 缓存                                                |
| `drcr`  | `drush core-cron`                                           | 为指定站点运行所有启用模块中的全部 cron 钩子                         |
| `drct`  | `drush cc theme-registry`                                   | 清除 theme-registry 缓存                                             |
| `drcv`  | `drush cc views`                                            | 清除 views 缓存 _(请确保 views 模块已启用)_                          |
| `drdmp` | `drush drush sql-dump --ordered-dump --result-file=dumpsql` | 把数据库备份到一个新的 dump.sql 文件                                 |
| `drf`   | `drush features`                                            | 显示 features 状态                                                   |
| `drfr`  | `drush features-revert -y`                                  | 回滚站点上的一个 feature 模块                                        |
| `drfra` | `drush features-revert-all`                                 | 回滚站点上所有已启用的 feature 模块                                  |
| `drfu`  | `drush features-update -y`                                  | 更新站点上的一个 feature 模块                                        |
| `drif`  | `drush image-flush --all`                                   | 清空所有派生图像                                                     |
| `drpm`  | `drush pm-list --type=module`                               | 显示可用模块列表                                                     |
| `drst`  | `drush core-status`                                         | 概览当前 Drupal 安装(如果存在)的整体状况                           |
| `druli` | `drush user:login`                                          | 为用户 ID 1 或其他用户显示一次性登录链接                             |
| `drup`  | `drush updatedb`                                            | 执行所需的全部数据库更新(如同运行 update.php)                       |
| `drups` | `drush updatedb-status`                                     | 列出待执行的数据库更新                                               |
| `drv`   | `drush version`                                             | 显示 drush 版本                                                      |
| `drvd`  | `drush variable-del`                                        | 删除一个变量                                                         |
| `drvg`  | `drush variable-get`                                        | 获取部分或全部站点变量及其取值的列表                                 |
| `drvs`  | `drush variable-set`                                        | 设置一个变量                                                         |
| `drws`  | `drush watchdog:show`                                       | 显示 watchdog 消息                                                   |
| `drwse` | `drush watchdog:show --extended`                            | 显示 watchdog 消息及其扩展信息                                       |
| `drwst` | `drush watchdog:tail`                                       | 实时跟踪(tail)watchdog 消息                                        |

## 函数

- `dren`:下载并启用一个或多个扩展(模块或主题)。调用时必须带一个或多个参数,
  例如:`dren devel` 或 `dren devel module_filter views`。

- `drf`:编辑 drushrc、站点别名(site alias)和 Drupal 的 settings.php 文件。
  调用时可带一个参数或不带参数,例如:`drf 1`。

- `dris`:禁用一个或多个扩展(模块或主题)。调用时必须带
  一个或多个参数,例如:`dris devel` 或 `dris devel module_filter views`。

- `drpu`:卸载一个或多个模块。调用时必须带一个或多个
  参数,例如:`drpu devel` 或 `drpu devel module_filter views`。

- `drnew`:创建一个全新的 Drupal 网站。注意:安装一完成,
  `drush` 就会把用户名和一个随机密码打印到终端:

  ```text
  Installation complete.  User name: admin  User password: cf7t8yqNEm
  ```
