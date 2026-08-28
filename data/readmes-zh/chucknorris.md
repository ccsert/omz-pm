# chucknorris

Oh My Zsh 的 Chuck Norris fortune 插件,非常适合用作 MOTD(每日消息)。

✅ 启用方式:把「chucknorris」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

| 命令        | 说明                           |
| ----------- | ------------------------------ |
| `chuck`     | 随机打印一条 Chuck Norris 语录 |
| `chuck_cow` | 以 cowthink 的形式打印语录     |

示例:`chuck_cow` 的输出:

```
Last login: Fri Jan 30 23:12:26 on ttys001
 ______________________________________
( When Chuck Norris plays Monopoly, it )
( affects the actual world economy.    )
 --------------------------------------
        o   ^__^
         o  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

## 依赖要求

- `fortune`
- `cowsay`(如果使用 `chuck_cow` 则需要)

可通过 homebrew、apt 等包管理器安装。

> [!NOTE]  
> 除了安装 `fortune` 之外,可能还需要运行:
> 
> `strfile $ZSH/plugins/chucknorris/fortunes/chucknorris\n`
> 
> (把 "\n" 按字面原样输入),以便把 fortune 数据写入正确的目录。
