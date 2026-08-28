# Ionic 插件

本插件为 [Ionic CLI](https://ionicframework.com/docs/cli) 提供自动补全,
并为常用的 Ionic 命令提供了一些别名。

✅ 启用方式:把「ionic」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名  | 命令                                 | 说明                                                             |
|-------|--------------------------------------|------------------------------------------------------------------|
| iv    | `ionic --version`                    | 查看 Ionic 版本                                                  |
| ih    | `ionic --help`                       | Ionic 帮助命令                                                   |
| ist   | `ionic start`                        | 创建新项目                                                       |
| ii    | `ionic info`                         | 打印系统/环境信息                                                |
| is    | `ionic serve`                        | 启动本地开发服务器,用于应用开发/测试                            |
| icba  | `ionic cordova build android`        | 构建 web 资源,并为 android 平台目标做好准备                     |
| icbi  | `ionic cordova build ios`            | 构建 web 资源,并为 ios 平台目标做好准备                         |
| icra  | `ionic cordova run android`          | 在已连接的 android 设备上运行 Ionic 项目                         |
| icri  | `ionic cordova run ios`              | 在已连接的 ios 设备上运行 Ionic 项目                             |
| icrsa | `ionic cordova resources android`    | 自动创建 android 的图标和启动画面资源                            |
| icrsi | `ionic cordova resources ios`        | 自动创建 ios 的图标和启动画面资源                                |
| icpaa | `ionic cordova platform add android` | 添加 Cordova android 平台目标                                    |
| icpai | `ionic cordova platform add ios`     | 添加 Cordova ios 平台目标                                        |
| icpra | `ionic cordova platform rm android`  | 移除 Cordova 平台目标                                            |
| icpri | `ionic cordova platform rm ios`      | 移除 Cordova 平台目标                                            |
