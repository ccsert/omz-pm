# jenv 插件

[jenv](https://www.jenv.be/) 是一个 Java 版本管理器,类似于 [rbenv](https://github.com/rbenv/rbenv)
和 [pyenv](https://github.com/yyuu/pyenv)。

本插件会初始化 jenv,并提供 `jenv_prompt_info` 函数,用于把 Java 版本信息加进提示符。

✅ 启用方式:把「jenv」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 主题示例

你可以修改 `$PROMPT` 或 `$RPROMPT` 变量来调用 `jenv_prompt_info`。

例如:
```
PROMPT="%~$ "
RPROMPT='$(jenv_prompt_info)'
```
会把提示符变成:
```
~/java/project$ ▋                                       oracle64-1.6.0.39
```
