# Vundle 插件

本插件添加了一些函数,用于控制 vim 的 [vundle](https://github.com/VundleVim/Vundle.vim) 插件管理器。

✅ 启用方式:把「vundle」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 函数

| 函数          | 用法            | 说明                                                                       |
|---------------|-----------------|----------------------------------------------------------------------------|
| vundle-init   | `vundle-init`   | 通过把 git 仓库克隆到 ~/.vim 文件夹来安装 vundle                           |
| vundle        | `vundle`        | 安装 .vimrc 中设置的插件(等价于 `:PluginInstall`)                         |
| vundle-update | `vundle-update` | 更新 .vimrc 中设置的插件(等价于 `:PluginInstall!`)                        |
| vundle-clean  | `vundle-clean`  | 删除已从 .vimrc 中移除的插件(等价于 `:PluginClean!`)                      |
