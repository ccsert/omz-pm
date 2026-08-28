# OpenTofu 插件

本插件面向 OpenTofu——Terraform 的一个开源、社区驱动、由 Linux 基金会管理的分支。它为
`tofu` 命令添加补全,还提供别名和一个提示符函数。

✅ 启用方式:把「opentofu」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 依赖要求

- [OpenTofu](https://opentofu.org/)

## 别名

| 别名   | 命令                         |
|--------|------------------------------|
| `tt`   | `tofu`                       |
| `tta`  | `tofu apply`                 |
| `tta!` | `tofu apply -auto-approve`   |
| `ttc`  | `tofu console`               |
| `ttd`  | `tofu destroy`               |
| `ttd!` | `tofu destroy -auto-approve` |
| `ttf`  | `tofu fmt`                   |
| `ttfr` | `tofu fmt -recursive`        |
| `tti`  | `tofu init`                  |
| `tto`  | `tofu output`                |
| `ttp`  | `tofu plan`                  |
| `ttv`  | `tofu validate`              |
| `tts`  | `tofu state`                 |
| `ttsh` | `tofu show`                  |
| `ttr`  | `tofu refresh`               |
| `ttt`  | `tofu test`                  |
| `ttws` | `tofu workspace`             |


## 提示符函数

- `tofu_prompt_info`:当处于 OpenTofu 项目目录中时,显示当前工作区。

- `tofu_version_prompt_info`:显示 `tofu` 命令的当前版本。

要使用它们,请把它们加入主题或 `.zshrc` 文件中的 `PROMPT` 变量:

```sh
PROMPT='$(tofu_prompt_info)'
RPROMPT='$(tofu_version_prompt_info)'
```

你还可以用下列变量为这两个函数指定 PREFIX 和 SUFFIX 字符串:

```sh
# for tofu_prompt_info
ZSH_THEME_TOFU_PROMPT_PREFIX="%{$fg[white]%}"
ZSH_THEME_TOFU_PROMPT_SUFFIX="%{$reset_color%}"
# for tofu_version_prompt_info
ZSH_THEME_TOFU_VERSION_PROMPT_PREFIX="%{$fg[white]%}"
ZSH_THEME_TOFU_VERSION_PROMPT_SUFFIX="%{$reset_color%}"
```
