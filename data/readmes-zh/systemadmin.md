# Systemadmin 插件

本插件提供一系列别名和函数,让系统管理员的日常工作更轻松。

✅ 启用方式:把「systemadmin」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名    | 命令                                                                       | 说明                                                               |
|---------|----------------------------------------------------------------------------|--------------------------------------------------------------------|
| ping    | `ping -c 5`                                                                | 只发送 5 条 ICMP 消息                                              |
| ping6   | `ping6 -c 5`                                                               | 只发送 5 条 ICMPv6 消息                                            |
| clr     | `clear; echo Currently logged in on $TTY, as $USERNAME in directory $PWD.` | 清屏并打印当前用户、TTY 和所在目录                                 |
| path    | `print -l $path`                                                           | 显示 PATH,每个条目独占一行                                        |
| mkdir   | `mkdir -pv`                                                                | 自动创建父目录并显示详细输出                                       |
| psmem   | `ps -e -orss=,args= \| sort -b -k1 -nr`                                    | 显示占用内存最多的进程                                             |
| psmem10 | `ps -e -orss=,args= \| sort -b -k1 -nr \| head -n 10`                      | 显示占用内存最多的前 10 个进程                                     |
| pscpu   | `ps -e -o pcpu,cpu,nice,state,cputime,args \|sort -k1 -nr`                 | 显示占用 CPU 最多的进程                                            |
| pscpu10 | `ps -e -o pcpu,cpu,nice,state,cputime,args \|sort -k1 -nr \| head -n 10`   | 显示占用 CPU 最多的前 10 个进程                                    |
| hist10  | `print -l ${(o)history%% *} \| uniq -c \| sort -nr \| head -n 10`          | 显示历史记录中使用最多的前 10 条命令                               |

## 函数

| 函数        |  说明                                                                                                                 |
|-------------|-----------------------------------------------------------------------------------------------------------------------|
| dls         | 只列出当前目录中的子目录                                                                                              |
| psgrep      | 列出与命令后输入的模式匹配的所有进程                                                                                  |
| killit      | 杀死与其收到的正则表达式匹配的所有进程                                                                                |
| tree        | 以树状格式列出目录内容(在 tree 未安装时)                                                                            |
| sortcons    | 按状态对连接排序                                                                                                      |
| con80       | 查看所有 80 端口连接                                                                                                  |
| sortconip   | 对已连接的 IP 按连接数排序                                                                                            |
| req20       | 列出 80 端口上排名前 20 的请求                                                                                        |
| http20      | 基于 tcpdump 数据列出 80 端口上排名前 20 的连接                                                                       |
| timewait20  | 列出排名前 20 的 time_wait 连接                                                                                       |
| syn20       | 列出排名前 20 的 SYN 连接                                                                                             |
| port_pro    | 按端口号输出所有进程                                                                                                  |
| accessip10  | 列出 nginx/access.log 文件(或指定的其他日志文件)中访问次数最多的前 10 个 IP 地址                                    |
| visitpage20 | 列出 nginx/access.log 文件(或指定的其他日志文件)中访问最多的前 20 个文件或页面                                       |
| consume100  | 列出最耗时的 100 个页面(超过 60 秒)及其出现的相应次数                                                               |
| webtraffic  | 从 nginx/access.log 文件(或指定的其他日志文件)统计网站流量(以 GB 计)                                               |
| c404        | 列出 nginx/access.log 文件(或指定的其他日志文件)中 404 连接的统计信息                                               |
| httpstatus  | 基于 nginx/access.log 文件(或指定的其他日志文件)中的 HTTP 状态列出统计信息                                          |
| d0          | 递归删除当前目录(或指定的其他目录)中的 0 字节文件                                                                   |
| geteip      | 使用 [icanhazip.com](https://icanhazip.com) 获取外部 IP 地址信息                                                      |
| getip       | 用 `ip addr` 或 `ifconfig` 确定本地 IP 地址                                                                           |
| clrz        | 清除僵尸进程                                                                                                          |
| conssec     | 基于 nginx/access.log 文件(或指定的其他日志文件)显示每秒并发连接数                                                  |
