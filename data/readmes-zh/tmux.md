# tmux

本插件为终端复用器 [tmux](https://tmux.github.io/) 提供一组别名。✅ 启用方式:把「tmux」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

本插件还支持以下功能:

- 检测 tmux 是否已安装,如果没有,会提示用户安装 tmux
- 检测终端是否支持 256 色,并设置相应的配置变量
- 设置要使用的正确本地配置文件

## 别名

| 别名       | 命令                       | 说明                                                      |
| ---------- | -------------------------- | -------------------------------------------------------- |
| `ta`       | tmux attach -t             | 把新的 tmux 会话连接到已在运行的指定命名会话                  |
| `tad`      | tmux attach -d -t          | 分离(detach)指定的 tmux 会话                              |
| `tds`      | `_tmux_directory_session`  | 为当前路径创建会话,或连接到该会话                            |
| `tkss`     | tmux kill-session -t       | 终止正在运行的指定命名 tmux 会话                              |
| `tksv`     | tmux kill-server           | 终止所有正在运行的 tmux 会话                                 |
| `tl`       | tmux list-sessions         | 显示正在运行的 tmux 会话列表                                 |
| `to`       | tmux new-session -A -s     | 创建或连接到一个命名 tmux 会话                                |
| `tmux`     | `_zsh_tmux_plugin_run`     | 启动一个新的 tmux 会话                                       |
| `tmuxconf` | `$EDITOR $ZSH_TMUX_CONFIG` | 用编辑器打开 .tmux.conf 文件                                  |
| `ts`       | tmux new-session -s        | 创建一个新的命名 tmux 会话                                   |

## 配置变量

| 变量                                | 说明                                                                                                                           |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `ZSH_TMUX_AUTOREFRESH`              | 自动刷新全局环境(默认:`false`)                                                                                                |
| `ZSH_TMUX_AUTOSTART`                | 自动启动 tmux(默认:`false`)                                                                                                   |
| `ZSH_TMUX_AUTOSTART_ONCE`           | 仅在之前未启动过 tmux 时才自动启动(默认:`true`)                                                                                |
| `ZSH_TMUX_AUTOCONNECT`              | 如果存在之前的会话则自动连接(默认:`true`)                                                                                      |
| `ZSH_TMUX_AUTOQUIT`                 | tmux 退出后自动关闭终端(默认:`ZSH_TMUX_AUTOSTART`)                                                                             |
| `ZSH_TMUX_CONFIG`                   | 设置配置文件路径(默认:`$HOME/.tmux.conf`、`$XDG_CONFIG_HOME/tmux/tmux.conf`)                                                   |
| `ZSH_TMUX_DEFAULT_SESSION_NAME`     | 在启用自动启动时设置 tmux 的默认会话名                                                                                          |
| `ZSH_TMUX_AUTONAME_SESSION`         | 根据 `$PWD` 的 basename 自动为新会话命名(默认:`false`)                                                                          |
| `ZSH_TMUX_DETACHED`                 | 设置分离(detached)模式(默认:`false`)                                                                                          |
| `ZSH_TMUX_FIXTERM`                  | 根据当前终端的支持情况,决定是否把 `$TERM` 设为 256 色的 term                                                                    |
| `ZSH_TMUX_FIXTERM_WITHOUT_256COLOR` | 非 256 色终端使用的 `$TERM`(默认:可用时为 `tmux`,否则为 `screen`)                                                              |
| `ZSH_TMUX_FIXTERM_WITH_256COLOR`    | 256 色终端使用的 `$TERM`(默认:可用时为 `tmux-256color`,否则为 `screen-256color`)                                               |
| `ZSH_TMUX_ITERM2`                   | 为 [iTerm2 tmux 集成](https://iterm2.com/documentation-tmux-integration.html)设置 `-CC` 选项(默认:`false`)                      |
| `ZSH_TMUX_UNICODE`                  | 设置 `tmux -u` 选项以支持 unicode                                                                                               |
