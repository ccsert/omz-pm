# `svn` 插件

本插件添加了一些实用函数,用于显示你的当前 svn 仓库的额外信息。完整的 svn 文档参见 https://subversion.apache.org/ 。

✅ 启用方式:把「svn」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 函数

| 命令                  | 说明                                        |
|:----------------------|:--------------------------------------------|
| `svn_prompt_info`     | 在主题中显示 svn 提示符                     |
| `in_svn`              | 检查当前是否处于 svn 仓库中                 |
| `svn_get_repo_name`   | 获取仓库名称                                |
| `svn_get_branch_name` | 获取分支名称(见[注意事项](#caveats))       |
| `svn_get_rev_nr`      | 获取修订版本号                              |
| `svn_dirty`           | 检查 svn 仓库中是否有未提交的变更           |

## 注意事项

插件期望路径的第一个目录就是当前的分支 / 标签 / trunk。因此如果你不使用分支,它会返回第一个路径元素。

## 在主题中的用法

要在 `agnoster` 主题中使用,请按照以下说明操作:

1. 启用 svn 插件

2. 把以下几行加入你的 `zshrc` 文件:

    ```shell
    prompt_svn() {
        local rev branch
        if in_svn; then
            rev=$(svn_get_rev_nr)
            branch=$(svn_get_branch_name)
            if [[ $(svn_dirty_choose_pwd 1 0) -eq 1 ]]; then
                prompt_segment yellow black
                echo -n "$rev@$branch"
                echo -n "±"
            else
                prompt_segment green black
                echo -n "$rev@$branch"
            fi
        fi
    }
    ```

3. 覆盖 agnoster 的 `build_prompt()` 函数:

    ```zsh
    build_prompt() {
        RETVAL=$?
        prompt_status
        prompt_context
        prompt_dir
        prompt_git
        prompt_svn
        prompt_end
    }
    ```

