# brew 插件

本插件为常用的 [brew](https://brew.sh) 命令添加了若干别名。

✅ 启用方式:把「brew」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## Shellenv

如果在 PATH 中找不到 `brew`,本插件会尝试在常见位置查找它,并执行 `brew shellenv` 来正确设置环境。为方便起见,若之前未曾定义,本插件还会导出 `HOMEBREW_PREFIX="$(brew --prefix)"`。

如果你把 `brew` 安装在了非常见位置,仍可在 source `oh-my-zsh.sh` 之前设置 `BREW_LOCATION` 变量,令其指向 `brew` 可执行文件,插件会据此完成环境设置。

### sbin 目录

如果 `$HOMEBREW_PREFIX/sbin` 目录存在且尚未加入 PATH,本插件也会把它加入 PATH。一些 Homebrew formula(如 `mtr`)会把可执行文件安装到 `sbin`,`brew doctor` 会检查这一点。这能保证 `bdr` 别名运行时不出现警告。

## 别名

| 别名 | 命令 | 说明 |
| ---- | ---- | ---- |
| `ba` | `brew autoremove` | 卸载不再需要的 formula。 |
| `bcfg` | `brew config` | 显示对调试有用的 Homebrew 与系统配置信息。 |
| `bci` | `brew info --cask` | 显示给定 cask 的信息。 |
| `bcin` | `brew install --cask` | 安装给定的 cask。 |
| `bcl` | `brew list --cask` | 列出已安装的 cask。 |
| `bcn` | `brew cleanup` | 执行清理。 |
| `bco` | `brew outdated --cask` | 报告所有已过时的 cask。 |
| `bcrin` | `brew reinstall --cask` | 重新安装给定的 cask。 |
| `bcubc` | `brew upgrade --cask && brew cleanup` | 升级过时的 cask,然后执行清理。 |
| `bcubo` | `brew update && brew outdated --cask` | 更新 Homebrew 数据,然后列出过时的 cask。 |
| `bcup` | `brew upgrade --cask` | 升级所有过时的 cask。 |
| `bdr` | `brew doctor` | 检查系统是否存在潜在问题。 |
| `bfu` | `brew upgrade --formula` | 仅升级 formula(不升级 cask)。 |
| `bi` | `brew install` | 安装一个 formula。 |
| `bih` | `brew install --HEAD` | 以 --HEAD 安装 formula |
| `bl` | `brew list` | 列出所有已安装的 formula。 |
| `bo` | `brew outdated` | 列出有更新版本可用的已安装 formula。 |
| `br` | `brew reinstall` | 重新安装一个 formula。 |
| `brewp` | `brew pin` | 固定指定的 formula,使其不被升级。 |
| `brews` | _函数_ | 列出已安装的叶子 formula 及其依赖,然后列出 cask。 |
| `brewsp` | `brew list --pinned` | 列出已固定的 formula,或显示给定 formula 的版本。 |
| `brh` | `brew reinstall --HEAD` | 以 --HEAD 重新安装 formula |
| `bs` | `brew search` | 对文本执行 cask token 与 formula 名称的子串搜索。 |
| `bsl` | `brew services list` | 列出所有正在运行的服务。 |
| `bsoff` | `brew services stop` | 停止服务,并注销其在登录(或开机)时启动。 |
| `bsoffa` | `bsoff --all` | 停止所有已启动的服务。 |
| `bson` | `brew services start` | 启动服务,并注册其在登录(或开机)时启动。 |
| `bsona` | `bson --all` | 启动所有已停止的服务。 |
| `bsr` | `brew services run` | 运行服务,但不注册其在登录(或开机)时启动。 |
| `bsra` | `bsr --all` | 运行所有已停止的服务。 |
| `bu` | `brew update` | 更新 brew 及所有已安装的 formula。 |
| `bubo` | `brew update && brew outdated` | 更新 Homebrew 数据,然后列出过时的 formula 和 cask。 |
| `bubu` | `bubo && bup` | 执行上面最后两个操作。 |
| `bugbc` | `brew upgrade --greedy && brew cleanup` | 升级过时的 formula 和 cask(贪婪模式),然后执行清理。 |
| `bup` | `brew upgrade` | 升级过时且未固定的 brew 包。 |
| `buz` | `brew uninstall --zap` | 删除与某个 cask 关联的所有文件。 |

## 补全

本插件会自动配置好 Homebrew 补全函数所需的路径,无需你手动操作。参见:https://docs.brew.sh/Shell-Completion#configuring-completions-in-zsh 。

自 Homebrew 1.0 发布起,官方决定把 zsh 补全作为 brew 安装的一部分一并提供,因此本插件不再附带补全,现在只包含 brew 别名。如果你发现 brew 补全不再生效,请确保你的 Homebrew 安装已完全更新至最新。
