# zoxide 插件

初始化 [zoxide](https://github.com/ajeetdsouza/zoxide),一个更聪明的终端 cd
命令。

![Tutorial](https://raw.githubusercontent.com/ajeetdsouza/zoxide/97dc08347d9dbf5b5a4516b79e0ac27366b962ce/contrib/tutorial.webp)

✅ 启用方式:把「zoxide」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 覆盖 `z` 别名

你可以设置 `ZOXIDE_CMD_OVERRIDE`,它会被传给 `zoxide init` 的 `--cmd` 标志。这让你可以把
`z` 命令默认改成 `cd`。

**注意:** 你必须先[安装 zoxide](https://github.com/ajeetdsouza/zoxide#step-1-install-zoxide)。
