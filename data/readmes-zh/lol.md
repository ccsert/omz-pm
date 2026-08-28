# lol 插件

一个添加「猫语」(catspeak)别名的插件,何乐而不为。

✅ 启用方式:把「lol」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名         | 命令                                                            |
| ------------ | --------------------------------------------------------------- |
| `:3`         | `echo`                                                          |
| `alwayz`     | `tail -f`                                                       |
| `bringz`     | `git pull`                                                      |
| `btw`        | `nice`                                                          |
| `byes`       | `exit`                                                          |
| `chicken`    | `git add`                                                       |
| `cya`        | `reboot`                                                        |
| `donotwant`  | `rm`                                                            |
| `dowant`     | `cp`                                                            |
| `gimmeh`     | `touch`                                                         |
| `gtfo`       | `mv`                                                            |
| `hackzor`    | `git init`                                                      |
| `hai`        | `cd`                                                            |
| `icanhas`    | `mkdir`                                                         |
| `ihasbucket` | `df -h`                                                         |
| `iminurbase` | `finger`                                                        |
| `inur`       | `locate`                                                        |
| `invisible`  | `cat`                                                           |
| `iz`         | `ls`                                                            |
| `kthxbai`    | `halt`                                                          |
| `letcat`     | `git checkout`                                                  |
| `moar`       | `more`                                                          |
| `nomnom`     | `killall`                                                       |
| `nomz`       | `ps aux`                                                        |
| `nowai`      | `chmod`                                                         |
| `oanward`    | `git commit -m`                                                 |
| `obtw`       | `nohup`                                                         |
| `onoz`       | `cat /var/log/errors.log`                                       |
| `ooanward`   | `git commit -am`                                                |
| `plz`        | `pwd`                                                           |
| `pwned`      | `ssh`                                                           |
| `rtfm`       | `man`                                                           |
| `rulz`       | `git push`                                                      |
| `tldr`       | `less`                                                          |
| `violenz`    | `git rebase`                                                    |
| `visible`    | `echo`                                                          |
| `wtf`        | `dmesg`                                                         |
| `yolo`       | `git commit -m "$(curl -s https://whatthecommit.com/index.txt)"` |

## 用法示例

```sh
# mkdir new-directory
icanhas new-directory

# killall firefox
nomnom firefox

# chmod u=r,go= some.file
nowai u=r,go= some.file

# ssh root@catserver.org
pwned root@catserver.org

# git commit -m "$(curl -s https://whatthecommit.com/index.txt)"
yolo
```
