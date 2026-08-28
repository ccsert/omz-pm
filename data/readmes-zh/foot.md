# foot

本插件为 [foot——一款快速、轻量且极简的 Wayland 终端模拟器](https://codeberg.org/dnkl/foot)提供 shell 集成。

✅ 启用方式:把「foot」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 在当前工作目录中启动新终端实例

启动新终端实例时(默认为 `ctrl+shift+n`),新实例将在当前工作目录中启动。

## 在提示符之间跳转

Foot 可以移动当前视口,聚焦到已执行命令的提示符(默认绑定为 ctrl+shift+z / ctrl+shift+x)。

## 管道传输上一条命令的输出

键位绑定 `pipe-command-output` 可以把上一条命令的输出通过管道传给你指定的应用程序
(类似于其他 `pipe-*` 键位绑定):

```
[key-bindings]
pipe-command-output=[sh -c "f=$(mktemp); cat - > $f; footclient emacsclient -nw $f; rm $f"] Control+Shift+g
```

按下 ctrl+shift+g 时,上一条命令的输出会写入一个临时文件,然后在一个新的 footclient 实例中启动 emacsclient。
footclient 实例关闭后,该临时文件会被删除。
