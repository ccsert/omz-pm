# Systemd 插件

systemd 插件为 systemd 提供了许多实用的别名。

✅ 启用方式:把「systemd」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名 | 命令 | 说明 |
|:-----|:-----|:-----|
| `sc-failed` | `systemctl --failed` | 列出失败的 systemd 单元 |
| `sc-list-units` | `systemctl list-units` | 列出 systemd 内存中的所有单元 |
| `sc-is-active` | `systemctl is-active` | 显示某个单元是否处于活动状态 |
| `sc-status` | `systemctl status` | 显示一个或多个单元的简要运行时状态信息 |
| `sc-show` | `systemctl show` | 显示单元、任务或管理器本身的属性 |
| `sc-help` | `systemctl help` | 显示单元的 man 手册页 |
| `sc-list-unit-files` | `systemctl list-unit-files` | 列出系统中已安装的 unit 文件 |
| `sc-is-enabled` | `systemctl is-enabled` | 检查指定的 unit 文件中是否有已启用的 |
| `sc-list-jobs` | `systemctl list-jobs` | 列出正在进行的任务 |
| `sc-show-environment` | `systemctl show-environment` | 输出 systemd 管理器的环境块 |
| `sc-cat` | `systemctl cat` | 显示一个或多个单元的底层文件 |
| `sc-list-timers` | `systemctl list-timers` | 列出当前内存中的 timer 单元 |
| **带 sudo 的别名** |||
| `sc-start` | `sudo systemctl start` | 启动单元(可多个) |
| `sc-stop` | `sudo systemctl stop` | 停止单元(可多个) |
| `sc-reload` | `sudo systemctl reload` | 重载单元(可多个) |
| `sc-restart` | `sudo systemctl restart` | 重启单元(可多个) |
| `sc-try-restart` | `sudo systemctl try-restart` | 重启单元(可多个) |
| `sc-isolate` | `sudo systemctl isolate` | 启动某个单元及其依赖,并停止所有其他单元 |
| `sc-kill` | `sudo systemctl kill` | 杀死单元(可多个) |
| `sc-reset-failed` | `sudo systemctl reset-failed` | 重置指定单元的 "failed" 状态, |
| `sc-enable` | `sudo systemctl enable` | 启用单元(可多个) |
| `sc-disable` | `sudo systemctl disable` | 禁用单元(可多个) |
| `sc-reenable` | `sudo systemctl reenable` | 重新启用单元(可多个) |
| `sc-preset` | `sudo systemctl preset` | 重置一个或多个 unit 文件的启用/禁用状态 |
| `sc-mask` | `sudo systemctl mask` | 屏蔽单元(可多个) |
| `sc-unmask` | `sudo systemctl unmask` | 取消屏蔽单元(可多个) |
| `sc-link` | `sudo systemctl link` | 把 unit 文件链接到 unit 文件搜索路径中 |
| `sc-load` | `sudo systemctl load` | 加载单元(可多个) |
| `sc-cancel` | `sudo systemctl cancel` | 取消任务(可多个) |
| `sc-set-environment` | `sudo systemctl set-environment` | 设置一个或多个 systemd 管理器环境变量 |
| `sc-unset-environment` | `sudo systemctl unset-environment` | 取消设置一个或多个 systemd 管理器环境变量 |
| `sc-edit` | `sudo systemctl edit` | 编辑 drop-in 片段,或用 `--full` 编辑完整的替换文件 |
| `sc-enable-now` | `sudo systemctl enable --now` | 启用并启动单元(可多个) |
| `sc-disable-now` | `sudo systemctl disable --now` | 禁用并停止单元(可多个) |
| `sc-mask-now` | `sudo systemctl mask --now` | 屏蔽并停止单元(可多个) |

### 用户别名

把前缀 `sc` 换成 `scu`,就能以 `--user` 方式使用上面的别名。
例如:`scu-list-units` 会被别名为 `systemctl --user list-units`。

### 单元状态提示符

你可以仿照 gitfast 插件的做法,在提示符中添加一个 token。要把该 token 加入提示符,只需把 `$(systemd_prompt_info [unit]...)` 放进你的提示符(可以指定多个单元)。

对每个 `$unit`,插件会在你的提示符中添加如下内容:

```text
<prefix><unit>:<active|notactive><suffix>
```

你可以用以下变量控制这些部分:

- `<prefix>`:设置 `$ZSH_THEME_SYSTEMD_PROMPT_PREFIX`。

- `<suffix>`:设置 `$ZSH_THEME_SYSTEMD_PROMPT_SUFFIX`。

- `<unit>`:作为参数传给函数的名称。如果你想让它全部大写,可以把变量 `$ZSH_THEME_SYSTEMD_PROMPT_CAPS` 设置为一个非空字符串。

- `<active>`:在 systemd 单元处于活动状态时显示。
  设置 `$ZSH_THEME_SYSTEMD_PROMPT_ACTIVE`。

- `<notactive>`:在 systemd 单元*不*处于活动状态时显示。
  设置 `$ZSH_THEME_SYSTEMD_PROMPT_NOTACTIVE`。

例如,如果你的提示符包含 `PROMPT='$(systemd_prompt_info dhcpd httpd)'`,并设置了以下变量:

```sh
ZSH_THEME_SYSTEMD_PROMPT_PREFIX="["
ZSH_THEME_SYSTEMD_PROMPT_SUFFIX="]"
ZSH_THEME_SYSTEMD_PROMPT_ACTIVE="+"
ZSH_THEME_SYSTEMD_PROMPT_NOTACTIVE="X"
ZSH_THEME_SYSTEMD_PROMPT_CAPS=1
```

如果 `dhcpd` 正在运行,而 `httpd` 没有,那么你的提示符将如下所示:

```text
[DHCPD: +][HTTPD: X]
```
