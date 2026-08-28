# cp 插件

本插件定义了一个 `cpv` 函数,它底层使用 `rsync`,让你同时获得该命令的特性与安全性。

✅ 启用方式:把「cp」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 说明

rsync 启用的选项如下:

- `-p`:保留权限。

- `-o`:保留属主。

* `-g`:保留属组。

* `-b`:如果原始文件已存在,先为其创建备份,而不是直接覆盖。

* `-r`:递归处理目录。

* `-hhh`:以人类可读的格式输出数字,单位进率为 1024(K、M、G、T)。

* `--backup-dir="/tmp/rsync-$USERNAME"`:把备份副本移动到 "/tmp/rsync-$USERNAME"。

* `-e /dev/null`:只处理本地文件(禁用远程 shell)。

* `--progress`:显示进度。
