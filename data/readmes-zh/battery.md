# Battery 插件

本插件提供了一些函数,可用于在你的自定义主题中显示电池信息。

✅ 启用方式:把「battery」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

然后,把 `battery_pct_prompt` 函数加到你的自定义主题里。例如:

```zsh
RPROMPT='$(battery_pct_prompt) ...'
```

此外,你还可以把 `BATTERY_CHARGING` 变量设置成自己喜欢的样子。
例如:

```zsh
BATTERY_CHARGING="⚡️"
```

你可以用下面的设置查看充电器的功率(仅限 MacOS)

```zsh
BATTERY_SHOW_WATTS=true
```

## 依赖要求

- 在 Linux 上,你的操作系统必须安装 `acpi` 或 `acpitool` 命令。
  在 Debian/Ubuntu 上,可以用 `sudo apt install acpi` 或 `sudo apt install acpitool` 来安装。

- 在 Android 上(通过 [Termux](https://play.google.com/store/apps/details?id=com.termux)),你必须:

  1. 安装 `Termux:API` 附加应用:
     [Google Play](https://play.google.com/store/apps/details?id=com.termux.api) | [F-Droid](https://f-droid.org/packages/com.termux.api/)

  2. 在 termux 中安装 `termux-api` 包:

     ```sh
     pkg install termux-api
     ```
