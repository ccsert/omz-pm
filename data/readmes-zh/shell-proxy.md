# shell-proxy 插件

这是一个纯用户态程序——shell-proxy 设置器,用 Python3 和 Zsh 编写。

✅ 启用方式:把「shell-proxy」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 主要特性

- 支持 macOS 和 Linux(Ubuntu、Archlinux 等)
- 通过设置 `$GIT_SSH` 支持 git
- 通过设置别名支持 ssh、sftp、scp、slogin 和 ssh-copy-id
- 内置自动补全

## 用法

### 方法 1

把 `SHELLPROXY_URL` 环境变量设置为代理服务器的 URL:

```sh
SHELLPROXY_URL="http://127.0.0.1:8123"
SHELLPROXY_NO_PROXY="localhost,127.0.0.1"
proxy enable
```

### 方法 2

在 `$HOME/.config/proxy` 写一个程序文件,使代理 URL 可以动态定义。
注意该程序文件必须是可执行的。

示例:

```sh
#!/bin/bash

# HTTP Proxy
if [[ "$(uname)" = Darwin ]]; then
  echo "http://127.0.0.1:6152" # Surge Mac
else
  echo "http://127.0.0.1:8123" # polipo
fi

# No Proxy
echo "localhost,127.0.0.1"
```

### 方法 3

使用[方法 2](#method-2),但通过设置 `SHELLPROXY_CONFIG` 环境变量来指定程序文件的位置:

```sh
SHELLPROXY_CONFIG="$HOME/.dotfiles/proxy-config"
```

## 参考资料

- `$GIT_SSH`: <https://www.git-scm.com/docs/git#Documentation/git.txt-codeGITSSHcode>
- OpenSSH 手册: <https://man.openbsd.org/ssh>

## 维护者

- [@septs](https://github.com/septs)
