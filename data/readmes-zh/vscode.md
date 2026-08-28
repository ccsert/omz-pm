# VS Code 插件

本插件提供一组实用别名,用于简化命令行与 VS Code、VSCodium 或 Cursor 之间的交互。

✅ 启用方式:把「vscode」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 依赖要求

本插件要求安装一款受支持的编辑器,且其可执行文件能在 `PATH` 中找到。

你可以安装以下任意一款:

- VS Code(`code`)
- VS Code Insiders(`code-insiders`)
- VSCodium(`codium`)
- Cursor(`cursor`)

### macOS

Linux 安装通常会把可执行文件加入 `PATH`,而 macOS 用户可能仍需手动完成:

[针对 VS Code 和 VS Code Insiders](https://code.visualstudio.com/docs/setup/mac#_launching-from-the-command-line),
用 `F1` 或 `Shift+Cmd+P` 打开命令面板,然后搜索以下命令:

> Shell Command: Install 'code' command in PATH

[针对 VSCodium](https://github.com/VSCodium/vscodium/blob/master/DOCS.md#how-do-i-open-vscodium-from-the-terminal),
用 `F1` 或 `Shift+Cmd+P` 打开命令面板,然后搜索以下命令:

> Shell Command: Install 'codium' command in PATH

对于 Cursor,用 `F1` 或 `Cmd+Shift+P` 打开命令面板,然后搜索以下命令:

> Shell Command: Install 'cursor' command in PATH

## 选择编辑器

如果你安装了多款受支持的编辑器,例如 VS Code(稳定版)和 VS Code Insiders,你可以手动指定插件使用哪个可执行文件。把下面这一行加入 `~/.zshrc`,放在 `ZSH_THEME` 和 `plugins=()` 两行之间。这样插件就会使用你手动定义的可执行文件。

```zsh
ZSH_THEME=...

# Choose one of `code`, `code-insiders`, `codium`, or `cursor`.
# The following line makes the plugin open VS Code Insiders.
# Invalid entries are ignored and no aliases are added.
VSCODE=code-insiders

plugins=(... vscode)

source $ZSH/oh-my-zsh.sh
```

## 常用别名

| 别名 | 命令 | 说明 |
| ---- | ---- | ---- |
| vsc | code . | 在 VS Code 中打开当前文件夹 |
| vsc `[args ...]` | code `[args ...]` | 把参数透传给 VS Code,例如文件、文件夹或 CLI 标志。 |
| vsca `dir` | code --add `dir` | 把一个或多个文件夹添加到最后活动的窗口。 |
| vscd `file` `file` | code --diff `file` `file` | 将两个文件相互比较。 |
| vscg `file:line[:char]` | code --goto `file:line[:char]` | 在指定行和字符位置打开文件。 |
| vscn | code --new-window | 强制在新窗口中打开。 |
| vscr | code --reuse-window | 强制在最后活动的窗口中打开文件或文件夹。 |
| vscw | code --wait | 等待文件关闭后再返回。 |
| vscu `dir` | code --user-data-dir `dir` | 指定存储用户数据的目录。可用于打开多个互不相同的 Code 实例。 |
| vscp `profile` | code --profile `profile` | 指定打开 Code 时使用的 profile(配置档案)。 |

## 扩展别名

| 别名 | 命令 | 说明 |
| ---- | ---- | ---- |
| vsced `dir` | code --extensions-dir `dir` | 设置扩展的根目录。 |
| vscie `ext-id or vsix-path` | code --install-extension `ext-id or vsix-path` | 安装或更新扩展。 |
| vscue `ext-id` | code --uninstall-extension `ext-id` | 卸载扩展。 |

## 其他选项

| 别名 | 命令 | 说明 |
| ---- | ---- | ---- |
| vscv | code --verbose | 打印详细输出(隐含 `--wait`)。 |
| vscl `level` | code --log `level` | 要使用的日志级别。默认为 `info`。 |
| vscde | code --disable-extensions | 禁用所有已安装的扩展。 |
