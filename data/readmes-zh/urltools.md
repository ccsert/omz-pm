# URLTools 插件

本插件提供两个别名,用于对字符串做 URL 编码和 URL 解码。

✅ 启用方式:把「urltools」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

原作者:[Ian Chesal](https://github.com/ianchesal)
原始创意与别名:[Ruslan Spivak](https://ruslanspivak.wordpress.com/2010/06/02/urlencode-and-urldecode-from-a-command-line/)

## 命令

| 命令        | 说明                         |
| :---------- | :--------------------------- |
| `urlencode` | 对给定字符串做 URL 编码      |
| `urldecode` | 对给定字符串做 URL 解码      |

## 示例

```zsh
urlencode 'https://github.com/ohmyzsh/ohmyzsh/search?q=urltools&type=Code'
# returns https%3A%2F%2Fgithub.com%2Fohmyzsh%2Fohmyzsh%2Fsearch%3Fq%3Durltools%26type%3DCode

urldecode 'https%3A%2F%2Fgithub.com%2Fohmyzsh%2Fohmyzsh%2Fsearch%3Fq%3Durltools%26type%3DCode'
# returns https://github.com/ohmyzsh/ohmyzsh/search?q=urltools&type=Code
```
