# Jira 插件

本插件提供一组命令行工具,用于与 Atlassian 的 [JIRA](https://www.atlassian.com/software/jira)
缺陷跟踪软件交互。

✅ 启用方式:把「jira」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

所有交互都通过 Web 完成,无需在本地安装 JIRA。

在本文档中,「JIRA」指 JIRA 问题跟踪服务器,而 `jira` 指本插件提供的命令。

## 用法

本插件只提供一个命令 `jira`,它的全部功能都通过该命令暴露。这个命令的大多数形式都会在你的
Web 浏览器中打开一个 JIRA 页面。

## 命令

`jira help` 或 `jira usage` 会打印下面的用法说明

| 命令                          | 说明                                                     |
| :---------------------------- | :------------------------------------------------------- |
| `jira`                        | 执行默认操作                                              |
| `jira new`                    | 打开新建 Jira 问题的对话框                                 |
| `jira ABC-123`                | 打开一个已有问题                                           |
| `jira ABC-123 m`              | 打开一个已有问题以便添加评论                                |
| `jira project ABC`            | 打开 JIRA 项目摘要                                         |
| `jira dashboard [rapid_view]` | 打开你的 JIRA 仪表板                                       |
| `jira mine`                   | 查询你自己的问题                                           |
| `jira tempo`                  | 打开你的 JIRA Tempo                                        |
| `jira reported [username]`    | 查询某用户报告的问题                                       |
| `jira assigned [username]`    | 查询分配给某用户的问题                                     |
| `jira branch`                 | 打开与当前分支名匹配的已有问题                              |
| `jira help`                   | 打印用法说明                                               |


### Jira Branch 使用说明

分支名可能带有以「/」结尾的前缀,如「feature/MP-1234」;也可能带有以「_」开头的后缀,
如「MP-1234_fix_dashboard」。在这两种情况下,打开的问题都是「MP-1234」。

它还会检查名称中是否带有前缀,没有的话会自动补上,因此:「MP-1234」会打开问题「MP-1234」,
「mp-1234」会打开问题「mp-1234」,而「1234」会打开问题「MP-1234」。

如果你的分支命名规范与此不同,可以自己重写 jira_branch 函数来解析并输出 Jira 问题键。
在你的 `.zshrc` 中、加载 `oh-my-zsh.sh` 之后定义一个 `jira_branch` 函数。
示例:
```zsh
# Determine branch name from naming convention 'type/KEY-123/description'.
function jira_branch() {
  # Get name of the branch
  issue_arg=$(git rev-parse --abbrev-ref HEAD)
  # Strip prefixes like feature/ or bugfix/
  issue_arg=${issue_arg#*/}
  # Strip suffixes like /some-branch-description
  issue_arg=${issue_arg%%/*}
  # Return the value
  echo $issue_arg
}
```


#### 调试用法

这些调用形式供开发者使用,随时可能变化。

```
jira dumpconfig   # displays the effective configuration
```

## 设置

JIRA 实例的 URL 由 `$JIRA_URL` 或一个 `.jira_url` 文件设置。

在项目根目录放一个 `.jira-url` 文件。你也可以在 `~/.zshrc` 中设置 `$JIRA_URL`,
或者在家目录放一个 `.jira-url`。当前目录下的 `.jira-url` 优先级最高,因此可以针对每个项目做定制。

`.jira-prefix` 和 `$JIRA_PREFIX` 的用法相同。它们控制加到所有问题 ID 前面的前缀,
用于区分同一个 JIRA 实例中的不同项目。

例如:

```
cd to/my/project
echo "https://jira.atlassian.com" >> .jira-url
```

(注意:当前实现只在当前目录查找 `.jira-url` 和 `.jira-prefix`,不会向上级目录查找,
所以如果你在项目的子目录里,它会回退到你默认的 JIRA URL。不过这一点将来很可能会改变。)

### 变量

* `$JIRA_URL` - 你的 JIRA 实例的 URL
* `$JIRA_NAME` - 你的 JIRA 用户名;用作 `assigned`/`reported` 搜索的默认用户
* `$JIRA_PREFIX` - 加在问题 ID 参数前面的前缀
* `$JIRA_RAPID_BOARD` - 如果你使用 Rapid Board,设为 `true`
* `$JIRA_RAPID_VIEW` - 设置默认的 rapid view;若 `$JIRA_RAPID_BOARD` 设为 false 则无效
* `$JIRA_DEFAULT_ACTION` - 不带参数调用 `jira` 时执行的操作;默认为「new」
* `$JIRA_TEMPO_PATH` - 你的 JIRA Tempo URL 路径;默认为「/secure/Tempo.jspa」


### 浏览器

与 JIRA 实例交互时使用的是你的默认 Web 浏览器,具体由 `open_command` 处理 `http://`
URL 的方式决定。如果你修改了系统的 URL 处理程序关联,`jira` 使用的浏览器也会随之改变。
