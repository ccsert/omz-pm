# toolbox 插件

[toolbox](https://containertoolbx.org) 插件。toolbox 是一个使用容器化 CLI 环境的工具。

✅ 启用方式:把「toolbox」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 提示符函数

本插件添加了 `toolbox_prompt_info()` 函数。把它用在提示符中,当你正运行在 toolbox 容器里时
会显示 toolbox 指示符 ⬢,否则什么都不显示。

把 `$(toolbox_prompt_info)` 加进你的 `PROMPT` 或 `RPROMPT` 变量即可使用:

```zsh
RPROMPT='$(toolbox_prompt_info)'
```

同样地,插件还添加了 `toolbox_prompt_name()`,用于显示容器化环境的名称。

## 别名

| 别名  | 命令            | 说明                                 |
| ----- | --------------- | ------------------------------------ |
| tbe   | `toolbox enter` | 进入 toolbox 环境                    |
| tbr   | `toolbox run`   | 在已有的 toolbox 中运行一条命令      |
