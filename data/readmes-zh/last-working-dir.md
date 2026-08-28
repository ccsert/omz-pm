# last-working-dir 插件

记录最近使用的工作目录,并在新 shell 启动时自动跳转到该目录,
除非起始目录不是 `$HOME`。

它还添加了一个 `lwd` 函数,用于跳转到最近的工作目录。

✅ 启用方式:把「last-working-dir」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 特性

### 为不同的 SSH 密钥使用不同的 last-working-dir 文件

如果同一个用户账户被多个通过不同 SSH 密钥连接的用户使用,你可以配置 SSH 把它们映射为不同的
`SSH_USER`,插件就会为每个用户使用单独的 lwd 文件。

请确保你的 SSH 服务器允许使用环境变量。你可以在 `/etc/sshd/sshd_config` 文件中启用该特性:

```
PermitUserEnvironment yes
```

然后,在 `authorized_keys` 文件里的 SSH 密钥前面加上 `environment="SSH_USER=<SSH_USERNAME>"`:

```
environment="SSH_USER=a.test@example.com" ssh-ed25519 AAAAC3Nz...
```
