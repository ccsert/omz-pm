# virtualenv

本插件显示已创建的虚拟容器的信息,并支持对它进行主题定制。

✅ 启用方式:把「virtualenv」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

本插件会创建一个 `virtualenv_prompt_info` 函数供你在主题中使用,它显示
当前 `$VIRTUAL_ENV` 的基本名称(basename)。它使用两个变量来控制显示方式:

- `ZSH_THEME_VIRTUALENV_PREFIX`:设置 VIRTUAL_ENV 的前缀。默认为 `[`。

- `ZSH_THEME_VIRTUALENV_SUFFIX`:设置 VIRTUAL_ENV 的后缀。默认为 `]`。
