# Arch Linux 插件

本插件为 Arch Linux 提供了一些别名和函数。

✅ 启用方式:把「archlinux」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 特性

### Pacman

| 别名         | 命令                                   | 说明                                                       |
|--------------|----------------------------------------|------------------------------------------------------------|
| pacin        | `sudo pacman -S`                       | 从仓库安装软件包                                           |
| pacins       | `sudo pacman -U`                       | 从本地文件安装软件包                                       |
| pacinsd      | `sudo pacman -S --asdeps`              | 将软件包作为其他软件包的依赖安装                           |
| paclean      | `sudo pacman -Sc`                      | 清理旧的、不再使用的缓存和软件包                           |
| pacloc       | `pacman -Qi`                           | 显示本地数据库中某个软件包的信息                           |
| paclocs      | `pacman -Qs`                           | 在本地数据库中搜索软件包                                   |
| paclr        | `sudo pacman -Scc`                     | 清空缓存中的全部文件                                       |
| paclsorphans | `pacman -Qdt`                          | 列出所有孤立的软件包                                       |
| pacmir       | `sudo pacman -Syy`                     | 更新镜像列表后强制刷新所有软件包列表                       |
| pacre        | `sudo pacman -R`                       | 删除软件包,但保留其设置和依赖                             |
| pacrem       | `sudo pacman -Rns`                     | 删除软件包,连同其设置和依赖一起删除                       |
| pacrep       | `pacman -Si`                           | 显示仓库中某个软件包的信息                                 |
| pacreps      | `pacman -Ss`                           | 在仓库中搜索软件包                                         |
| pacrmorphans | `sudo pacman -Rs $(pacman -Qtdq)`      | 删除所有孤立的软件包                                       |
| pacupd       | `sudo pacman -Sy`                      | 更新并刷新本地软件包、ABS 和 AUR 数据库                    |
| pacupg       | `sudo pacman -Syu`                     | 先与仓库同步,再升级软件包                                 |
| pacfileupg   | `sudo pacman -Fy`                      | 从服务器下载最新的软件包数据库                             |
| pacfiles     | `pacman -F`                            | 在软件包文件名中搜索匹配的字符串                           |
| pacls        | `pacman -Ql`                           | 列出某个软件包内的文件                                     |
| pacown       | `pacman -Qo`                           | 显示某个文件属于哪个软件包                                 |
| upgrade[¹](#f1) | `sudo pacman -Syu`                  | 先与仓库同步,再升级软件包                                 |

| 函数           | 说明                                                      |
|----------------|-----------------------------------------------------------|
| pacdisowned    | 列出系统中所有不属于任何软件包的文件                      |
| paclist        | 列出所有显式安装的软件包及其描述                          |
| pacmanallkeys  | 获取开发者和受信任用户的全部密钥                          |
| pacmansignkeys | 在本地信任作为参数传入的所有密钥                          |
| pacweb         | 打开 ArchLinux 软件包的网站                               |

注:paclist 过去会列出同时满足以下两个条件的软件包及其描述:(1)显式安装;
(2)有可用升级。由于脚本实现有缺陷,在没有可用升级时,它会把所有软件包都打印出来。
请改用 `pacman -Que`。

### AUR 助手

#### Aura

| 别名    | 命令                                                           | 说明                                                                      |
|---------|----------------------------------------------------------------|---------------------------------------------------------------------------|
| auclean | `sudo aura -Sc`                                                | 清理旧的、不再使用的缓存和软件包                                          |
| auclr   | `sudo aura -Scc`                                               | 清空缓存中的全部文件                                                      |
| auin    | `sudo aura -S`                                                 | 从仓库安装软件包                                                          |
| aurin   | `sudo aura -A`                                                 | 从仓库安装软件包                                                          |
| auins   | `sudo aura -U`                                                 | 从本地文件安装软件包                                                      |
| auinsd  | `sudo aura -S --asdeps`                                        | 将软件包作为其他软件包的依赖安装(仅限仓库)                              |
| aurinsd | `sudo aura -A --asdeps`                                        | 将软件包作为其他软件包的依赖安装(仅限 AUR)                              |
| auloc   | `aura -Qi`                                                     | 显示本地数据库中某个软件包的信息                                          |
| aulocs  | `aura -Qs`                                                     | 在本地数据库中搜索软件包                                                  |
| auls    | `aura -Qql`                                                    | 列出指定软件包拥有的所有文件                                              |
| aulst   | `aura -Qe`                                                     | 列出已安装的软件包,包括来自 AUR 的(标记为「local」)                    |
| aumir   | `sudo aura -Syy`                                               | 更新镜像列表后强制刷新所有软件包列表                                      |
| aurph   | `sudo aura -Oj`                                                | 用 aura 删除孤立软件包                                                    |
| auown   | `aura -Qqo`                                                    | 搜索拥有指定文件的软件包                                                  |
| aure    | `sudo aura -R`                                                 | 删除软件包,但保留其设置和依赖                                            |
| aurem   | `sudo aura -Rns`                                               | 删除软件包,连同其设置和不再需要的依赖一起删除                            |
| aurep   | `aura -Si`                                                     | 显示仓库中某个软件包的信息                                                |
| aurrep  | `aura -Ai`                                                     | 显示 AUR 中某个软件包的信息                                               |
| aureps  | `aura -As --both`                                              | 在仓库和 AUR 中搜索软件包                                                 |
| auras   | `aura -As --both`                                              | 同上                                                                      |
| auupd   | `sudo aura -Sy`                                                | 更新并刷新本地软件包、ABS 和 AUR 数据库                                   |
| auupg   | `sudo sh -c "aura -Syu              && aura -Au"`              | 先与仓库同步,再升级所有软件包(包括 AUR 的)                            |
| ausu    | `sudo sh -c "aura -Syu --no-confirm && aura -Au --no-confirm"` | 与 `auupg` 相同,但无需确认                                               |
| upgrade[¹](#f1) | `sudo aura -Syu`                                       | 先与仓库同步,再升级软件包                                                |

| 函数            | 说明                                                                |
|-----------------|---------------------------------------------------------------------|
| auownloc _file_ | 显示拥有指定文件的软件包的信息                                      |
| auownls  _file_ | 列出拥有指定文件的软件包所包含的所有文件                            |

#### Pacaur

| 别名    | 命令                              | 说明                                                                |
|---------|-----------------------------------|---------------------------------------------------------------------|
| pacclean| `pacaur -Sc`                      | 清理旧的、不再使用的缓存和软件包                                    |
| pacclr  | `pacaur -Scc`                     | 清空缓存中的全部文件                                                |
| pain    | `pacaur -S`                       | 从仓库安装软件包                                                    |
| pains   | `pacaur -U`                       | 从本地文件安装软件包                                                |
| painsd  | `pacaur -S --asdeps`              | 将软件包作为其他软件包的依赖安装                                    |
| paloc   | `pacaur -Qi`                      | 显示本地数据库中某个软件包的信息                                    |
| palocs  | `pacaur -Qs`                      | 在本地数据库中搜索软件包                                            |
| palst   | `pacaur -Qe`                      | 列出已安装的软件包,包括来自 AUR 的(标记为「local」)              |
| pamir   | `pacaur -Syy`                     | 更新镜像列表后强制刷新所有软件包列表                                |
| paorph  | `pacaur -Qtd`                     | 用 pacaur 删除孤立软件包                                            |
| pare    | `pacaur -R`                       | 删除软件包,但保留其设置和依赖                                      |
| parem   | `pacaur -Rns`                     | 删除软件包,连同其设置和不再需要的依赖一起删除                      |
| parep   | `pacaur -Si`                      | 显示仓库中某个软件包的信息                                          |
| pareps  | `pacaur -Ss`                      | 在仓库中搜索软件包                                                  |
| paupd   | `pacaur -Sy`                      | 更新并刷新本地软件包、ABS 和 AUR 数据库                             |
| paupg   | `pacaur -Syua`                    | 先与仓库同步,再升级所有软件包(包括 AUR 的)                      |
| pasu    | `pacaur -Syua --no-confirm`       | 与 `paupg` 相同,但无需确认                                         |
| upgrade[¹](#f1) | `pacaur -Syu`             | 先与仓库同步,再升级软件包                                          |

#### Trizen

| 别名    | 命令                              | 说明                                                                |
|---------|-----------------------------------|---------------------------------------------------------------------|
| trconf  | `trizen -C`                       | 用 vimdiff 修复所有配置文件                                         |
| trclean | `trizen -Sc`                      | 清理旧的、不再使用的缓存和软件包                                    |
| trclr   | `trizen -Scc`                     | 清空缓存中的全部文件                                                |
| trin    | `trizen -S`                       | 从仓库安装软件包                                                    |
| trins   | `trizen -U`                       | 从本地文件安装软件包                                                |
| trinsd  | `trizen -S --asdeps`              | 将软件包作为其他软件包的依赖安装                                    |
| trloc   | `trizen -Qi`                      | 显示本地数据库中某个软件包的信息                                    |
| trlocs  | `trizen -Qs`                      | 在本地数据库中搜索软件包                                            |
| trlst   | `trizen -Qe`                      | 列出已安装的软件包,包括来自 AUR 的(标记为「local」)              |
| trmir   | `trizen -Syy`                     | 更新镜像列表后强制刷新所有软件包列表                                |
| trorph  | `trizen -Qtd`                     | 用 yaourt 删除孤立软件包                                            |
| trre    | `trizen -R`                       | 删除软件包,但保留其设置和依赖                                      |
| trrem   | `trizen -Rns`                     | 删除软件包,连同其设置和不再需要的依赖一起删除                      |
| trrep   | `trizen -Si`                      | 显示仓库中某个软件包的信息                                          |
| trreps  | `trizen -Ss`                      | 在仓库中搜索软件包                                                  |
| trupd   | `trizen -Sy`                      | 更新并刷新本地软件包、ABS 和 AUR 数据库                             |
| trupg   | `trizen -Syua`                    | 先与仓库同步,再升级所有软件包(包括 AUR 的)                      |
| trsu    | `trizen -Syua --no-confirm`       | 与 `trupg` 相同,但无需确认                                         |
| upgrade[¹](#f1) | `trizen -Syu`             | 先与仓库同步,再升级软件包                                          |

#### Yay

| 别名    | 命令                           | 说明                                                              |
|---------|--------------------------------|-------------------------------------------------------------------|
| yaconf  | `yay -Pg`                      | 打印当前配置                                                      |
| yaclean | `yay -Sc`                      | 清理旧的、不再使用的缓存和软件包                                  |
| yaclr   | `yay -Scc`                     | 清空缓存中的全部文件                                              |
| yain    | `yay -S`                       | 从仓库安装软件包                                                  |
| yains   | `yay -U`                       | 从本地文件安装软件包                                              |
| yainsd  | `yay -S --asdeps`              | 将软件包作为其他软件包的依赖安装                                  |
| yaloc   | `yay -Qi`                      | 显示本地数据库中某个软件包的信息                                  |
| yalocs  | `yay -Qs`                      | 在本地数据库中搜索软件包                                          |
| yalst   | `yay -Qe`                      | 列出已安装的软件包,包括来自 AUR 的(标记为「local」)            |
| yamir   | `yay -Syy`                     | 更新镜像列表后强制刷新所有软件包列表                              |
| yaorph  | `yay -Qtd`                     | 用 yay 删除孤立软件包                                             |
| yare    | `yay -R`                       | 删除软件包,但保留其设置和依赖                                    |
| yarem   | `yay -Rns`                     | 删除软件包,连同其设置和不再需要的依赖一起删除                    |
| yarep   | `yay -Si`                      | 显示仓库中某个软件包的信息                                        |
| yareps  | `yay -Ss`                      | 在仓库中搜索软件包                                                |
| yaupd   | `yay -Sy`                      | 更新并刷新本地软件包、ABS 和 AUR 数据库                           |
| yaupg   | `yay -Syu`                     | 先与仓库同步,再升级软件包                                        |
| yasu    | `yay -Syu --no-confirm`        | 与 `yaupg` 相同,但无需确认                                       |
| upgrade[¹](#f1) | `yay -Syu`             | 先与仓库同步,再升级软件包                                        |

---

<span id="f1">¹</span>
`upgrade` 别名对所有软件包管理器都会设置。它的具体取值取决于对应软件包管理器
是否已安装,按以下顺序依次检查:

1. `yay`
2. `trizen`
3. `pacaur`
4. `aura`
5. `pacman`

## 贡献者

- Benjamin Boudreau - dreurmail@gmail.com
- Celso Miranda - contacto@celsomiranda.net
- ratijas (ivan tkachenko) - me@ratijas.tk
- Juraj Fiala - doctorjellyface@riseup.net
- KhasMek - Boushh@gmail.com
- Majora320 (Moses Miller) - Majora320@gmail.com
- Martin Putniorz - mputniorz@gmail.com
- MatthR3D - matthr3d@gmail.com
- ornicar - thibault.duplessis@gmail.com
- Ybalrid (Arthur Brainville) - ybalrid@ybalrid.info
- Jeff M. Hubbard - jeffmhubbard@gmail.com
- K. Harishankar(harishnkr) - hari2menon1234@gmail.com
- WH-2099 - wh2099@outlook.com
