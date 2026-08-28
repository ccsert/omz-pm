# debian 插件

本插件为 zsh 提供 Debian 相关的别名和函数。

✅ 启用方式:把「debian」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 设置

- `$apt_pref`:若系统装有 aptitude 或 apt 则使用之,否则回退到 apt-get。
- `$apt_upgr`:使用 upgrade 或 safe-upgrade(适用于 aptitude)。

把 `$apt_pref` 和 `$apt_upgr` **两者**都设置成你想要的命令(需在加载 Oh My Zsh 之前),即可覆盖上述行为,例如:

```sh
apt_pref='apt'
apt_upgr='full-upgrade'
```

## 常用别名

| 别名   | 命令                                                                   | 说明                                                       |
| ------ | ---------------------------------------------------------------------- | ---------------------------------------------------------- |
| `age`  | `apt-get`                                                              | 处理软件包的命令行工具                                     |
| `api`  | `aptitude`                                                             | 功能与 `apt-get` 相同,并提供额外选项                      |
| `acs`  | `apt-cache search`                                                     | 用于搜索 apt 软件包缓存的命令行工具                        |
| `aps`  | `aptitude search`                                                      | 用 aptitude 搜索已安装的软件包                             |
| `as`   | `aptitude -F '* %p -> %d \n(%v/%V)' --no-gui --disable-columns search` | 以自定义格式显示搜索到的软件包                             |
| `afs`  | `apt-file search --regexp`                                             | 在软件包中搜索文件                                         |
| `asrc` | `apt-get source`                                                       | 通过 `apt-get` 获取源码包                                  |
| `app`  | `apt-cache policy`                                                     | 显示软件包源的优先级                                       |

## 超级用户操作别名

| 别名     | 命令                                                                                  | 说明                                                          |
| -------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| `aac`    | `sudo $apt_pref autoclean`                                                            | 清空本地仓库中已下载的软件包文件                              |
| `aar`    | `sudo $apt_pref autoremove`                                                           | 删除不再需要的、自动安装的软件包                              |
| `abd`    | `sudo $apt_pref build-dep`                                                            | 安装构建软件包所需的全部依赖                                  |
| `ac`     | `sudo $apt_pref clean`                                                                | 清空本地仓库中已下载的软件包文件(锁定文件除外)              |
| `ad`     | `sudo $apt_pref update`                                                               | 更新可供升级的软件包列表                                      |
| `adg`    | `sudo $apt_pref update && sudo $apt_pref $apt_upgr`                                   | 更新并升级软件包                                              |
| `ads`    | `sudo apt-get dselect-upgrade`                                                        | 按清单安装软件包,并移除所有不在清单中的软件包                |
| `adu`    | `sudo $apt_pref update && sudo $apt_pref dist-upgrade`                                | 能智能处理依赖关系的升级                                      |
| `afu`    | `sudo apt-file update`                                                                | 更新软件包中的文件列表                                        |
| `ai`     | `sudo $apt_pref install`                                                              | 安装软件包的命令行工具                                        |
| `ail`    | `sed -e 's/ */ /g' -e 's/ *//' \| cut -s -d ' ' -f 1 \| xargs sudo $apt_pref install` | 安装命令行中给出的所有软件包,每行只取第一个词                |
| `alu`    | `sudo apt update && apt list -u && sudo apt upgrade`                                  | 更新、列出并升级软件包                                        |
| `ap`     | `sudo $apt_pref purge`                                                                | 卸载软件包并连同其配置文件一起删除                            |
| `au`     | `sudo $apt_pref $apt_upgr`                                                            | 安装软件包升级                                                |
| `di`     | `sudo dpkg -i`                                                                        | 安装当前目录下的所有 .deb 文件                                |
| `dia`    | `sudo dpkg -i ./*.deb`                                                                | 安装当前目录下的所有 .deb 文件                                |
| `kclean` | `sudo aptitude remove -P ?and(~i~nlinux-(ima\|hea) ?not(~n$(uname -r)))`              | 删除除正在使用的之外的所有内核镜像和头文件                    |

## 别名——使用 `su` 的命令

| 别名  | 命令                                                      |
| ----- | --------------------------------------------------------- |
| `aac` | `su -ls "$apt_pref autoclean" root`                       |
| `aar` | `su -ls "$apt_pref autoremove" root`                      |
| `ac`  | `su -ls "$apt_pref clean" root`                           |
| `ad`  | `su -lc "$apt_pref update" root`                          |
| `adg` | `su -lc "$apt_pref update && aptitude $apt_upgr" root`    |
| `adu` | `su -lc "$apt_pref update && aptitude dist-upgrade" root` |
| `afu` | `su -lc "apt-file update"`                                |
| `au`  | `su -lc "$apt_pref $apt_upgr" root`                       |
| `dia` | `su -lc "dpkg -i ./*.deb" root`                           |

## 其他别名

| 别名      | 命令                                           | 说明                           |
| --------- | ---------------------------------------------- | ------------------------------ |
| `allpkgs` | `aptitude search -F "%p" --disable-columns ~i` | 显示所有已安装的软件包         |
| `mydeb`   | `time dpkg-buildpackage -rfakeroot -us -uc`    | 创建一个基础的 .deb 软件包     |

## 函数

| 函数                | 说明                                                            |
| ------------------- | --------------------------------------------------------------- |
| `apt-copy`          | 生成一个可用于「复制」系统的简单脚本                            |
| `apt-history`       | 显示某个命令的 apt 历史                                         |
| `apt-list-packages` | 按大小列出软件包                                                |
| `kerndeb`           | 构建内核软件包                                                  |

## 作者

- [@AlexBio](https://github.com/AlexBio)
- [@dbb](https://github.com/dbb)
- [@Mappleconfusers](https://github.com/Mappleconfusers)
