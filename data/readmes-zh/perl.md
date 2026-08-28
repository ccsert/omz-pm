# Perl

本插件为 [perl](https://www.perl.org/) 提供实用的别名/函数。

✅ 启用方式:把「perl」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## Perlbrew 激活

如果插件检测到 `perlbrew` 尚未激活,但 `$PERLBREW_ROOT` 中存在它的安装,就会默认进行初始化。要避免这一行为,请在 zshrc 中 `source oh-my-zsh.sh` 之前设置 `ZSH_PERLBREW_ACTIVATE=false`。

## 别名

| 别名        | 命令               | 说明                                   |
| :---------- | :----------------- | :------------------------------------- |
| pbi         | `perlbrew install` | 安装指定版本的 perl                    |
| pbl         | `perlbrew list`    | 列出已安装的所有 perl 版本             |
| pbo         | `perlbrew off`     | 回到系统 perl                          |
| pbs         | `perlbrew switch`  | 重新开启                               |
| pbu         | `perlbrew use`     | 使用指定版本的 perl                    |
| pd          | `perldoc`          | 显示 perl 文档                         |
| ple         | `perl -wlne`       | 像 awk/sed 一样使用 perl               |
| latest-perl | `curl ...`         | 显示 Perl 的最新稳定版本               |

## 函数

- `newpl`:创建一个基础的 Perl 脚本文件,并用 $EDITOR 打开它。

- `pgs`:Perl 全局替换(Perl Global Substitution):`pgs <find_pattern> <replace_pattern> <filename>` 在
  `<filename>` 中查找 `<find_pattern>` 并将其替换为 `<replace_pattern>`。

- `prep`:Perl 版的 grep,因为 'grep -P' 实在难用:`prep <pattern> [<filename>]` 既可以用管道也可以用
  文件(如果不提供 `<filename>`,则使用 stdin)。

## 依赖要求

为了使其正常工作,你需要安装 perl。关于用法和安装的更多信息:
https://www.perl.org/get.html
