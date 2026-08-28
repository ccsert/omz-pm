# Sprunge 插件

本插件把数据上传到 pastebin http://sprunge.us 并取回 URL。

✅ 启用方式:把「sprunge」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

| 命令                         | 说明                                   |
| ---------------------------- | -------------------------------------- |
| `sprunge filename.txt`       | 上传 filename.txt                      |
| `sprunge "this is a string"` | 上传纯文本                             |
| `sprunge < filename.txt`     | 把 filename.txt 的内容重定向给 sprunge |
| `echo data \| sprunge`       | 任何通过管道传入的数据都会被上传       |

sprunge 处理完输入后,会给你一个唯一的 HTTP 地址:

```console
$ sprunge "hello"
http://sprunge.us/XxjnKz
```

## 注意

- sprunge 接受管道数据、stdin 重定向、文本字符串或文件名作为输入。
  同一时刻只能使用其中一种。
- 参数优先级如下:stdin > 管道输入 > 文本字符串。
- 如果文件名拼错或缺少必要的路径信息,它不会报错,而是把它当作文本字符串处理。

## 致谢

- 原始代码:[shellperson.net](https://web.archive.org/web/20190910065842/https://www.shellperson.net/sprunge-pastebin-script/)。
- 改编者:Matt Parnell(@ilikenwf)。
