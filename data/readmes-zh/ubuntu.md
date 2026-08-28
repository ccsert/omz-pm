# Ubuntu 插件

本插件为 [Ubuntu](https://www.ubuntu.com/) 提供补全和别名。

✅ 启用方式:把「ubuntu」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

使用 `$APT` 的命令,若已安装 `apt-fast` 则使用 `apt-fast`,否则若已安装 `apt` 则使用 `apt`,再否则回退到 `apt-get`。

| 别名 | 命令 | 说明 |
| ---- | ---- | ---- |
| age | `sudo $APT` | 以 sudo 运行 apt-get |
| acs | `apt-cache search` | 按指定条件搜索 apt-cache |
| acsp | `apt-cache showpkg` | 显示所列软件包的信息 |
| acp | `apt-cache policy` | 显示软件包来源的优先级 |
| afs | `apt-file search --regexp` | 执行正则表达式方式的 apt-file 搜索 |
| afu | `sudo apt-file update` | 生成或更新 apt-file 软件包数据库 |
| aga | `sudo $APT autoclean` | 清空本地仓库中已无法再下载的软件包文件 |
| agb | `sudo $APT build-dep <source_pkg>` | 安装/移除软件包,以满足指定构建包的依赖 |
| agc | `sudo $APT clean` | 清空本地仓库中已获取的软件包文件,仅保留锁文件中的内容 |
| agd | `sudo $APT dselect-upgrade` | 按照 dselect 的选择进行软件包安装 |
| agi | `sudo $APT install <pkg>` | 安装指定的软件包 |
| agli | `apt list --installed` | 列出已安装的软件包 |
| aglu | `apt list --upgradable` | 仅列出可用的更新 |
| agp | `sudo $APT purge <pkg>` | 移除软件包,包括其所有配置文件 |
| agr | `sudo $APT remove <pkg>` | 移除软件包 |
| ags | `$APT source <pkg>` | 获取指定软件包的源代码 |
| agu | `sudo $APT update` | 更新软件包列表 |
| agud | `sudo $APT update && sudo $APT dist-upgrade` | 更新软件包列表并执行发行版升级 |
| agug | `sudo $APT upgrade` | 升级可用的软件包 |
| agar | `sudo $APT autoremove` | 移除不再需要的自动安装软件包 |
| aguu | `sudo $APT update && sudo $APT upgrade` | 更新软件包列表并升级可用的软件包 |
| allpkgs | `dpkg --get-selections \| grep -v deinstall` | 打印所有已安装的软件包 |
| kclean | `sudo aptitude remove -P ?and(~i~nlinux-(ima\|hea) ?not(~n$(uname -r)))` | 移除除正在使用的之外的所有内核镜像和头文件 |
| mydeb | `time dpkg-buildpackage -rfakeroot -us -uc` | 创建一个基础的 .deb 软件包 |
| ppap | `sudo ppa-purge <ppa>` | 移除指定的 PPA |

## 函数

| 函数 | 用法 | 说明 |
| ---- | ---- | ---- |
| aar | `aar ppa:xxxxxx/xxxxxx [packagename]` | apt-add-repository,并自动安装/升级所需的软件包 |
| apt-history | `apt-history <action>` | 打印指定操作的 Apt 历史记录 |
| apt-list-packages | `apt-list-packages` | 按大小列出软件包 |
| kerndeb | `kerndeb` | 内核包构建快捷方式 |

## 作者

- [@AlexBio](https://github.com/AlexBio)
- [@dbb](https://github.com/dbb)
- [@Mappleconfusers](https://github.com/Mappleconfusers)
- [@trinaldi](https://github.com/trinaldi)
- [Nicolas Jonas](https://nextgenthemes.com)
- [@loctauxphilippe](https://github.com/loctauxphilippe)
- [@HaraldNordgren](https://github.com/HaraldNordgren)
- [@AmrElsayyad](https://github.com/AmrElsayyad)
