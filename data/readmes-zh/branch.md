# Branch 插件

本插件能快速显示当前的 Git 或 Mercurial 分支;如果在 Mercurial 仓库中,还会显示当前的书签(bookmark),如果存在的话。

✅ 启用方式:把「branch」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 速度测试

- `hg branch`:

  ```console
  $ time hg branch
  0.11s user 0.14s system 70% cpu 0.355 total
  ```

- branch 插件:

  ```console
  $ time zsh /tmp/branch_prompt_info_test.zsh
  0.00s user 0.01s system 78% cpu 0.014 total
  ```

## 用法

把你的主题复制到 `$ZSH_CUSTOM/themes/`,然后修改它,在提示符中加入 `$(branch_prompt_info)`。
下面这个示例针对 `robbyrussell` 主题:

```diff
diff --git a/themes/robbyrussell.zsh-theme b/themes/robbyrussell.zsh-theme
index 2fd5f2cd..9d89a464 100644
--- a/themes/robbyrussell.zsh-theme
+++ b/themes/robbyrussell.zsh-theme
@@ -1,5 +1,5 @@
 PROMPT="%(?:%{$fg_bold[green]%}➜ :%{$fg_bold[red]%}➜ )"
-PROMPT+=' %{$fg[cyan]%}%c%{$reset_color%} $(git_prompt_info)'
+PROMPT+=' %{$fg[cyan]%}%c%{$reset_color%} $(branch_prompt_info)'

 ZSH_THEME_GIT_PROMPT_PREFIX="%{$fg_bold[blue]%}git:(%{$fg[red]%}"
 ZSH_THEME_GIT_PROMPT_SUFFIX="%{$reset_color%} "
```

## 维护者

Victor Torres (<vpaivatorres@gmail.com>)
