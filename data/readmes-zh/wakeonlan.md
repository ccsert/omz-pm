# wakeonlan

本插件对 "wakeonlan" 工具做了一个封装。该工具可在大多数发行版的软件仓库中找到,
也可以从[以下网站](https://github.com/jpoliv/wakeonlan)获取。

✅ 启用方式:把「wakeonlan」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

要使用这个封装,请先创建 `~/.wakeonlan` 目录,并在该目录中为每个你希望能够唤醒的设备
放一个文件。给文件起一个能描述该设备的名字,比如它的主机名。每个文件应包含一行,
内容为目标设备的 MAC 地址和网络广播地址。

例如,可能存在一个 ~/.wakeonlan/leto 文件,内容如下:

```
00:11:22:33:44:55:66 192.168.0.255
```

要唤醒该设备,使用以下命令:

```console
$ wake leto
```

可用的设备名会被自动补全,因此:

```console
$ wake <tab>
```

……会给出 "leto" 的建议,以及放置在 ~/.wakeonlan 目录中的其他所有配置文件。

关于配置文件格式的更多信息,请查看 wakeonlan 的 man 手册页。
