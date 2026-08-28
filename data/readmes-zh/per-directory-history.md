# per-directory-history 插件

本插件为 zsh 提供按目录区分的历史记录,同时保留全局历史,并支持用快捷键在两者之间切换。
它是对 [@jimhester 的官方插件][5] 的打包集成。

✅ 启用方式:把「per-directory-history」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

这是 zsh 按目录历史的一种实现,bash 中也存在一些类似的实现[1][],[2][]。它还实现了一个
toggle-history 函数,用于从目录历史切换到全局历史。无论处于哪种模式,历史始终会同时
保存到全局历史和目录历史中,因此切换状态不会影响已保存的历史。能够在全局历史和
目录历史之间即时切换是一项新颖的特性。

## 用法

默认模式是按目录历史,像平常一样使用你的历史记录即可。

按 ^G(同时按下 <kbd>Control</kbd> 和 <kbd>G</kbd> 键)可在本地历史和全局历史之间切换。
如果你更喜欢别的切换快捷键,可以设置 `PER_DIRECTORY_HISTORY_TOGGLE` 环境变量。

## 配置

* `HISTORY_BASE` 是一个全局变量,定义存储各目录历史的基准目录(默认 `$HOME/.directory_history`)。
* `per-directory-history-toggle-history` 是在本地历史和全局历史之间切换的函数。
* `PER_DIRECTORY_HISTORY_TOGGLE` 是用于运行上述 toggle-history 函数的按键绑定(默认 `^G`)
* `PER_DIRECTORY_HISTORY_PRINT_MODE_CHANGE` 是一个变量,控制模式切换后是否把当前模式打印到屏幕上(默认 `true`)
* `HISTORY_START_WITH_GLOBAL` 是一个全局变量,定义插件的启动方式:全局还是本地(默认 `false`)

## 历史

按目录区分历史这一想法/灵感来自 [Stewart MacArthur][1] 和 [Dieter][2],
实现思路来自 [Bart Schaefer][3]。实现者是 [Jim Hester][4],完成于 2012 年 9 月。

[1]: http://www.compbiome.com/2010/07/bash-per-directory-bash-history.html
[2]: http://dieter.plaetinck.be/per_directory_bash
[3]: https://www.zsh.org/mla/users/1997/msg00226.html
[4]: https://jimhester.com
[5]: https://github.com/jimhester/per-directory-history
