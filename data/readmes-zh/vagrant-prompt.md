# vagrant-prompt

本插件在提示符中显示 Vagrant 虚拟机的状态。它同样支持单主机和多主机配置。

✅ 启用方式:把「vagrant-prompt」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

**Alberto Re <alberto.re@gmail.com>**

## 用法

要在提示符中显示 Vagrant 信息,请把 `vagrant_prompt_info` 加入主题中的
`$PROMPT` 或 `$RPROMPT` 变量。示例:

```zsh
PROMPT="$PROMPT"' $(vagrant_prompt_info)'
# or
RPROMPT='$(vagrant_prompt_info)'
```

### 自定义

`vagrant_prompt_info` 使用以下自定义变量,可以在你的 `.zshrc` 文件中设置:

```zsh
ZSH_THEME_VAGRANT_PROMPT_PREFIX="%{$fg_bold[blue]%}["
ZSH_THEME_VAGRANT_PROMPT_SUFFIX="%{$fg_bold[blue]%}]%{$reset_color%} "
ZSH_THEME_VAGRANT_PROMPT_RUNNING="%{$fg_no_bold[green]%}●"
ZSH_THEME_VAGRANT_PROMPT_POWEROFF="%{$fg_no_bold[red]%}●"
ZSH_THEME_VAGRANT_PROMPT_SUSPENDED="%{$fg_no_bold[yellow]%}●"
ZSH_THEME_VAGRANT_PROMPT_NOT_CREATED="%{$fg_no_bold[white]%}○"
```

### 状态与变量的对应关系

插件使用 `vagrant status` 报告的输出,按照下表打印匹配的符号:

| 状态        | 符号                                   |
| ----------- | -------------------------------------- |
| running     | `ZSH_THEME_VAGRANT_PROMPT_RUNNING`     |
| not running | `ZSH_THEME_VAGRANT_PROMPT_POWEROFF`    |
| poweroff    | `ZSH_THEME_VAGRANT_PROMPT_POWEROFF`    |
| paused      | `ZSH_THEME_VAGRANT_PROMPT_SUSPENDED`   |
| saved       | `ZSH_THEME_VAGRANT_PROMPT_SUSPENDED`   |
| suspended   | `ZSH_THEME_VAGRANT_PROMPT_SUSPENDED`   |
| not created | `ZSH_THEME_VAGRANT_PROMPT_NOT_CREATED` |
