## 简介

> 让前端 Web 开发的搜索变得更轻松

## 安装

✅ 启用方式:把「frontend-search」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

frontend-search 插件有两种使用形式:

- `frontend <context> <term> [more terms if you want]`
- `<context> <term> [more terms if you want]`

例如,下面两种写法是等价的:

```zsh
$ angular dependency injection
# Will turn into ...
$ frontend angular dependency injection
```

可用的搜索上下文如下:

| 上下文        | URL                                                                         |
| ------------- | --------------------------------------------------------------------------- |
| angular       | `https://angular.io/?search=`                                               |
| angularjs     | `https://google.com/search?as_sitesearch=angularjs.org&as_q=`               |
| bem           | `https://google.com/search?as_sitesearch=bem.info&as_q=`                    |
| bootsnipp     | `https://bootsnipp.com/search?q=`                                           |
| bundlephobia  | `https://bundlephobia.com/result?p=`                                        |
| caniuse       | `https://caniuse.com/#search=`                                              |
| codepen       | `https://codepen.io/search?q=`                                              |
| compassdoc    | `http://compass-style.org/search?q=`                                        |
| cssflow       | `http://www.cssflow.com/search?q=`                                          |
| dartlang      | `https://api.dartlang.org/apidocs/channels/stable/dartdoc-viewer/dart:`     |
| emberjs       | `https://www.google.com/search?as_sitesearch=emberjs.com/&as_q=`            |
| flowtype      | `https://google.com/search?as_sitesearch=flow.org/en/docs/&as_q=`           |
| fontello      | `http://fontello.com/#search=`                                              |
| github        | `https://github.com/search?q=`                                              |
| html5please   | `https://html5please.com/#`                                                 |
| jestjs        | `https://www.google.com/search?as_sitesearch=jestjs.io&as_q=`               |
| jquery        | `https://api.jquery.com/?s=`                                                |
| lodash        | `https://devdocs.io/lodash/index#`                                          |
| mdn           | `https://developer.mozilla.org/search?q=`                                   |
| nodejs        | `https://www.google.com/search?as_sitesearch=nodejs.org/en/docs/&as_q=`     |
| npmjs         | `https://www.npmjs.com/search?q=`                                           |
| packagephobia | `https://packagephobia.now.sh/result?p=`                                    |
| qunit         | `https://api.qunitjs.com/?s=`                                               |
| reactjs       | `https://google.com/search?as_sitesearch=facebook.github.io/react&as_q=`    |
| smacss        | `https://google.com/search?as_sitesearch=smacss.com&as_q=`                  |
| stackoverflow | `https://stackoverflow.com/search?q=`                                       |
| typescript    | `https://google.com/search?as_sitesearch=www.typescriptlang.org/docs&as_q=` |
| unheap        | `http://www.unheap.com/?s=`                                                 |
| vuejs         | `https://www.google.com/search?as_sitesearch=vuejs.org&as_q=`               |
| nextjs        | `https://www.google.com/search?as_sitesearch=nextjs.org&as_q=`              |

如果你想要别的搜索上下文,欢迎开一个 Issue 告诉我们!

## 回退搜索行为

如果某个搜索上下文对应的文档站点没有搜索功能,插件会回退使用 Google。你可以在
`~/.zshrc` 文件中、Oh My Zsh 被加载之前设置
`FRONTEND_SEARCH_FALLBACK='duckduckgo'`,把回退搜索引擎改为 DuckDuckGo。

## DuckDuckGo 手气搜索

启用 DuckDuckGo 的「ducky」(手气)搜索功能,即可自动访问排名第一的搜索结果。该功能
针对 DuckDuckGo 做了优化,因为 Google 会先跳转到一个中间页面。FRONTEND_SEARCH_FALLBACK_LUCKY
环境变量会触发使用 DuckDuckGo 的手气搜索,在这种情况下就不再需要 FRONTEND_SEARCH_FALLBACK
设置了。

## 作者

**Wilson Mendes (willmendesneto)**

- <https://twitter.com/willmendesneto>
- <https://github.com/willmendesneto>
