# rbenv 插件

本插件的首要任务是提供 `rbenv_prompt_info`,你可以把它加入你的主题,
让提示符中包含 Ruby 版本和 gemset 信息。

如果未同时安装 rbenv 的 *gemset* 插件,本插件的某些功能将无法使用。
https://github.com/jf/rbenv-gemset

✅ 启用方式:把「rbenv」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名    | 命令                | 说明                   |
| ------- | ------------------- | ---------------------- |
| rubies  | `rbenv versions`    | 列出已安装的 Ruby 版本 |
| gemsets | `rbenv gemset list` | 列出现有的 gemset      |

## 函数

* `current_ruby`:当前正在使用的 Ruby 版本。
* `current_gemset`:当前 gemset 的名称。
* `gems`:以增强的格式和颜色列出已安装的 gem。
* `rbenv_prompt_info`:用于向提示符添加信息。格式:`<ruby version>@<current gemset>`。
