# apache2-macports 插件

提供一组别名,用于控制通过 [MacPorts](https://www.macports.org/) 安装的本地 Apache2。

✅ 启用方式:把「apache2-macports」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名           | 函数                                    | 说明            |
|----------------|-----------------------------------------|-----------------|
| apache2restart | `sudo /path/to/apache2.wrapper restart` | 重启 apache 守护进程 |
| apache2start   | `sudo /path/to/apache2.wrapper start`   | 启动 apache 守护进程 |
| apache2stop    | `sudo /path/to/apache2.wrapper stop`    | 停止 apache 守护进程 |

## 贡献者

- Alexander Rinass (alex@rinass.net)
