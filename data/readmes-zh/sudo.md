# sudo

按两次 <kbd>esc</kbd>,即可轻松给当前或上一条命令加上 `sudo` 前缀。

✅ 启用方式:把「sudo」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

### 给当前输入的命令补 sudo

假设你已经敲了一条很长的命令,却忘了在前面加 `sudo`:

```console
$ apt-get install build-essential
```

连按两次 <kbd>esc</kbd> 键,同一条命令就会自动带上 `sudo` 前缀,不用重新输入:

```console
$ sudo apt-get install build-essential
```

用默认编辑器编辑文件时也一样(编辑器按 `$SUDO_EDITOR`、`$VISUAL`、`$EDITOR` 的顺序取第一个非空者):

假设默认编辑器是 `vim`:

```console
$ vim /etc/hosts
```

连按两次 <kbd>esc</kbd> 键,命令会变成 `sudo -e` 而不是原来的编辑器——它将以 root 权限用该编辑器打开文件:

```console
$ sudo -e /etc/hosts
```

### 给上一条执行过的命令补 sudo

假设你想删除一个系统文件但被拒绝了:

```console
$ rm some-system-file.txt
-su: some-system-file.txt: Permission denied
$
```

连按两次 <kbd>esc</kbd> 键,同一条命令就会自动带上 `sudo` 前缀,不用重新输入:

```console
$ rm some-system-file.txt
-su: some-system-file.txt: Permission denied
$ sudo rm some-system-file.txt
Password:
$
```

编辑文件被拒时同理,如前所述。

## 键位绑定

默认情况下,`sudo` 插件使用 <kbd>Esc</kbd><kbd>Esc</kbd> 作为触发键。
如果想改键,可以用 `bindkey` 命令把它绑定到别的按键:

```sh
bindkey -M emacs '<seq>' sudo-command-line
bindkey -M vicmd '<seq>' sudo-command-line
bindkey -M viins '<seq>' sudo-command-line
```

其中 `<seq>` 是你想用的按键序列。先运行 `cat`,再按下你想用的组合键,就能查到对应的键盘序列。
