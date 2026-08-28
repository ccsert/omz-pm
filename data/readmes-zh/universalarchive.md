# universalarchive 插件

`universalarchive` 插件提供了一个便捷的命令行界面,可以用多种压缩格式归档文件和目录——无需记住每种工具的具体语法。

✅ 启用方式:把「universalarchive」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 特性
 - 用一条简单统一的命令压缩文件和目录:ua <format> <files>
 - 自动检测文件/目录名,生成合适的输出名称
 - 当输出文件已存在时,支持回退命名
 - 支持多种常见及进阶的压缩格式
 - 为终端中的简单快捷使用而设计

## 用法

基本命令格式:
```sh
ua <format> <files...>
```
- `<format>`:要使用的归档格式(例如 `zip`、`tar.gz`、`xz`、`7z` 等)
- `<files...>`:要压缩的一个或多个文件或目录

## 示例:

把 `notes.txt` 和 `images` 压缩为 `notes.zip`
```sh
ua zip notes.txt images/
```

创建 `myproject.tar.gz`
```sh
ua tar.gz myproject/
```

把所有 .log 文件压缩为 `current_folder.xz`
```sh
ua xz *.log
```

插件会根据输入生成默认的归档文件名:
 - 对单个文件,输出名取自去掉扩展名后的文件名。
 - 对目录,使用目录名。
 - 对多个文件,使用共同父目录的名称。

 若输出文件已存在,则会用 `mktemp` 生成一个唯一的文件名。

## 支持的归档格式

| 格式             | 说明                           | 使用的工具       |
|:-----------------|:-------------------------------|:-----------------|
| `7z`             | 7zip 归档                      | `7z`             |
| `bz2`            | Bzip2 压缩文件                 | `bzip2`          |
| `gz`             | Gzip 压缩文件                  | `gzip`           |
| `lzma`           | LZMA 压缩文件                  | `lzma`           |
| `lzo`            | LZO 压缩文件                   | `lzop`           |
| `rar`            | WinRAR 归档                    | `rar`            |
| `tar`            | 未压缩的 tar 包                | `tar`            |
| `tbz`,`tar.bz2`  | 用 Bzip2 压缩的 tar 包         | `tar + bzip2`    |
| `tgz`,`tar.gz`   | 用 Gzip 压缩的 tar 包          | `tar + gzip`     |
| `tlz`,`tar.lzma` | 用 LZMA 压缩的 tar 包          | `tar + lzma`     |
| `txz`,`tar.xz`   | 用 LZMA2 压缩的 tar 包         | `tar + xz`       |
| `tZ`,`tar.Z`     | 用 LZW 压缩的 tar 包           | `tar + compress` |
| `xz`             | XZ 压缩文件                    | `xz`             |
| `Z`              | LZW 压缩文件                   | `compress`       |
| `zip`            | 标准 Zip 归档                  | `zip`            |
| `zst`            | Zstandard 压缩文件             | `zstd`           |

 > 注意:某些格式可能需要在系统上安装特定工具(例如 `7z`、`rar`、`lzop`、`zstd`)。请确保这些工具在你的 `$PATH` 中可用。

## 自动补全

本插件为支持的格式和输入文件提供 Tab 补全。输入 `ua <TAB>` 查看可用格式,输入 `ua <format> <TAB>` 浏览文件。
