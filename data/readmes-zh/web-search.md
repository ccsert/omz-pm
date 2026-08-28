# web-search 插件

本插件提供使用 Google、Wiki、Bing、YouTube 及其他热门服务进行搜索的别名。

✅ 启用方式:把「web-search」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

你可以用以下两种形式使用 `web-search` 插件:

- `web_search <context> <term> [more terms if you want]`
- `<context> <term> [more terms if you want]`

例如,下面两条命令是等价的:

```zsh
$ web_search google oh-my-zsh
$ google oh-my-zsh
```

可用的搜索上下文如下:

| 上下文                | URL                                             |
| --------------------- | ----------------------------------------------- |
| `bing`                | `https://www.bing.com/search?q=`                |
| `google`              | `https://www.google.com/search?q=`              |
| `brs` or `brave`      | `https://search.brave.com/search?q=`            |
| `yahoo`               | `https://search.yahoo.com/search?p=`            |
| `ddg` or `duckduckgo` | `https://www.duckduckgo.com/?q=`                |
| `sp` or `startpage`   | `https://www.startpage.com/do/search?q=`        |
| `yandex`              | `https://yandex.ru/yandsearch?text=`            |
| `github`              | `https://github.com/search?q=`                  |
| `baidu`               | `https://www.baidu.com/s?wd=`                   |
| `ecosia`              | `https://www.ecosia.org/search?q=`              |
| `goodreads`           | `https://www.goodreads.com/search?q=`           |
| `qwant`               | `https://www.qwant.com/?q=`                     |
| `givero`              | `https://www.givero.com/search?q=`              |
| `stackoverflow`       | `https://stackoverflow.com/search?q=`           |
| `wolframalpha`        | `https://wolframalpha.com/input?i=`             |
| `archive`             | `https://web.archive.org/web/*/`                |
| `scholar`             | `https://scholar.google.com/scholar?q=`         |
| `ask`                 | `https://www.ask.com/web?q=`                    |
| `youtube`             | `https://www.youtube.com/results?search_query=` |
| `deepl`               | `https://www.deepl.com/translator#auto/auto/`   |
| `dockerhub`           | `https://hub.docker.com/search?q=`              |
| `gems`                | `https://rubygems.org/search?query=`            |
| `npmpkg`              | `https://www.npmjs.com/search?q=`               |
| `packagist`           | `https://packagist.org/?query=`                 |
| `gopkg`               | `https://pkg.go.dev/search?m=package&q=`        |
| `chatgpt`             | `https://chatgpt.com/?q=`                       |
| `claudeai`            | `https://claude.ai/new?q=`                      |
| `grokcom`             | `https://grok.com/?q=`                          |
| `reddit`              | `https://www.reddit.com/search/?q=`             |
| `ppai`                | `https://www.perplexity.ai/search/new?q=`       |
| `rscrate`             | `https://crates.io/search?q=`                   |
| `rsdoc`               | `https://docs.rs/releases/search?query=`        |

另外还有用于 DuckDuckGo bang 搜索的别名:

| 上下文  | Bang |
| ------- | ---- |
| `wiki`  | `!w` |
| `news`  | `!n` |
| `map`   | `!m` |
| `image` | `!i` |
| `ducky` | `!`  |

### 自定义搜索引擎

如果你想给插件添加其他搜索上下文,可以使用 `$ZSH_WEB_SEARCH_ENGINES` 变量。
在加载 Oh My Zsh 之前设置它,格式如下:

```zsh
ZSH_WEB_SEARCH_ENGINES=(
    <context> <URL>
    <context> <URL>
)
```

其中 `<context>` 是搜索上下文的名称,`<URL>` 则是与上文搜索上下文同类的 URL。
例如,要添加 `reddit`,你可以这样写:

```zsh
ZSH_WEB_SEARCH_ENGINES=(reddit "https://www.reddit.com/search/?q=")
```

这些自定义搜索引擎同样会被转换成别名,因此你既可以执行 `web_search reddit <查询词>`,
也可以直接执行 `reddit <查询词>`。
