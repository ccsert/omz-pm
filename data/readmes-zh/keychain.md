# keychain 插件

本插件会自动启动 [`keychain`](https://www.funtoo.org/Keychain),
为你设置并加载 gpg 与 ssh 连接所需的任意凭据。

✅ 启用方式:把「keychain」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

**注意**:强烈建议同时启用 `gpg-agent` 插件。

## 使用说明

**重要:请把这些设置放在 source oh-my-zsh 那一行_之前_**

要**调整 keychain 所管理的 agent**,请按下文所示使用 `agents` 样式。
默认只管理 `gpg` 这一个 agent。

```zsh
zstyle :omz:plugins:keychain agents gpg,ssh
```

要**加载多个身份**,请使用 `identities` 样式,例如:

```zsh
zstyle :omz:plugins:keychain identities id_ed25519 id_github 2C5879C2
```

要**向 `keychain` 程序传递额外选项**,请使用 `options` 样式,例如:

```zsh
zstyle :omz:plugins:keychain options --quiet
```

## 致谢

基于 `ssh-agent` 插件的代码。

## 参考资料

- [Keychain](https://www.funtoo.org/Keychain)
