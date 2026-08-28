# Lando ZSH(lando-zsh)

本插件为在 [Lando](https://docs.lando.dev/basics/)(基于 Docker)中使用各种语言和框架添加了别名。它只会在由 lando 驱动的项目目录中生效。

✅ 启用方式:把「lando」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 包装的命令

| 别名       | 说明             |
|:----------:|:----------------:|
| `artisan`  | `lando artisan`  |
| `composer` | `lando composer` |
| `drush`    | `lando drush`    |
| `gulp`     | `lando gulp`     |
| `npm`      | `lando npm`      |
| `php`      | `lando php`      |
| `wp`       | `lando wp`       |
| `yarn`     | `lando yarn`     |

如需包装更多或不同的命令,可以设置 `LANDO_ZSH_WRAPPED_COMMANDS`,见下文[设置](#settings)。

## 工作原理:

本插件免去了在命令前面输入 `lando` 的麻烦。它会使用受支持命令的 lando 版本,前提是运行目录满足以下条件:

- 在当前目录或 `$LANDO_ZSH_SITES_DIRECTORY` 范围内的任何父目录中找到了 `.lando.yml` 文件。
- 当前目录位于 `$LANDO_ZSH_SITES_DIRECTORY` 之内,但不是 `$LANDO_ZSH_SITES_DIRECTORY` 本身。
- 如果该命令不属于 lando 环境中可用的命令,则会不带 `lando` 直接运行。

## 设置:

> 注意:这些设置必须在插件加载*之前*设置好,并且任何更改都需要重启 shell 才能生效。

- `LANDO_ZSH_SITES_DIRECTORY`:插件向上层目录搜索 `CONFIG_FILE` 时,一旦到达该目录就会停止:
  ```sh
  LANDO_ZSH_SITES_DIRECTORY="$HOME/Code"
  ```

- `LANDO_ZSH_CONFIG_FILE`:插件会检查这个指定文件是否存在,以判断 Lando 是否存在:
  ```sh
  LANDO_ZSH_CONFIG_FILE=".lando.dev.yml"
  ```

- `LANDO_ZSH_WRAPPED_COMMANDS`:要包装的命令列表,是一个以空格分隔各命令的字符串:
  ```sh
  LANDO_ZSH_WRAPPED_COMMANDS="mysql php composer test artisan"
  ```

## 作者:

- 作者:Joshua Bedford
- URL:[https://github.com/joshuabedford/lando-zsh](https://github.com/joshuabedford/lando-zsh)
