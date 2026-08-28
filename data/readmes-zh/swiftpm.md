# Swift Package Manager

本插件提供了一些实用工具,让你在使用 [Swift Package Manager](https://github.com/apple/swift-package-manager) 的日常工作中更加高效,
并为 Swift 5.9 提供自动补全。

✅ 启用方式:把「swiftpm」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名  | 说明                                 | 命令                                |
| ----- | ------------------------------------ | ----------------------------------- |
| `spi` | 初始化一个新包                       | `swift package init`                |
| `spf` | 拉取包依赖                           | `swift package fetch`               |
| `spu` | 更新包依赖                           | `swift package update`              |
| `spx` | 生成一个 Xcode 工程                  | `swift package generate-xcodeproj`  |
| `sps` | 打印已解析的依赖关系图               | `swift package show-dependencies`   |
| `spd` | 把解析后的 Package.swift 打印为 JSON | `swift package dump-package`        |
