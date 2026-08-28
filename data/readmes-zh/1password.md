# 1Password 插件

本插件为 oh-my-zsh 增加 1Password 功能。

✅ 启用方式:把「1password」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

之后,你就可以用 `opswd` 命令把各服务的密码复制到剪贴板。

## `opswd`

`opswd` 命令是对 `op` 命令的一层封装。它接受一个服务名作为参数,先复制该服务的用户名,经用户确认后再把密码复制到剪贴板。

如果该服务还存有 TOTP,同样会在用户确认后复制到剪贴板。最后,20 秒后剪贴板会被清空。

例如,`opswd github.com` 会先把你的 GitHub 用户名放进剪贴板;然后询问你是否继续,确认后把密码复制到剪贴板;最后,如果存在 TOTP,会在你确认后复制到剪贴板。

该函数支持补全,因此你可以用 Tab 补全来选择想要获取的服务。

> 注意:`opswd` 需要在已登录的状态下才能工作。如果你使用生物识别解锁,1Password CLI 会自动提示你登录。参见:
>
> - [1Password CLI 2 入门:登录](https://developer.1password.com/docs/cli/get-started#sign-in)
> - [手动登录你的 1Password 账户](https://developer.1password.com/docs/cli/sign-in-manually)

## 依赖要求

- [1Password CLI 2](https://developer.1password.com/docs/cli/get-started#install)

  > 注意:如果你用的是 1Password CLI 1,[请查看如何升级到 CLI 2](https://developer.1password.com/docs/cli/upgrade)。
