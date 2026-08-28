# Coffeescript 插件

本插件提供一组别名,用于快速编译和预览你的 coffeescript 代码。

编写 Coffeescript 时,经常需要预览某段代码的输出:可能是想测试输出结果,
也可能是想在只认 JavaScript、不认 Coffeescript 的浏览器控制台里执行它。

可以用 `cf "code"` 预览 coffeescript 的编译结果,如下所示:

```zsh
$ cf 'if a then b else c'
if (a) {
  b;
} else {
  c;
}
```

本插件还提供以下别名:

* **cfc:** 把编译后的 JS 复制到剪贴板。当你想在 JS 控制台里运行代码时非常好用。

* **cfp:** 从当前剪贴板内容编译。当你想编译较长或多行的代码片段时很有用。

* **cfpc:** 从剪贴板粘贴 coffeescript,编译成 JS,再把结果复制回剪贴板。
