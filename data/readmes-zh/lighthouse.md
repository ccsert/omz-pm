# Lighthouse 插件

本插件添加了用于管理 [Lighthouse](https://lighthouseapp.com/) 的命令。

✅ 启用方式:把「lighthouse」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 命令

* `open_lighthouse_ticket <issue>`(别名:`lho`):

  打开以参数传入的 issue 对应的 URL。使用前需在所在目录放一个 `.lighthouse-url` 文件,
  其中写入对应项目的 URL。

  示例:

  ```console
  $ cat .lighthouse-url
  https://rails.lighthouseapp.com/projects/8994

  $ lho 23
  Opening ticket #23
  # The browser goes to https://rails.lighthouseapp.com/projects/8994/tickets/23
  ```

观看演示:http://screencast.com/t/ZDgwNDUwNT
