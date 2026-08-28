# colorize

借助本插件,你可以为 300 多种受支持语言以及其他文本格式的文件内容做语法高亮。

Colorize 会根据文件扩展名来决定高亮方式。如果找不到与给定扩展名对应的语法高亮方法,
它会尝试通过查看文件内容来寻找合适的方法。如果依然找不到任何高亮方法,
它就会像普通的 `cat` 一样直接输出文件,不做语法高亮。

## 设置

✅ 启用方式:把「colorize」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 配置

### 依赖要求

本插件要求至少安装了以下工具之一:

* [Chroma](https://github.com/alecthomas/chroma)
* [Pygments](https://pygments.org/download/)

### Colorize 工具

Colorize 支持 `pygmentize` 和 `chroma` 两种语法高亮工具。默认情况下,colorize 优先使用 `pygmentize`,除非它未安装而 `chroma` 已安装。可以通过 `ZSH_COLORIZE_TOOL` 环境变量覆盖这一默认行为:

```
ZSH_COLORIZE_TOOL=chroma
```

### 样式

Pygments 提供多种样式。默认使用 `default` 样式,你可以通过设置 `ZSH_COLORIZE_STYLE` 环境变量来选择其他主题:

```
ZSH_COLORIZE_STYLE="colorful"
```

### Chroma Formatter 设置

Chroma 支持 8 色、256 色和真彩色(true-color)的终端输出。如果你需要把默认的终端输出样式从标准的 8 色输出改掉,请设置 `ZSH_COLORIZE_CHROMA_FORMATTER` 环境变量:

```
ZSH_COLORIZE_CHROMA_FORMATTER=terminal256
```

## 用法

* `ccat <file> [files]`:为文件内容着色(如果提供了多个文件,则处理所有文件)。
  如果没有传入文件,则对标准输入着色。

* `cless [less-options] <file> [files]`:为文件内容着色(如果提供了多个文件,则处理所有文件),并用 less 打开。
  如果没有传入文件,则对标准输入着色。
  为此,LESSOPEN 和 LESSCLOSE 会被覆盖,但只会在局部作用域内生效。
