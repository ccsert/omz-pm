# command-not-found 插件

本插件使用 zsh 的 command-not-found 包,在找不到某个命令时给出建议安装的软件包。

✅ 启用方式:把「command-not-found」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

下面是本插件在 Ubuntu 上的工作示例:
```
$ mutt
The program 'mutt' can be found in the following packages:
 * mutt
 * mutt-kz
 * mutt-patched
Try: sudo apt install <selected package>
```

### 支持的平台

对于下列平台的 command-not-found 包,本插件开箱即用:

- [Ubuntu](https://launchpad.net/ubuntu/+source/command-not-found)
- [Debian](https://packages.debian.org/search?keywords=command-not-found)
- [Arch Linux](https://wiki.archlinux.org/title/Zsh#pkgfile_"command_not_found"_handler)
- [macOS (Homebrew)](https://github.com/Homebrew/brew/blob/main/docs/Command-Not-Found.md)
- [Fedora](https://fedoraproject.org/wiki/Features/PackageKitCommandNotFound)
- [NixOS](https://github.com/NixOS/nixpkgs/tree/master/nixos/modules/programs/command-not-found)
- [Termux](https://github.com/termux/command-not-found)
- [SUSE](https://www.unix.com/man-page/suse/1/command-not-found/)
- [Gentoo](https://github.com/AndrewAmmerlaan/command-not-found-gentoo/tree/main)
- [Void Linux](https://codeberg.org/classabbyamp/xbps-command-not-found)
- [Alpine Linux](https://pkgs.alpinelinux.org/package/edge/main/x86_64/command-not-found)

你可以提交 Pull Request 来为其他平台添加支持。
