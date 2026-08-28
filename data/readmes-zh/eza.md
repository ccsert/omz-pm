# eza 插件

本插件提供一组别名,用 [`eza`](https://github.com/eza-community/eza) 工具代替 `ls`。

✅ 启用方式:把「eza」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 配置

所有配置都通过 `zstyle` 命令在 `:omz:plugins:eza` 命名空间下完成。

**注意:**配置必须在 OMZ 加载插件之前完成。插件加载之后,再修改 `zstyle` 就不会
有任何效果了。

### `dirs-first`

```zsh
zstyle ':omz:plugins:eza' 'dirs-first' yes|no
```

设为 `yes` 时,目录会排在前面成组显示。

默认值:`no`

### `git-status`

```zsh
zstyle ':omz:plugins:eza' 'git-status' yes|no
```

设为 `yes` 时,始终附加 `--git` 选项以显示 git 状态(如果被 git 跟踪/处于 git 仓库中)。

默认值:`no`

### `header`

```zsh
zstyle ':omz:plugins:eza' 'header' yes|no
```

设为 `yes` 时,始终附加 `-h` 选项,为每一列添加表头行。

默认值:`no`

### `show-group`

```zsh
zstyle ':omz:plugins:eza' 'show-group' yes|no|smart
```

设为 `yes`(默认)时,始终附加 `-g` 选项以显示属组。
设为 `smart` 时,附加 `--smart-group` 选项,仅当属组名与属主名不同时才显示属组。

默认值:`yes`

### `icons`

```zsh
zstyle ':omz:plugins:eza' 'icons' yes|no
```

设为 `yes` 时,设置 `eza` 的 `--icons` 选项,为文件和文件夹添加图标。

默认值:`no`

### `color-scale`

```zsh
zstyle ':omz:plugins:eza' 'color-scale' all|age|size
```

以不同颜色高亮显示各字段的级别。使用逗号(,)分隔的 `all`、`age`、`size` 列表。

默认值:`none`

### `color-scale-mode`

```zsh
zstyle ':omz:plugins:eza' 'color-scale-mode' gradient|fixed
```

选择高亮所用的模式:

- `gradient`(默认)—— 渐变着色
- `fixed` —— 固定着色

默认值:`gradient`

### `size-prefix`

```zsh
zstyle ':omz:plugins:eza' 'size-prefix' (binary|none|si)
```

选择显示文件大小时使用的词头:

- `binary` —— 使用[二进制前缀](https://en.wikipedia.org/wiki/Binary_prefix),如 "Ki"、"Mi"、"Gi"
  等
- `none` —— 不使用任何词头,以字节显示大小
- `si`(默认)—— 使用[公制/国际单位制(SI)词头](https://en.wikipedia.org/wiki/Metric_prefix)

默认值:`si`

### `time-style`

```zsh
zstyle ':omz:plugins:eza' 'time-style' $TIME_STYLE
```

设置 `eza` 的 `--time-style` 选项。(可用取值见 `man eza`)

默认值:未设置,即采用 `eza` 的默认行为。

### `hyperlink`

```zsh
zstyle ':omz:plugins:eza' 'hyperlink' yes|no
```

设为 `yes` 时,始终附加 `--hyperlink` 选项,通过转义序列创建超链接。

默认值:`no`

## 别名

**注意:**

- 别名可能会受上文配置的影响
- 「文件」一词若不带「仅」类限定词,则同时指文件与目录

| 别名   | 命令              | 说明                                                           |
| ------ | ----------------- | -------------------------------------------------------------- |
| `la`   | `eza -la`         | 以长列表列出所有文件(不含 . 和 ..)                          |
| `ldot` | `eza -ld .*`      | 仅列出点文件(目录作为条目显示,而不是递归进入)               |
| `lD`   | `eza -lD`         | 仅列出目录(不含点目录),以长列表显示                         |
| `lDD`  | `eza -laD`        | 仅列出目录(含点目录),以长列表显示                            |
| `ll`   | `eza -l`          | 以长列表列出文件                                               |
| `ls`   | `eza`             | 直接调用 eza                                                   |
| `lsd`  | `eza -d`          | 以网格列出指定文件,目录作为条目显示                           |
| `lsdl` | `eza -dl`         | 以长列表列出指定文件,目录作为条目显示                         |
| `lS`   | `eza -l -ssize`   | 以长列表列出文件,按大小排序                                   |
| `lT`   | `eza -l -snewest` | 以长列表列出文件,按日期排序(最新的排在最后)                 |
