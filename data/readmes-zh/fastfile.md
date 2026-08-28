# Fastfile 插件

本插件提供了一种借助全局别名(快捷方式)来引用常用文件或文件夹的方法。

✅ 启用方式:把「fastfile」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

示例:你非常频繁地访问文件夹 `/code/project/backend/database`。

首先,生成一个名为 `pjdb` 的快捷方式:

```zsh
$ fastfile pjdb /code/project/backend/database
```

下次想访问它时,使用 `§pjdb` 即可。例如:

```zsh
$ cd §pjdb
$ subl §pjdb
```

其中 § 是 fastfile 的前缀(如何修改见[下文](#options))。

**注意:** 名称中带空格的快捷方式,在创建全局别名时空格会被替换成下划线(`_`)。例如:
名为 `"hello world"` 的快捷方式对应 `§hello_world`。

## 函数

- `fastfile <shortcut_name> [path/to/file/or/folder]`:生成一个快捷方式。
  如果未提供第二个参数,则使用当前目录。

- `fastfile_print <shortcut_name>`:打印一个快捷方式,格式为
  `<prefix><shortcut_name> -> <shortcut_path>`。

- `fastfile_ls`:列出所有快捷方式。

- `fastfile_rm <shortcut_name>`:删除一个快捷方式。

- `fastfile_sync`:为所有快捷方式生成全局别名。

### 内部函数

- `fastfile_resolv <shortcut_name>`:解析快捷方式文件的位置,即 fastfile 目录中
  存储该快捷方式路径的那个文件。

- `fastfile_get <shortcut_name>`:获取快捷方式的实际路径。

## 别名

| 别名   | 函数             |
|--------|------------------|
| ff     | `fastfile`       |
| ffp    | `fastfile_print` |
| ffrm   | `fastfile_rm`    |
| ffls   | `fastfile_ls`    |
| ffsync | `fastfile_sync`  |

## 选项

以下选项可用于修改插件的某些部分。要修改它们,请在加载 Oh My Zsh 之前,
把 `<variable>=<value>` 添加到你的 zshrc 文件中。
例如:`fastfile_var_prefix='@'`。

- `fastfile_var_prefix`:所创建全局别名的前缀。控制创建出来的全局别名的前缀。
  **默认值:** `§`(章节符号),在德式键盘上通过
  [`⇧ Shift`+`3`](https://en.wikipedia.org/wiki/German_keyboard_layout#/media/File:KB_Germany.svg)
  组合键即可轻松输入,在 macOS 上则是 `⌥ Option`+`6`。

- `fastfile_dir`:存储 fastfile 快捷方式的目录。必须以斜杠结尾。
  **默认值:** `$HOME/.fastfile/`。

## 作者

- [Karolin Varner](https://github.com/koraa)
