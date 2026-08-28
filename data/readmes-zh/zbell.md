# zbell 插件

本插件会在一条命令结束运行且其运行时长超过指定阈值时,输出一个响铃字符(bell)。

✅ 启用方式:把「zbell」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 设置

这些设置需要写进你的 zshrc 文件,并且要在 source Oh My Zsh 之前。

- `zbell_duration`:以秒为单位的时长,命令运行超过该时长就考虑提示其已结束。默认:15 秒。

- `zbell_ignore`:如果有些程序你明知会长时间运行、不希望它在结束后响铃,
  就把它们加入 `zbell_ignore` 数组。默认忽略 `$EDITOR` 和 `$PAGER`:

  ```zsh
  zbell_ignore=($EDITOR $PAGER)
  ```

- `zbell_use_notify_send`:如果设为 `true`,则(在可用的情况下)使用 `notify-send` 工具
  在屏幕上显示弹窗。默认:`true`(启用)。

## 作者

改编自 [Jean-Philippe Ouellet](https://github.com/jpouellet) 的原始版本。
以 ISC 许可证发布。
