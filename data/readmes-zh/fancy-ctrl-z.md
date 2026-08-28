# fancy-ctrl-z

让你可以再按一次 Ctrl-Z,切换回刚才切到后台的任务。

✅ 启用方式:把「fancy-ctrl-z」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 动机

我经常需要在 shell 里执行一些临时命令。为此我会按 Ctrl-z 暂停 Vim,输入命令,
再按 fg<Enter> 切回 Vim。fg 这一步实在让我难受。我只想再按一次 Ctrl-z 就能回到
Vim。我找不到现成的解决方案,于是自己动手做了一个,它在 ZSH 下工作得非常好。

来源:http://sheerun.net/2014/03/21/how-to-boost-your-vim-productivity/

致谢:
- 原始创意来自 @sheerun
- 由 @mbologna 加入 OMZ
