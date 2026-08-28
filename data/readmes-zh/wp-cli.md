# WP-CLI

[WordPress CLI](https://wp-cli.org/) 是一个用于管理 WordPress 站点的命令行工具。
你无需打开浏览器,即可更新插件、配置多站点(multisite)安装,以及完成更多操作。

本插件为 `wp-cli` 提供 [Tab 补全](https://wp-cli.org/#tab-completions),
并为常用命令提供了一些别名。

✅ 启用方式:把「wp-cli」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

**维护者:**[joshmedeski](https://github.com/joshmedeski)

## 别名

`wp-cli` 命令的完整列表见此处:https://developer.wordpress.org/cli/commands/

| 别名      | 命令                        |
|-----------|-----------------------------|
| **核心**                                |
| `wpcc`    | `wp core config`            |
| `wpcd`    | `wp core download`          |
| `wpci`    | `wp core install`           |
| `wpcii`   | `wp core is-installed`      |
| `wpcmc`   | `wp core multisite-convert` |
| `wpcmi`   | `wp core multisite-install` |
| `wpcu`    | `wp core update`            |
| `wpcudb`  | `wp core update-db`         |
| `wpcvc`   | `wp core verify-checksums`  |
| **Cron**                                |
| `wpcre`   | `wp cron event`             |
| `wpcrs`   | `wp cron schedule`          |
| `wpcrt`   | `wp cron test`              |
| **数据库**                              |
| `wpdbe`   | `wp db export`              |
| `wpdbi`   | `wp db import`              |
| `wpdbcr`  | `wp db create`              |
| `wpdbs`   | `wp db search`              |
| `wpdbch`  | `wp db check`               |
| `wpdbr`   | `wp db repair`              |
| **菜单**                                |
| `wpmc`    | `wp menu create`            |
| `wpmd`    | `wp menu delete`            |
| `wpmi`    | `wp menu item`              |
| `wpml`    | `wp menu list`              |
| `wpmlo`   | `wp menu location`          |
| **插件**                                |
| `wppa`    | `wp plugin activate`        |
| `wppda`   | `wp plugin deactivate`      |
| `wppd`    | `wp plugin delete`          |
| `wppg`    | `wp plugin get`             |
| `wppi`    | `wp plugin install`         |
| `wppis`   | `wp plugin is-installed`    |
| `wppl`    | `wp plugin list`            |
| `wppp`    | `wp plugin path`            |
| `wpps`    | `wp plugin search`          |
| `wppst`   | `wp plugin status`          |
| `wppt`    | `wp plugin toggle`          |
| `wppun`   | `wp plugin uninstall`       |
| `wppu`    | `wp plugin update`          |
| **文章**                                |
| `wppoc`   | `wp post create`            |
| `wppod`   | `wp post delete`            |
| `wppoe`   | `wp post edit`              |
| `wppogen` | `wp post generate`          |
| `wppog`   | `wp post get`               |
| `wppol`   | `wp post list`              |
| `wppom`   | `wp post meta`              |
| `wppou`   | `wp post update`            |
| `wppourl` | `wp post url`               |
| **边栏**                                |
| `wpsbl`   | `wp sidebar list`           |
| **主题**                                |
| `wpta`    | `wp theme activate`         |
| `wptd`    | `wp theme delete`           |
| `wptdis`  | `wp theme disable`          |
| `wpte`    | `wp theme enable`           |
| `wptg`    | `wp theme get`              |
| `wpti`    | `wp theme install`          |
| `wptis`   | `wp theme is-installed`     |
| `wptl`    | `wp theme list`             |
| `wptm`    | `wp theme mod`              |
| `wptp`    | `wp theme path`             |
| `wpts`    | `wp theme search`           |
| `wptst`   | `wp theme status`           |
| `wptu`    | `wp theme update`           |
| **用户**                                |
| `wpuac`   | `wp user add-cap`           |
| `wpuar`   | `wp user add-role`          |
| `wpuc`    | `wp user create`            |
| `wpud`    | `wp user delete`            |
| `wpugen`  | `wp user generate`          |
| `wpug`    | `wp user get`               |
| `wpui`    | `wp user import-csv`        |
| `wpul`    | `wp user list`              |
| `wpulc`   | `wp user list-caps`         |
| `wpum`    | `wp user meta`              |
| `wpurc`   | `wp user remove-cap`        |
| `wpurr`   | `wp user remove-role`       |
| `wpusr`   | `wp user set-role`          |
| `wpuu`    | `wp user update`            |
| **小工具**                              |
| `wpwa`    | `wp widget add`             |
| `wpwda`   | `wp widget deactivate`      |
| `wpwd`    | `wp widget delete`          |
| `wpwl`    | `wp widget list`            |
| `wpwm`    | `wp widget move`            |
| `wpwu`    | `wp widget update`          |
