# wd

[![Build Status](https://github.com/mfaerevaag/wd/actions/workflows/test.yml/badge.svg)](https://github.com/mfaerevaag/wd/actions)

`wd`(*warp directory*,传送目录)让你在 zsh 中无需 `cd` 即可跳转到自定义目录。
为什么?
因为当某个文件夹访问频繁或路径很长时,`cd` 显得效率不高。

![Demo](https://raw.githubusercontent.com/mfaerevaag/wd/master/tty.gif)

## 安装

### [oh-my-zsh](https://github.com/ohmyzsh/ohmyzsh)

`wd` 已随 oh-my-zsh 自带!

✅ 启用方式:把「wd」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

### [Antigen](https://github.com/zsh-users/antigen)

在你的 `.zshrc` 中:

```zsh
antigen bundle mfaerevaag/wd
```

### [Antibody](https://github.com/getantibody/antibody)

在你的 `.zshrc` 中:

```zsh
antibody bundle mfaerevaag/wd
```

### [Fig](https://fig.io)

在这里安装 `wd`:[![Fig plugin store](https://fig.io/badges/install-with-fig.svg)](https://fig.io/plugins/other/wd_mfaerevaag)

### Arch ([AUR](https://aur.archlinux.org/packages/zsh-plugin-wd-git/))

1. 从 AUR 安装

```zsh
yay -S zsh-plugin-wd-git
# or use any other AUR helper
```

2. 然后加入你的 `.zshrc`:

```zsh
wd() {
    . /usr/share/wd/wd.sh
}
```

### [Home Manager](https://github.com/nix-community/home-manager)

把以下内容加入你的 `home.nix`,然后运行 `home-manager switch`:

```nix
programs.zsh.plugins = [
  {
    name = "wd";
    src = pkgs.zsh-wd;
    file = "share/wd/wd.plugin.zsh";
    completions = [ "share/zsh/site-functions" ];
  }
];
```

### [zplug](https://github.com/zplug/zplug)

```zsh
zplug "mfaerevaag/wd", as:command, use:"wd.sh", hook-load:"wd() { . $ZPLUG_REPOS/mfaerevaag/wd/wd.sh }"
```

### 自动安装

_注意:自动安装不提供 manpage。而且,在不先审查的情况下运行远程代码并非良好的安全实践,所以你应当先看看[这里](https://github.com/mfaerevaag/wd/blob/master/install.sh)_

在终端中运行以下任一命令:

```zsh
curl -L https://github.com/mfaerevaag/wd/raw/master/install.sh | sh
```

或

```zsh
wget --no-check-certificate https://github.com/mfaerevaag/wd/raw/master/install.sh -O - | sh
```

### 手动安装

1. 在本机上把该仓库克隆到一个合适的位置(当然,如果你清楚自己在做什么,这一切都由你决定):

```zsh
git clone git@github.com:mfaerevaag/wd.git ~/.local/wd --depth 1
```

2. 把 `wd` 函数加入 `.zshrc`(或 `.profile` 等):

```zsh
wd() {
    . ~/.local/wd/wd.sh
}
```

3. 安装 manpage(可选):

把 manpage 移动到合适的目录,然后触发 `mandb` 让它被发现

```zsh
sudo install -m 644 ~/.local/wd/wd.1 /usr/share/man/man1/wd.1
sudo mandb /usr/share/man/man1
```

**注意:** 拉取并更新 `wd` 时,如果 manpage 有变动,你需要重复第 3 步

## 补全

如果你_没有_使用 [oh-my-zsh](https://github.com/robbyrussell/oh-my-zsh),但又想使用 zsh 补全功能,还需要把你安装 `wd` 的路径(如果用的是自动安装脚本,则为 `~/bin/wd`)加入你的 `fpath`。
例如在你的 `~/.zshrc` 中:

```zsh
fpath=(~/path/to/wd $fpath)
```

另外,你可能还需要运行以下命令,强制重建 `zcompdump`:

```zsh
rm -f ~/.zcompdump; compinit
```

## 浏览

`wd` 自带一个由 `fzf` 驱动的浏览功能,可对所有 warp 点进行模糊搜索。通过 `wd browse` 命令可用。为了快速访问,你可以在 `.zshrc` 中设置一个别名或键位绑定:

```zsh
# ctrl-b to open the fzf browser
bindkey ${FZF_WD_BINDKEY:-'^B'} wd_browse_widget
```

## 用法

* 为当前工作目录添加 warp 点:

```zsh
wd add foo
```

如果同名 warp 点已存在,可用 `wd add foo --force` 覆盖它。

**注意:** warp 点不能包含冒号,也不能只由空格和点组成。
前者会与 `wd` 存储 warp 点的方式冲突,后者会与其他功能冲突,如下所述。

* 为任意目录添加 warp 点(使用默认名称):

```zsh
wd addcd /foo/ bar
```

* 为任意目录添加 warp 点(自定义名称):

```zsh
wd addcd /foo/
```


你可以省略点名称,自动改用当前目录的名称。

* 在任意目录下,用以下方式传送到 `foo`:

```zsh
wd foo
```

* 你还可以(带自动补全地)传送到 `foo` 内的某个目录:

```zsh
wd foo some/inner/path
```

* 你可以用下面的点号语法,传送回上一个目录乃至更上层:

```zsh
wd ..
wd ...
```

这是对 zsh `dirs` 函数的一层封装。  
_如果你没有使用 [oh-my-zsh](https://github.com/ohmyzsh/ohmyzsh),可能需要在 `.zshrc` 中加入 `setopt AUTO_PUSHD`。_

* 删除 warp 点:

```zsh
wd rm foo
```

你可以省略点名称,改用当前目录的名称。

* 列出所有 warp 点(默认存储在 `~/.warprc`):

```zsh
wd list
```

* 列出给定 warp 点中的文件:

```zsh
wd ls foo
```

* 显示给定 warp 点的路径:

```zsh
wd path foo
```

* 列出指向当前目录的 warp 点,或者(可选地)显示指向给定 warp 点的路径:

```zsh
wd show
```

* 删除指向不存在目录的 warp 点。

```zsh
wd clean
```

使用 `wd clean --force` 可免于确认提示。

* 打印用法信息:

```zsh
wd help
```

不带命令直接调用 `wd` 时也会打印用法信息

* 打印所运行的 `wd` 版本:

```zsh
wd --version
```

* 显式指定配置文件(默认为 `~/.warprc`),这对测试很有用:

```zsh
wd --config ./file <command>
```

* 静默所有输出:

```zsh
wd --quiet <command>
```

## 配置

你可以用以下环境变量配置 `wd`:

### `WD_CONFIG`

定义 warp 点的存储路径。默认为 `$HOME/.warprc`。

## 测试

`wd` 自带一个小型测试套件,使用 [shunit2](https://github.com/kward/shunit2) 运行。它可以用来确认在你的环境中一切按预期工作,或用来演示某个问题。

要运行,只需 `cd` 进 `test` 目录并运行 `tests.sh`。

```zsh
cd ./test
./tests.sh
```

## 维护者

在 @mfaerevaag 退出本仓库的活跃维护工作之后,以下用户现在也是本仓库的维护者:

* @alpha-tango-kilo

* @MattLewin

其他任何人的贡献都非常受欢迎,并将在发布说明中被提及!

---

感谢 [altschuler](https://github.com/altschuler) 提出的绝妙主意。

希望你喜欢!
