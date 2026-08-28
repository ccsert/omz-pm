# nodenv 插件

本插件的主要作用是提供 `nodenv_prompt_info`,你可以把它加进主题,让提示符显示 Node
版本信息。

✅ 启用方式:把「nodenv」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 函数

* `nodenv_prompt_info`:显示 nodenv 正在使用的 Node 版本;如果没找到 nodenv,则显示全局
  Node 版本。你可以在提示符中使用这个函数,只需把
  `$(nodenv_prompt_info)` 加到 PROMPT 或 RPROMPT:

  ```zsh
  RPROMPT='$(nodenv_prompt_info)'
  ```
