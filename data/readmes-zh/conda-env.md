# conda-env

本插件显示已创建的 conda 虚拟容器的信息,并支持后台主题化。

✅ 启用方式:把「conda-env」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

插件会创建一个 `conda_prompt_info` 函数供你在主题中使用,它会显示当前
`$CONDA_DEFAULT_ENV` 的基本名(basename)。

你可以在主题里使用这个提示符函数,把它加入 `PROMPT` 或 `RPROMPT` 变量即可。更多信息见 [示例](#example)。

## 设置

它使用两个变量来控制信息的显示方式:

- `ZSH_THEME_CONDA_PREFIX`:设置 CONDA_DEFAULT_ENV 的前缀。
默认为 `[`。

- `ZSH_THEME_CONDA_SUFFIX`:设置 CONDA_DEFAULT_ENV 的后缀。
默认为 `]`。

## 示例

```sh
ZSH_THEME_CONDA_PREFIX='conda:%F{green}'
ZSH_THEME_CONDA_SUFFIX='%f'
RPROMPT='$(conda_prompt_info)'
```

## `CONDA_CHANGEPS1`

本插件还会自动把 `CONDA_CHANGEPS1` 变量设为 `false`,以避免 conda 自动修改提示符。
其效果等同于运行 `conda config --set changeps1 false`。

你可以在 `.zshrc` 文件中、于 Oh My Zsh 被 source 之后添加 `unset CONDA_CHANGEPS1`,
来覆盖这一行为。

参考链接:

- https://conda.io/projects/conda/en/latest/user-guide/tasks/manage-environments.html#determining-your-current-environment
- https://conda.io/projects/conda/en/latest/user-guide/configuration/use-condarc.html#precedence
