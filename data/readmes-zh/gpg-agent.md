# gpg-agent

针对 [GPG 的 gpg-agent](https://www.gnupg.org/documentation/manuals/gnupg/) 常见的一些问题提供若干修复。

具体来说,本插件会:

- 在每次 shell 执行前更新 `GPG_TTY` 环境变量。
- 在 `enable-ssh-support` 开启的情况下更新 `SSH_AUTH_SOCK` 环境变量。

✅ 启用方式:把「gpg-agent」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。
