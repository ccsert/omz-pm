# svn-fast-info 插件

主 SVN 插件实现的更快速替代方案。适用于 svn 1.6 及更新版本。
请作为 svn 插件的直接替代品(drop-in replacement)使用,而不是与之搭配使用。

✅ 启用方式:把「svn-fast-info」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

它更快的原因在于对 svn 的高效使用(单次 svn 调用),这在庞大的代码库上能节省大量开销。
它会显示本地文件的当前状态(已添加、已删除、已修改、已替换或其他状态)。

在你的主题中使用 `svn_prompt_info` 方法来显示 svn 仓库状态。

## 函数

- `svn_prompt_info`:显示与 svn 仓库状态有关的全部可用信息。

- `svn_repo_need_upgrade`:显示仓库是否需要升级。`svn_prompt_info` 会根据
  这个函数的结果来决定是否查询其余函数。

- `svn_current_branch_name`:显示当前分支。

- `svn_repo_root_name`:显示仓库根目录。

- `svn_current_revision`:显示当前检出的修订版本。

- `svn_status_info`:根据仓库中文件的状态显示一组符号。

## 选项

- `ZSH_THEME_SVN_PROMPT_PREFIX`:显示在提示符信息输出开头的序列。

- `ZSH_THEME_SVN_PROMPT_SUFFIX`:显示在提示符信息输出末尾的序列。

- `ZSH_THEME_SVN_PROMPT_CLEAN`:仓库状态干净时显示的序列。

- `ZSH_THEME_SVN_PROMPT_ADDITIONS`:仓库中存在新增文件时显示的序列。
  **默认值:** `+`。

- `ZSH_THEME_SVN_PROMPT_DELETIONS`:仓库中存在被删除文件时显示的序列。
  **默认值:** `✖`。

- `ZSH_THEME_SVN_PROMPT_MODIFICATIONS`:仓库中存在被修改文件时显示的序列。
  **默认值:** `✎`。

- `ZSH_THEME_SVN_PROMPT_REPLACEMENTS`:仓库中存在被替换文件时显示的序列。
  **默认值:** `∿`。

- `ZSH_THEME_SVN_PROMPT_UNTRACKED`:仓库中存在未跟踪文件时显示的序列。
  **默认值:** `?`。

- `ZSH_THEME_SVN_PROMPT_DIRTY`:仓库处于脏状态时显示的序列。
  **默认值:** `!`。
