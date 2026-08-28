# Xcode

本插件提供一些实用工具,帮助你日常使用 Xcode 和进行 iOS 开发。

✅ 启用方式:把「xcode」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。


## 别名

| 别名  | 说明                                     | 命令                                           |
|-------|------------------------------------------|------------------------------------------------|
| xcb   | 构建 Xcode 项目和工作区                  | xcodebuild                                     |
| xcdd  | 清除所有临时构建信息                     | rm -rf ~/Library/Developer/Xcode/DerivedData/* |
| xcp   | 显示当前选定的 Xcode 目录                | xcode-select --print-path                      |
| xcsel | 按路径选择不同的 Xcode 目录              | sudo xcode-select --switch                     |
| xx    | 在 Xcode 中打开所列文件                  | open -a "Xcode.app"                            |


## 函数

###  `xc`

在 Xcode 中把当前目录作为 Xcode 项目或 Swift 包打开。它会打开当前工作目录中能找到的 `.xcworkspace`、`.xcodeproj`、`.swiftpm` 和 `Package.swift` 文件之一。你也可以指定一个目录,让它在其中查找 Xcode 文件。
如果没找到任何相关文件,则返回 1。

###  `xx`

在 Xcode 中打开所列文件,多个文件会在多文件浏览器中打开。

###  `simulator`

从命令行打开 iOS 模拟器,取决于当前哪一个是 Xcode 的活动开发者目录。(也就是说,它会遵循 `xcsel` 的设置。)

### `xcselv`

按版本名称选择不同的 Xcode 安装。与 `xcsel` 类似,区别在于它只接受版本名作为参数,而不是 Xcode 安装的完整路径。使用下文描述的命名约定。

* `xcselv <version>` 选择某个版本
 * 示例:`xcselv 6.2`
* `xcselv default` 选择默认的无版本号 `Applications/Xcode.app`
* 不带参数的 `xcselv` 以易读格式列出可用的 Xcode 版本
* `xcselv -l` 列出已安装的 Xcode 版本
* `xcselv -L` 以只含版本名的简短格式列出已安装的 Xcode 版本
* `xcselv -p` 打印当前活动 Xcode 版本的信息
* `xcselv -h` 打印帮助信息

`xcselv` 的选项解析比较简单。选项不能组合使用,且只识别第一个选项。

## 多版本 Xcode

`xcselv` 命令支持仅凭版本号在不同 Xcode 安装之间切换。不同的 Xcode 版本通过文件命名约定来识别。

### 带版本的 Xcode 命名约定

对于管理带版本 Xcode 安装的命名约定或其他组织机制,Apple 似乎既没有明确定义,也没有提供工具支持。Apple 过去发布的 beta 版本似乎既有 `Xcode<version>.app` 风格的名字,也有 `Xcode-<version>.app` 风格的名字,这两种风格都出现在论坛和博客的讨论中。

我们采用了以下命名约定:

* 带版本的 Xcode 安装以 `Xcode-<version>` 或 `Xcode<version>` 这样的名字标识。
* 分隔 `"Xcode"` 与版本名的 `-` 是可选的,也可以换成空格。
* 带版本的名字既可以用在 `Xcode.app` 本身上,也可以用在 `Applications/` 下包含它的子目录上。
* 不能同时在 `Xcode.app` 文件名本身和其所在子文件夹上标版本。
* 因此,以下各种写法完全等价。
 * `Applications/Xcode-<version>.app`
 * `Applications/Xcode-<version>/Xcode.app`
 * `Applications/Xcode<version>.app`
 * `Applications/Xcode <version>.app`
 * `Applications/Xcode <version>/Xcode.app`
* 系统 `/Applications/` 和用户 `$HOME/Applications/` 目录都会被搜索。
 * 对同一个版本,用户 `$HOME/Applications/` 优先于 `/Applications`。
 * 如果同一个 `Applications/` 文件夹中的多个命名变体指向同一版本(例如 `Xcode-3.2.1.app`、`Xcode3.2.1.app` 和 `Xcode-3.2.1/Xcode.app`),则优先顺序未定义,取决于具体实现。
* `<version>` 可以是文件名中合法的任意字符串。
* 特殊版本名 `"default"` 指 `Applications/Xcode.app`(位于 `/Applications/` 或 `$HOME/Applications/`)处「默认」的无版本号 Xcode。
* 版本名不能以 ``"-"`` 或空白字符开头。

这套命名约定上的限制将来可能需要收紧。特别是,如果还有其他知名应用程序的名字以字符串 `"Xcode"` 开头,可能就需要限制 `<version>` 允许使用的字符串,以免与其他应用冲突。如果有证据表明其中某种命名方式在实践中或被 Apple 明显偏好,我们也可能收紧命名约定来偏向它。

## 注意事项

用 `xcsel` 或 `xcselv` 选择一个安装在你 `$HOME` 下的 Xcode,可能会影响其他用户的正常使用,具体取决于你的系统设置。尽管如此我们仍允许你这样做,因为有些人实际上是以单用户方式运行 OS X,或者权限开放到这样也能正常工作。你也可以改用 `$DEVELOPER_DIR` 来代替 `xcsel`,它只作用于当前用户或会话,而不是全局设置。

该功能不会校验 Xcode 文件名中的版本名与该二进制文件的实际版本是否一致。把名字弄对是用户自己的责任。
