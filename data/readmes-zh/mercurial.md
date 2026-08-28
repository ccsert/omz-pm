# Mercurial 插件

本插件为使用 Mercurial 提供了一些顺手的别名,以及若干可在主题中使用的工具函数和提示符函数。

✅ 启用方式:把「mercurial」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名    | 命令                                        |
| ------- | ------------------------------------------- |
| `hga`   | `hg add`                                    |
| `hgc`   | `hg commit`                                 |
| `hgca`  | `hg commit --amend`                         |
| `hgci`  | `hg commit --interactive`                   |
| `hgb`   | `hg branch`                                 |
| `hgba`  | `hg branches`                               |
| `hgbk`  | `hg bookmarks`                              |
| `hgco`  | `hg checkout`                               |
| `hgd`   | `hg diff`                                   |
| `hged`  | `hg diffmerge`                              |
| `hgp`   | `hg push`                                   |
| `hgs`   | `hg status`                                 |
| `hgsl`  | `hg log --limit 20 --template "<template>"` |
| `hgun`  | `hg resolve --list`                         |
| `hgi`   | `hg incoming`                               |
| `hgl`   | `hg pull -u`                                |
| `hglr`  | `hg pull --rebase`                          |
| `hgo`   | `hg outgoing`                               |
| `hglg`  | `hg log --stat -v`                          |
| `hglgp` | `hg log --stat -p -v`                       |

## 提示符用法

- 切换到使用了 `hg_prompt_info` 的主题

- 或者自定义当前主题的 `$PROMPT` 变量,让它包含当前目录的 mercurial 仓库信息。
  具体做法:把主题的自定义版本放进 `$ZSH_CUSTOM`,或者在加载主题之后在
  `.zshrc` 中修改 `$PROMPT`。

  例如,对于 `robbyrussell` 主题,你需要修改 `$PROMPT` 变量,在 `$(git_prompt_info)` 后面加上 `$(hg_prompt_info)`,使其形如:

  ```zsh
  PROMPT='${ret_status}%{$fg_bold[green]%}%p %{$fg[cyan]%}%c %{$fg_bold[blue]%}$(git_prompt_info)$(hg_prompt_info)%{$fg_bold[blue]%} % %{$reset_color%}'
  ```

你还可以重新定义插件用到的其他变量(需在 Oh My Zsh 被加载之后):

```zsh
ZSH_THEME_HG_PROMPT_PREFIX="%{$fg_bold[magenta]%}hg:(%{$fg[red]%}"
ZSH_THEME_HG_PROMPT_SUFFIX="%{$reset_color%}"
ZSH_THEME_HG_PROMPT_DIRTY="%{$fg[magenta]%}) %{$fg[yellow]%}✗%{$reset_color%}"
ZSH_THEME_HG_PROMPT_CLEAN="%{$fg[magenta]%})"
```

### 在提示符中显示仓库分支和目录状态

这与 git 插件的做法相同。**注意**:要让此功能生效,还需要对 `.zshrc` 做额外修改,
或使用专为 `hg_prompt_info` 设计的主题。

## 维护者

- [ptrv](https://github.com/ptrv):原作者
- [oshybystyi](https://github.com/oshybystyi)
