# git lfs 插件

git lfs 插件为 [git-lfs](https://github.com/git-lfs/git-lfs) 提供[别名](#aliases)和[函数](#functions)。

✅ 启用方式:把「git-lfs」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名     | 命令                                |
| :------- | :---------------------------------- |
| `glfsi`  | `git lfs install`                   |
| `glfst`  | `git lfs track`                     |
| `glfsls` | `git lfs ls-files`                  |
| `glfsmi` | `git lfs migrate import --include=` |

## 函数

| 函数    | 命令                                            |
| :------- | :---------------------------------------------- |
| `gplfs`  | `git lfs push origin "$(current_branch)" --all` |
