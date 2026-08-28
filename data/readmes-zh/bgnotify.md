# bgnotify zsh 插件

为长时间运行的命令提供跨平台的后台通知!支持 OSX 和 Linux。

独立项目主页:[t413/zsh-background-notify](https://github.com/t413/zsh-background-notify)

---

## 使用方法

✅ 启用方式:把「bgnotify」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

- 在 OS X 上,你需要 [terminal-notifier](https://github.com/alloy/terminal-notifier)
  * `brew install terminal-notifier`(或 `gem install terminal-notifier`)
- 在 Linux 上,确保已安装 `notify-send` 或 `kdialog`。如果你用的是 Ubuntu,那么已经万事俱备!
- 在 Windows 上,你可以使用 [notifu](https://www.paralint.com/projects/notifu/) 或 Cygwin Ports 的 libnotify 包


## 截图

**Linux**

![screenshot from 2014-11-07 15 58 36](https://cloud.githubusercontent.com/assets/326829/4962187/256b465c-66da-11e4-927d-cc2fc105e31f.png)

**OS X**

![screenshot 2014-11-08 14 15 12](https://cloud.githubusercontent.com/assets/326829/4965780/19fa3eac-6795-11e4-8ed6-0355711123a9.png)

**Windows**

![screenshot from 2014-11-07 15 55 00](https://cloud.githubusercontent.com/assets/326829/4962159/a2625ca0-66d9-11e4-9e91-c5834913190e.png)


## 配置

可以配置的内容有:

- `bgnotify_bell` 启用或禁用终端铃声(默认为 true)
- `bgnotify_threshold` 设置通知的触发阈值时间(默认 6 秒)
- `function bgnotify_formatted` 允许你自定义通知内容。比如可以定制消息并传入一个图标。
- `bgnotify_extraargs` 向通知器追加额外参数(例如给 notify-send 加上 `-e` 来创建瞬时通知)

要使用它们,请在你的 source 调用之前添加相应的函数定义。示例:

```sh
bgnotify_bell=false   ## disable terminal bell
bgnotify_threshold=4  ## set your own notification threshold

function bgnotify_formatted {
  ## $1=exit_status, $2=command, $3=elapsed_time

  # Humanly readable elapsed time
  local elapsed="$(( $3 % 60 ))s"
  (( $3 < 60 ))   || elapsed="$((( $3 % 3600) / 60 ))m $elapsed"
  (( $3 < 3600 )) || elapsed="$((  $3 / 3600 ))h $elapsed"

  [ $1 -eq 0 ] && title="Holy Smokes Batman" || title="Holy Graf Zeppelin"
  [ $1 -eq 0 ] && icon="$HOME/icons/success.png" || icon="$HOME/icons/fail.png"
  bgnotify "$title - took ${elapsed}" "$2" "$icon"
}

plugins=(git bgnotify)  ## add to plugins list
source $ZSH/oh-my-zsh.sh  ## existing source call
```
