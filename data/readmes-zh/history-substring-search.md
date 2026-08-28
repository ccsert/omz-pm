# zsh-history-substring-search

这是 [Fish shell][1] 历史搜索功能的净室(clean-room)实现:你可以输入历史命令的
任意片段,然后按下选定的按键(例如上、下方向键),在匹配结果之间循环切换。

[1]: http://fishshell.com
[2]: http://www.zsh.org/mla/users/2009/msg00818.html
[3]: http://sourceforge.net/projects/fizsh/
[4]: https://github.com/robbyrussell/oh-my-zsh/pull/215
[5]: https://github.com/zsh-users/zsh-history-substring-search
[6]: https://github.com/zsh-users/zsh-syntax-highlighting


依赖要求
------------------------------------------------------------------------------

* [ZSH](http://zsh.sourceforge.net) 4.3 或更新版本

安装
------------------------------------------------------------------------------

使用 [Homebrew](https://brew.sh) 包管理器:

    brew install zsh-history-substring-search
    echo 'source $(brew --prefix)/share/zsh-history-substring-search/zsh-history-substring-search.zsh' >> ~/.zshrc

使用 [Fig](https://fig.io):

Fig 能为你现有的终端添加应用、快捷方式和自动补全功能。

只需一键,即可安装 `zsh-history-substring-search`。

<a href="https://fig.io/plugins/other/zsh-history-substring-search" target="_blank"><img src="https://fig.io/badges/install-with-fig.svg" /></a>

使用 [Oh-my-zsh](https://github.com/robbyrussell/oh-my-zsh):

1. 把本仓库克隆到 oh-my-zsh 的插件目录:

        git clone https://github.com/zsh-users/zsh-history-substring-search ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-history-substring-search

2. 在 `~/.zshrc` 中启用该插件:

        plugins=( [plugins...] zsh-history-substring-search)

3. 运行 `exec zsh` 使更改生效:

        exec zsh

使用 [zplug](https://github.com/zplug/zplug):

1. 把本仓库添加到 `~/.zshrc`:

        zplug "zsh-users/zsh-history-substring-search", as: plugin

使用 [antigen](https://github.com/zsh-users/antigen):

1. 把 `antigen bundle` 命令加在 `antigen apply` 之前,像这样:

``` 
antigen bundle zsh-users/zsh-history-substring-search
antigen apply
```
 
2. 然后,在 `antigen apply` **之后**添加键位绑定配置,像这样:
 
```
# zsh-history-substring-search configuration
bindkey '^[[A' history-substring-search-up # or '\eOA'
bindkey '^[[B' history-substring-search-down # or '\eOB'
HISTORY_SUBSTRING_SEARCH_ENSURE_UNIQUE=1
```

使用 [Zinit](https://github.com/zdharma-continuum/zinit):

1. 在 `~/.zshrc` 中使用 `Oh-my-zsh` 这个 Zinit snippet:

        zinit snippet OMZ::plugins/git/git.plugin.zsh`

2. 在 `~/.zshrc` 中加载该插件:

        zinit load 'zsh-users/zsh-history-substring-search'
        zinit ice wait atload'_history_substring_search_config'

3. 运行 `exec zsh` 使更改生效:

        exec zsh

用法
------------------------------------------------------------------------------

1.  把本脚本加载进你的交互式 ZSH 会话:

        source zsh-history-substring-search.zsh

    如果你想配合 [zsh-syntax-highlighting][6] 一起使用本脚本,请确保_先_加载它、
    再加载本脚本:

        source zsh-syntax-highlighting.zsh
        source zsh-history-substring-search.zsh

2.  为本脚本的函数绑定快捷键。

    用户通常会把上、下方向键绑定给本脚本,步骤如下:
    * 在你喜欢的终端模拟器里运行 `cat -v`,观察按键产生的键码。
      (**注意:** 某些情况下,`cat -v` 显示的键码是错的。如果 `cat -v` 显示的
      键码在你那里不管用,可以在 ZSH 命令行提示符下按 `<C-v><UP>` 和
      `<C-v><DOWN>`,以获得正确的键码。)
    * 按下上方向键,观察终端里打印出了什么。
    * 按下下方向键,观察终端里打印出了什么。
    * 同时按下 Control 和 C 键,结束 `cat -v`。
    * 根据前面几步观察到的内容创建键位绑定。
      例如,如果你观察到 UP 键是 `^[[A`、DOWN 键是 `^[[B`,那就这样写:

          bindkey '^[[A' history-substring-search-up
          bindkey '^[[B' history-substring-search-down

      不过,如果观察到的值不管用,可以试试 terminfo:

          bindkey "$terminfo[kcuu1]" history-substring-search-up
          bindkey "$terminfo[kcud1]" history-substring-search-down

      也有用户发现 `[OA` 和 `[OB` 才是正确的取值,
      _即使_它们并不是观察到的值。如果你用观察到的值遇到问题,不妨试试这两个。

      你可能还想绑定 Control-P/N 键,供 EMACS 模式使用:

          bindkey -M emacs '^P' history-substring-search-up
          bindkey -M emacs '^N' history-substring-search-down

      你可能还想绑定 `k` 和 `j` 键,供 VI 模式使用:

          bindkey -M vicmd 'k' history-substring-search-up
          bindkey -M vicmd 'j' history-substring-search-down

3.  输入任意一条历史命令的任意片段,然后:

    * 按下上述第 2 步中配置的 `history-substring-search-up` 键,选中命令历史中
      距离最近的一条命令,它(1)包含你输入的查询内容,并且(2)比当前命令更早。

    * 按下上述第 2 步中配置的 `history-substring-search-down` 键,选中命令历史中
      距离最近的一条命令,它(1)包含你输入的查询内容,并且(2)比当前命令更新。

    * 同时按下 `^U`(Control 和 U 键)中止搜索。

4.  如果匹配到的命令跨了多行文本,先按左方向键把光标从命令末尾移开,然后:

    * 按下上述第 2 步中配置的 `history-substring-search-up` 键,把光标移到
      当前行上面的那一行。当光标到达命令的第一行时,再按一次
      `history-substring-search-up` 键,本脚本就会重新执行一次搜索。

    * 按下上述第 2 步中配置的 `history-substring-search-down` 键,把光标移到
      当前行下面的那一行。当光标到达命令的最后一行时,再按一次上述第 2 步中
      配置的 `history-substring-search-down` 键,本脚本就会重新执行一次搜索。


配置
------------------------------------------------------------------------------

本脚本定义了以下全局变量。你可以覆盖它们的默认值。

* `HISTORY_SUBSTRING_SEARCH_HIGHLIGHT_FOUND` 是一个全局变量,定义查询内容在匹配
  命令内部应当如何高亮。其默认值会让本脚本以洋红色背景上的白色粗体文字进行高亮。
  可赋给该变量的取值类型,请查阅 zshzle(1) 手册页中的 "Character Highlighting" 一节。

* `HISTORY_SUBSTRING_SEARCH_HIGHLIGHT_NOT_FOUND` 是一个全局变量,定义历史中没有任何
  命令匹配查询内容时应当如何高亮。其默认值会让本脚本以红色背景上的白色粗体文字
  进行高亮。可赋给该变量的取值类型,请查阅 zshzle(1) 手册页中的
  "Character Highlighting" 一节。

* `HISTORY_SUBSTRING_SEARCH_GLOBBING_FLAGS` 是一个全局变量,定义如何在命令历史中
  搜索你的查询内容。其默认值会让本脚本执行大小写不敏感的搜索。可赋给该变量的
  取值类型,请查阅 zshexpn(1) 手册页中的 "Globbing Flags" 一节。

* `HISTORY_SUBSTRING_SEARCH_FUZZY` 是一个全局变量,定义如何在命令历史中搜索你的
  查询内容。如果设为非空值,本脚本会按词进行模糊搜索,并按给定顺序匹配,
  例如 `ab c` 会匹配 `*ab*c*`。

* `HISTORY_SUBSTRING_SEARCH_PREFIXED` 是一个全局变量,定义如何在命令历史中搜索你的
  查询内容。如果设为非空值,你的查询内容只会与每条历史条目的开头部分进行匹配。
  例如,该变量为空时,`ls` 能匹配 `ls -l` 和 `echo ls`;非空时,`ls` 只会匹配
  `ls -l`。

* `HISTORY_SUBSTRING_SEARCH_ENSURE_UNIQUE` 是一个全局变量,定义返回的搜索结果是否
  必须_唯一_。如果设为非空值,则只会呈现唯一的搜索结果。该行为默认关闭。确保搜索
  结果唯一的另一种办法是使用 `setopt HIST_IGNORE_ALL_DUPS`。如果该配置变量关闭且
  未设置 `setopt HIST_IGNORE_ALL_DUPS`,`setopt HIST_FIND_NO_DUPS` 依然会被遵守,
  它会让本脚本在你循环切换搜索结果时跳过_相邻_的重复结果,但这并不保证搜索结果
  唯一:如果你的搜索结果是 "Dog"、"Dog"、"HotDog"、"Dog",循环切换时会得到
  "Dog"、"HotDog"、"Dog"。注意 "Dog" 这条结果在你循环切换时出现了两次。如果你希望
  只收到全局唯一、仅出现一次的搜索结果,请使用该配置变量,或使用
  `setopt HIST_IGNORE_ALL_DUPS`。

* `HISTORY_SUBSTRING_SEARCH_HIGHLIGHT_TIMEOUT` 是一个全局变量,定义清除搜索高亮的
  超时时间(单位:秒)。


历史
------------------------------------------------------------------------------

* 2009 年 9 月:[Peter Stephenson][2] 最初编写了这个脚本,并发布到 zsh-users
  邮件列表。

* 2011 年 1 月:Guido van Steen(@guidovansteen)修订了这个脚本,并以 3 条款 BSD
  许可证将其作为 [fizsh][3](the Friendly Interactive ZSHell)的一部分发布。

* 2011 年 1 月:Suraj N. Kurapati(@sunaku)把这个脚本从 [fizsh][3] 1.0.1 中抽取
  出来,进行了大量重构,最终将它重新打包为 [oh-my-zsh 插件][4]和一个可独立加载的
  [ZSH 脚本][5]。

* 2011 年 7 月:Guido van Steen、Suraj N. Kurapati 和 Sorin Ionescu
  (@sorin-ionescu)与 Vincent Guerci(@vguerci)一起[继续开发了它][4]。

* 2016 年 3 月:Geza Lore(@gezalore)在 pull request #55 中对它进行了大幅重构。

---

## Oh My Zsh 发行版说明

你现在看到的是 Oh My Zsh 发行版把 zsh-history-substring-search 重新打包为 OMZ 模块后的结果。

上游仓库 zsh-users/zsh-history-substring-search 可以在 GitHub 上找到:
https://github.com/zsh-users/zsh-history-substring-search。

本节之上的全部内容都是原上游 README 的拷贝,因此在 OMZ 内使用时,实际情况可能
略有不同。特别是:你不需要自己在 `~/.zshrc` 里为上、下方向键设置键位绑定,
OMZ 插件已经替你做好了。不过,你可能仍然想按上文所述,额外配置 emacs 或 vi
专用的键位绑定。
