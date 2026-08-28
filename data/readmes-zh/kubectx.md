# kubectx - 显示当前活动的 kubectl context

本插件添加了 `kubectx_prompt_info()` 函数,用于显示当前活动的
kubectl context 名称(`kubectl config current-context`)。

你可以用它来定制提示符,随时知道自己是不是正处在生产集群上 ;)

✅ 启用方式:把「kubectx」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

### 用法

添加到 **.zshrc**:

```zsh
# right prompt
RPS1='$(kubectx_prompt_info)'
# left prompt
PROMPT="$PROMPT"'$(kubectx_prompt_info)'
```

在受支持的 zsh 版本上,context 会异步加载,这样 `kubectl` 就不会阻塞提示符。
如果想恢复同步行为,请在加载 Oh My Zsh 之前添加:

```zsh
zstyle ':omz:alpha:plugins:kubectx' async-prompt no
```

如果你的主题是通过另一个函数间接调用 `kubectx_prompt_info` 的,请改为强制注册异步处理器:

```zsh
zstyle ':omz:alpha:plugins:kubectx' async-prompt force
```

### 自定义 context 名称

你可以重命名默认的 context 名称,以获得更好的可读性或附加的格式化效果。
这些值接受 [prompt expansion 序列](http://zsh.sourceforge.net/Doc/Release/Prompt-Expansion.html),
例如 `%F{color}`、`%f`、`%K{color}`、`%k`、`%B`、`%b`、`%U`、`%u`、`%S`、`%s`、`%{...%}`。

**示例**:把下面的内容添加到你的 .zshrc 文件中:

```zsh
kubectx_mapping[minikube]="mini"
kubectx_mapping[context_name_from_kubeconfig]="$emoji[wolf_face]"
kubectx_mapping[production_cluster]="%{$fg[yellow]%}prod!%{$reset_color%}"
# contexts with spaces
kubectx_mapping[context\ with\ spaces]="%F{red}spaces%f"
# don't use quotes as it will break the prompt
kubectx_mapping["context with spaces"]="%F{red}spaces%f" # ti
```

你也可以一次性定义整个映射数组:

```zsh
typeset -A kubectx_mapping
kubectx_mapping=(
  minikube                      "mini"
  context_name_from_kubeconfig  "$emoji[wolf_face]"
  production_cluster            "%{$fg[yellow]%}prod!%{$reset_color%}"
  "context with spaces"         "%F{red}spaces%f"
)
```

![staging](stage.png)
![production](prod.png)
