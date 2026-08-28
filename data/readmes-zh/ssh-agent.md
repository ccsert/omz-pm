# ssh-agent 插件

本插件会自动启动 `ssh-agent`,为你设置并加载 ssh 连接所需的任意凭据。

✅ 启用方式:把「ssh-agent」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 设置

**重要:这些设置必须放在加载 oh-my-zsh 的那一行_之前_**

### `agent-forwarding`

要启用 **agent 转发支持**,请把下面这行加入你的 zshrc 文件:

```zsh
zstyle :omz:plugins:ssh-agent agent-forwarding yes
```

### `helper`

要设置一个**外部 helper** 来询问密码并可能将其存入系统钥匙串,可使用 `helper` 样式。例如:

```zsh
zstyle :omz:plugins:ssh-agent helper ksshaskpass
```

### `identities`

要**加载多个身份**,请使用 `identities` 样式(**如果启用了 `lazy` 设置,此项将不起作用**)。例如:

```zsh
zstyle :omz:plugins:ssh-agent identities id_rsa id_rsa2 id_github
```

**注意:**如果身份文件不在 `~/.ssh` 下,可以使用绝对路径。例如:

```zsh
zstyle :omz:plugins:ssh-agent identities ~/.config/ssh/id_rsa ~/.config/ssh/id_rsa2 ~/.config/ssh/id_github
# which can be simplified to
zstyle :omz:plugins:ssh-agent identities ~/.config/ssh/{id_rsa,id_rsa2,id_github}
```

### `lazy`

要**在启动时不加载任何身份**,请使用 `lazy` 设置。它与 `AddKeysToAgent` 设置
(OpenSSH 7.2 起可用)配合使用时特别有用,
因为它允许你只在首次使用时输入密码。_注意:可以用 `ssh -V` 查看你的
OpenSSH 版本。_

```zsh
zstyle :omz:plugins:ssh-agent lazy yes
```

你可以通过给 `ssh` 命令传入 `-o AddKeysToAgent=yes` 来启用 `AddKeysToAgent`,
也可以在 `~/.ssh/config` 文件中加入 `AddKeysToAgent yes` [1]。
参见 [OpenSSH 7.2 发行说明](http://www.openssh.com/txt/release-7.2)。

### `lifetime`

要**设置身份的最长生存期**,请使用 `lifetime` 样式。
生存期可以用秒数指定,或按 sshd_config(5) 中描述的格式指定
(参见 _TIME FORMATS_)。如果不指定,默认生存期为永久。

```zsh
zstyle :omz:plugins:ssh-agent lifetime 4h
```

### `quiet`

要让插件保持安静,使用下面的设置:

```zsh
zstyle :omz:plugins:ssh-agent quiet yes
```

### `ssh-add-args`

要**给启动时添加身份的 `ssh-add` 命令传递参数**,请使用 `ssh-add-args` 设置。
可以用空格分隔传入多个参数:

```zsh
zstyle :omz:plugins:ssh-agent ssh-add-args -K -c -a /run/user/1000/ssh-auth
```

这些参数会按原样传给 `ssh-add` 调用。上面的例子会变成:

```zsh
ssh-add -K -c -a /run/user/1000/ssh-auth <identities>
```

要查看 `ssh-add` 的有效参数,可运行 `ssh-add --help` 或 `man ssh-add`。

### Powerline 10k 专属设置

Powerlevel10k 有一个即时提示(instant prompt)功能,不喜欢本插件向控制台写入内容。
如果你在用 p10k,建议采用下面的设置(用法见上文):

```
zstyle :omz:plugins:ssh-agent quiet yes
zstyle :omz:plugins:ssh-agent lazy yes
```

### macOS 专属设置

macOS 支持在向 ssh-agent 添加身份时使用存储在钥匙串中的密码短语。

```
ssh-add --apple-use-keychain ~/.ssh/id_rsa ...
```


本插件可以配置为加载时使用钥匙串,方法如下:

```
zstyle :omz:plugins:ssh-agent ssh-add-args --apple-load-keychain
```

## 致谢

基于 Joseph M. Reagle 的代码:https://www.cygwin.com/ml/cygwin/2001-06/msg00537.html

Agent 转发支持基于 Florent Thoumie 与 Jonas Pfenniger 的思路
