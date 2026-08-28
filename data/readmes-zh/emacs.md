# Emacs 插件

本插件利用了 Emacs 的 daemon(守护进程)能力,让用户可以快速打开 frame——无论是通过 ssh 连接在终端里打开,还是在同一台主机上打开 X frame。插件还为这些操作提供了一些别名。

- 你再也不必承担每次都启动 Emacs 的开销
- 打开文件非常快,因为 Emacs 不用再分心做其他事情。
- 你可以在多个已打开的 frame 之间共享已打开的缓冲区。
- 在运行时所做的配置更改会应用到所有 frame。

**注意:** 需要 Emacs 24 及更新版本。

✅ 启用方式:把「emacs」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

本插件使用一个自定义启动器(下文称之为 `$EMACS_LAUNCHER`),它只是 [`emacsclient`](https://www.emacswiki.org/emacs/EmacsClient) 的一个封装。

| 别名   | 命令                                               | 说明                                                           |
|--------|----------------------------------------------------|----------------------------------------------------------------|
| emacs  | `$EMACS_LAUNCHER --no-wait`                        | 打开一个临时的 emacsclient frame                               |
| e      | `emacs`                                            | 与 emacs 别名相同                                              |
| te     | `$EMACS_LAUNCHER -nw`                              | 打开终端版 emacsclient                                         |
| eeval  | `$EMACS_LAUNCHER --eval`                           | 相当于 `M-x eval`,但在 Emacs 外部执行                          |
| eframe | `emacsclient --alternate-editor="" --create-frame` | 创建新的 X frame                                               |
| efile  | -                                                  | 打印当前缓冲区所打开文件的路径                                 |
| ecd    | -                                                  | 打印当前缓冲区所打开文件所在的目录                             |
