# emotty 插件

本插件根据当前 $TTY 编号返回一个 emoji,以便在提示符(prompt)中使用。

✅ 启用方式:把「emotty」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

**注意:** 它依赖 [emoji 插件](https://github.com/ohmyzsh/ohmyzsh/tree/master/plugins/emoji)。

## 用法

`emotty` 函数会基于与 `$TTY` 关联的编号,从当前字符集(默认:`emoji`)中显示一个 emoji。

有多套 emoji 字符集可供选择。想换一套的话,把 `$emotty_set` 设成你想用的那套的名称,例如:
```
emotty_set=nature
```

### 字符集

- emoji
- loral
- love
- nature
- stellar
- zodiac

用 `display_emotty` 函数可以列出当前字符集(或以第一个参数传入的字符集)中的所有 emoji。例如:

```
$ display_emotty zodiac
<list of all the emojis in the zodiac character set>
```
