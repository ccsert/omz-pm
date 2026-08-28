# Yarn 插件

本插件为 [Yarn 包管理器](https://yarnpkg.com/en/)提供自动补全,
以及一些常用 Yarn 命令的别名。

✅ 启用方式:把「yarn」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 全局 scripts 目录

本插件还会把 `yarn` 的全局 scripts 目录(通常是 `~/.yarn/bin`)加入 `$PATH`。
要禁用这一特性,请在 `.zshrc` 中设置如下 style:

```zsh
zstyle ':omz:plugins:yarn' global-path no
```

## Yarn Berry

如果你的全局 Yarn 版本是 Yarn berry(即 Yarn 2 或更高版本),应当配置本插件,
使其别名相应调整,请在 `.zshrc` 中设置如下 style:

```zsh
zstyle ':omz:plugins:yarn' berry yes
```

## 别名

- 标有 <sup>`*`</sup> 的别名仅在使用 Yarn v1(非 berry)时可用
- 标有 <sup>`b`</sup> 的别名仅在使用 Yarn berry 时可用

| 别名               | 命令                                                                                                  | 说明                                                                               |
| ------------------ | ----------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| y                  | `yarn`                                                                                                | Yarn 命令                                                                          |
| ya                 | `yarn add`                                                                                            | 把包安装进 dependencies(`package.json`)                                            |
| yad                | `yarn add --dev`                                                                                      | 把包安装进 devDependencies(`package.json`)                                         |
| yap                | `yarn add --peer`                                                                                     | 把包安装进 peerDependencies(`package.json`)                                        |
| yb                 | `yarn build`                                                                                          | 运行 `package.json` 中定义的 build 脚本                                             |
| ycc                | `yarn cache clean`                                                                                    | 清理 yarn 的全局包缓存                                                              |
| yd                 | `yarn dev`                                                                                            | 运行 `package.json` 中定义的 dev 脚本                                               |
| yf                 | `yarn format`                                                                                         | 运行 `package.json` 中定义的 dev 脚本                                               |
| yh                 | `yarn help`                                                                                           | 显示某个 yarn 命令的帮助                                                            |
| yi                 | `yarn init`                                                                                           | 以交互方式创建或更新 package.json 文件                                              |
| yin                | `yarn install`                                                                                        | 安装 `package.json` 中定义的依赖                                                    |
| yln                | `yarn lint`                                                                                           | 运行 `package.json` 中定义的 lint 脚本                                              |
| ylnf               | `yarn lint --fix`                                                                                     | 运行 `package.json` 中定义的 lint 脚本,自动修复问题                                 |
| yp                 | `yarn pack`                                                                                           | 创建包依赖的 gzip 压缩归档                                                          |
| yrm                | `yarn remove`                                                                                         | 移除已安装的包                                                                      |
| yrun               | `yarn run`                                                                                            | 运行一个已定义的包脚本                                                              |
| ys                 | `yarn serve`                                                                                          | 启动开发服务器                                                                      |
| yst                | `yarn start`                                                                                          | 运行 `package.json` 中定义的 start 脚本                                             |
| yt                 | `yarn test`                                                                                           | 运行 `package.json` 中定义的 test 脚本                                              |
| ytc                | `yarn test --coverage`                                                                                | 运行 `package.json` 中定义的 test 脚本并统计覆盖率                                   |
| yui                | `yarn upgrade-interactive`                                                                            | 交互式询问要升级哪些过期的包                                                        |
| yuil               | `yarn upgrade-interactive --latest` (or see `yui` when using [yarn berry](#yarn-berry))               | 交互式询问要把哪些过期的包升级到最新的可用版本                                      |
| yii                | `yarn install --frozen-lockfile` (or `yarn install --immutable` when using [yarn berry](#yarn-berry)) | 安装依赖;如果 lockfile 将被修改,则中止安装                                          |
| yifl               | `yii`                                                                                                 | 安装依赖;如果 lockfile 将被修改,则中止安装                                          |
| yup                | `yarn upgrade`                                                                                        | 把包升级到最新版本                                                                  |
| yv                 | `yarn version`                                                                                        | 更新你的包的版本号                                                                  |
| yw                 | `yarn workspace`                                                                                      | 在单个 workspace 中运行一条命令。                                                    |
| yws                | `yarn workspaces`                                                                                     | 在所有已定义的 workspace 中运行一条命令。                                            |
| yy                 | `yarn why`                                                                                            | 显示某个包为何被安装,并详细列出哪些其他包依赖它                                     |
| yga<sup>`*`</sup>  | `yarn global add`                                                                                     | 在你的操作系统上全局安装包                                                          |
| ygls<sup>`*`</sup> | `yarn global list`                                                                                    | 列出全局安装的包                                                                    |
| ygrm<sup>`*`</sup> | `yarn global remove`                                                                                  | 从你的操作系统中移除全局安装的包                                                    |
| ygu<sup>`*`</sup>  | `yarn global upgrade`                                                                                 | 把全局安装的包升级到最新版本                                                        |
| yls<sup>`*`</sup>  | `yarn list`                                                                                           | 列出已安装的包                                                                      |
| yout<sup>`*`</sup> | `yarn outdated`                                                                                       | 检查过期的包依赖                                                                    |
| yuca<sup>`*`</sup> | `yarn global upgrade && yarn cache clean`                                                             | 升级全局包并清理 yarn 的全局缓存                                                    |
| ydlx<sup>`b`</sup> | `yarn dlx`                                                                                            | 在临时环境中运行一个包。                                                            |
| yn<sup>`b`</sup>   | `yarn node`                                                                                           | 在已设置好 hook 的状态下运行 node。                                                  |
