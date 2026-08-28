# npm 插件

npm 插件提供补全,并添加了许多实用的别名。

✅ 启用方式:把「npm」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名    | 命令                         | 说明                                                            |
|:------  |:-----------------------------|:----------------------------------------------------------------|
| `npmg`  | `npm i -g`                   | 全局安装依赖                                                     |
| `npmS`  | `npm i -S`                   | 安装并保存到 package.json 的 dependencies 中                     |
| `npmD`  | `npm i -D`                   | 安装并保存到 package.json 的 dev-dependencies 中                 |
| `npmF`  | `npm i -f`                   | 忽略本地缓存,强制从远程 registry 安装                            |
| `npmE`  | `PATH="$(npm bin)":"$PATH"`  | 基于当前目录,从 node_modules 文件夹运行命令                      |
| `npmO`  | `npm outdated`               | 检查哪些 npm 模块已过时                                          |
| `npmU`  | `npm update`                 | 把列出的所有包更新到最新版本                                     |
| `npmV`  | `npm -v`                     | 查看包版本                                                       |
| `npmL`  | `npm list`                   | 列出已安装的包                                                   |
| `npmL0` | `npm ls --depth=0`           | 列出顶层的已安装包                                               |
| `npmst` | `npm start`                  | 运行 npm start                                                   |
| `npmt`  | `npm test`                   | 运行 npm test                                                    |
| `npmR`  | `npm run`                    | 运行 npm 脚本                                                    |
| `npmP`  | `npm publish`                | 运行 npm publish                                                 |
| `npmI`  | `npm init`                   | 运行 npm init                                                    |
| `npmi`  | `npm info`                   | 运行 npm info                                                    |
| `npmSe` | `npm search`                 | 运行 npm search                                                  |
| `npmrd` | `npm run dev`                | 运行 npm run dev                                                 |
| `npmrb` | `npm run build`              | 运行 npm run build                                               |

## `npm install` / `npm uninstall` 切换

本插件添加了一个函数,可以在当前命令或最近的命令中,于 `npm install` 与 `npm uninstall` 之间切换,最多可作用到前 2 条命令。**默认键位是连按两次 <kbd>F2</kbd>**。

你可以在 zshrc 文件中加入以下几行来更改该键位:

```zsh
bindkey -M emacs '<seq>' npm_toggle_install_uninstall
bindkey -M vicmd '<seq>' npm_toggle_install_uninstall
bindkey -M viins '<seq>' npm_toggle_install_uninstall
```

其中 `<seq>` 是按键序列:先运行 `cat`,再按下你想要使用的组合键,即可得到。
