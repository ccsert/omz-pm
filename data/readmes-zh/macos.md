# macOS 插件

本插件提供了一些实用工具,让 macOS(旧称 OSX)上的使用体验更加愉快。

✅ 启用方式:把「macos」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 支持的终端

- [iTerm](https://iterm.sourceforge.net/)
- [iTerm2](https://iterm2.com/)
- [Hyper](https://hyper.is/)
- [Tabby](https://tabby.sh/)
- [Ghostty](https://ghostty.org)

## 命令

| 命令          | 说明                                                     |
| :------------ | :------------------------------------------------------- |
| `tab`         | 在新标签页中打开当前目录                                 |
| `split_tab`   | 水平拆分当前终端标签页                                   |
| `vsplit_tab`  | 垂直拆分当前终端标签页                                   |
| `ofd`         | 在 Finder 中打开传入的目录(默认为 $PWD)                |
| `pfd`         | 返回最前面的 Finder 窗口的路径                           |
| `pfs`         | 返回 Finder 中当前选中的内容                             |
| `cdf`         | `cd` 到 Finder 当前所在的目录                            |
| `pushdf`      | `pushd` 到 Finder 当前所在的目录                         |
| `pxd`         | 返回当前 Xcode 项目的目录                                |
| `cdx`         | `cd` 到当前 Xcode 项目的目录                             |
| `quick-look`  | 对指定文件进行快速预览(Quick-Look)                     |
| `man-preview` | 在「预览」(Preview)应用中打开 man 手册页                |
| `showfiles`   | 在 Finder 中显示隐藏文件                                 |
| `hidefiles`   | 在 Finder 中隐藏隐藏文件                                 |
| `itunes`      | _已弃用_。从 macOS Catalina 起,请改用 `music`           |
| `music`       | 控制 Apple Music。使用 `music -h` 查看用法详情           |
| `spotify`     | 控制 Spotify,并可按艺术家、专辑、曲目等搜索…            |
| `rmdsstore`   | 在目录中递归删除 .DS_Store 文件                          |
| `btrestart`   | 重启蓝牙守护进程                                         |
| `freespace`   | 将所选磁盘上可清除的磁盘空间用 0 覆写                    |

## 致谢

原作者:[Sorin Ionescu](https://github.com/sorin-ionescu)

本应用使用了以下第三方脚本:

[shpotify](https://github.com/hnarayanan/shpotify)

版权所有 (c) 2012–2019 [Harish Narayanan](https://harishnarayanan.org/)。

特此免费向任何获得本软件及相关文档文件(下称
「软件」)副本之人授予许可,允许其不受限制地处理本软件,包括但不限于
使用、复制、修改、合并、发布、分发、再许可和/或销售本软件的副本,
并允许获得本软件之人如此行事,但须满足以下条件:

上述版权声明及本许可声明应包含在本软件的所有副本或主要部分中。

本软件按「原样」提供,不附带任何形式的保证,无论明示
还是默示,包括但不限于对适销性、
特定用途适用性以及不侵权的保证。在任何情况下,作者或
版权持有人均不对任何索赔、损害或其他
责任负责,无论是因合同诉讼、侵权诉讼还是其他诉讼引起,
也无论该责任源于本软件或与本软件相关,还是源于
使用本软件或以其他方式处理本软件而产生。
