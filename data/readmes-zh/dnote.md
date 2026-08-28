# Dnote 插件

本插件为 [Dnote](https://www.getdnote.com/)(一个简洁的命令行笔记本)提供自动补全。

✅ 启用方式:把「dnote」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

在最基本的层面上,本插件可以补全所有 Dnote 命令。

```zsh
$ dnote a(press <TAB> here)
```

会得到:

```zsh
$ dnote add
```

对于某些命令,本插件还会动态提示匹配的笔记本(book)名称。

举例来说,如果你有三个以 'j' 开头的笔记本:'javascript'、'job'、'js',

```zsh
$ dnote view j(press <TAB> here)
```

会得到:

```zsh
$ dnote v j
javascript  job         js
```

再举一个例子,

```zsh
$ dnote edit ja(press <TAB> here)
```

会得到:


```zsh
$ dnote v javascript
``````
