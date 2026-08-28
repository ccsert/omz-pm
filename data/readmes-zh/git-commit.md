# git-commit 插件

git-commit 插件为 [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/#summary) 风格的提交消息添加了若干
[git 别名](https://www.git-scm.com/docs/git-config#Documentation/git-config.txt-alias)。

✅ 启用方式:把「git-commit」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 语法

```zsh
git <type> [(-s, --scope) "<scope>"] [(-a, --attention)] "<message>"
```

其中 `type` 是下列之一:

- `build`
- `chore`
- `ci`
- `docs`
- `feat`
- `fix`
- `perf`
- `refactor`
- `rev`
- `style`
- `test`
- `wip`

> 注意:`revert` 类型的别名是 `rev`,否则它会与同名的 git 命令冲突。
> 它仍会生成格式为 `revert: <message>` 的提交消息

> ⚠️ 启用本插件会(有可能)覆盖你手动设置的所有 `alias.<type>`。请
> 谨慎使用!

## 示例

| Git 别名                                      | 命令                                                 |
| --------------------------------------------- | ---------------------------------------------------- |
| `git style "remove trailing whitespace"`      | `git commit -m "style: remove trailing whitespace"`  |
| `git wip "work in progress"`                  | `git commit -m "work in progress"`                   |
| `git fix -s "router" "correct redirect link"` | `git commit -m "fix(router): correct redirect link"` |
| `git rev -s "api" "rollback v2"`              | `git commit -m "revert(api): rollback v2"`           |
