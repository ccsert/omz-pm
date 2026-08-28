# fasd

[`Fasd`](https://github.com/clvv/fasd)(发音类似 "fast")是一款命令行效率提升工具。Fasd 为 POSIX shell 提供对文件和目录的快速访问。

✅ 启用方式:把「fasd」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 安装

详细安装指南请见[`这里`](https://github.com/whjvenyl/fasd#install)。

## 别名

| 别名 | 命令                                   | 说明                                       |
| ---- | -------------------------------------- | ------------------------------------------ |
| v    | `fasd -f -e "$EDITOR"`                 | 列出与给定文件名匹配的常用/近期文件。      |
| o    | `fasd -a -e xdg-open`                  | 列出匹配的常用/近期文件和目录。            |
| j    | `fasd_cd -d -i`                        | 通过交互式选择执行 cd                      |
