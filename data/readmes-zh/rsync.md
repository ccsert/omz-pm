# rsync

本插件为常用的 [rsync](https://rsync.samba.org/) 命令添加了别名,简化文件传输和同步任务。

✅ 启用方式:把「rsync」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

| 别名                | 命令                                             | 说明                                                                                                                                       |
| ------------------- | ------------------------------------------------ | ------------|
| `rsync-copy`        | `rsync -avz --progress -h`                       | 递归复制文件和目录,保留权限、时间戳和符号链接。启用压缩以加快传输速度。以人类可读的格式显示进度。                                          |
| `rsync-move`        | `rsync -avz --progress -h --remove-source-files` | 与 rsync-copy 相同,但在传输成功后删除源文件(实际上相当于执行了移动)。                                                                    |
| `rsync-update`      | `rsync -avzu --progress -h`                      | 类似 rsync-copy,但只在源比目标新(或目标文件缺失)时才更新文件。                                                                           |
| `rsync-synchronize` | `rsync -avzu --delete --progress -h`             | 执行双向风格的同步:像 rsync-update 一样更新文件,并删除目标中在源里已不存在的文件。适用于目录同步。                                        |

参数说明:
 - -a:归档模式;保留符号链接、权限、时间戳等。
 - -v:详细模式;显示传输过程的细节。
 - -z:传输过程中压缩文件数据以提高效率。
 - -u:跳过接收端上更新的文件。
 - --progress:在文件传输过程中显示进度。
 - -h:以人类可读的格式输出数字(例如 1K、234M)。
 - --remove-source-files:复制完成后删除源文件(用于 rsync-move)。
 - --delete:删除目标中源里不存在的文件(用于 rsync-synchronize)。
