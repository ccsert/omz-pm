# Octozen 插件

在启动时显示一条来自 GitHub Octocat 的禅语(zen quote)。

✅ 启用方式:把「octozen」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

它定义了 `display_octozen` 函数,用于获取一条 GitHub Octocat 禅语。
注意:需要联网(如果 2 秒内没有获取到就会超时)。
