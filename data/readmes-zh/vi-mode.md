# vi-mode 插件

本插件为 zsh 增加 `vi-like` 的功能。

✅ 启用方式:把「vi-mode」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 设置

- `VI_MODE_RESET_PROMPT_ON_MODE_CHANGE`:控制在切换到不同输入模式时是否重绘提示符。
  如果未设置,则切换到不同模式时模式指示器不会更新。
  将其设为 `true` 即可启用。例如:

  ```zsh
  VI_MODE_RESET_PROMPT_ON_MODE_CHANGE=true
  ```

  默认未设置;但如果使用了 `vi_mode_prompt_info`,则会自动将其设为 `true`。

- `VI_MODE_SET_CURSOR`:控制在切换到不同输入模式时是否改变光标样式。将其设为 `true` 即可启用(默认:未设置):

  ```zsh
  VI_MODE_SET_CURSOR=true
  ```

  参见 [光标样式](#cursor-styles) 了解如何控制不同模式下光标的外观。

- `MODE_INDICATOR`:控制 shell 处于普通模式时显示的字符串。
  详见 [模式指示器](#mode-indicators)。

- `INSERT_MODE_INDICATOR`:控制 shell 处于插入模式时显示的字符串。
  详见 [模式指示器](#mode-indicators)。

- `VI_MODE_DISABLE_CLIPBOARD`:如果设置,则禁用 yank/paste 时的剪贴板集成

## 模式指示器

若主题未定义,*普通模式*会在右侧提示符处以红色的 `<<<` 标记指示;*插入模式*默认不显示。

你可以通过设置 `MODE_INDICATOR`(*普通模式*)和 `INSERT_MODE_INDICATORS`(*插入模式*)变量来修改这些指示器。
这些设置支持 Prompt Expansion(提示符展开)序列。例如:

```zsh
MODE_INDICATOR="%F{white}+%f"
INSERT_MODE_INDICATOR="%F{yellow}+%f"
```

### 把模式指示器加入你的提示符

默认情况下,`Vi-mode` 会把模式指示器加到 `RPROMPT` 中,**除非**该变量已被之前的插件定义。

如果 `PROMPT` 或 `RPROMPT` 的定义不合你的意,你可以手动添加模式信息。可以使用 `vi_mode_prompt_info` 函数来插入模式指示器信息。

下面是一些示例:

```bash
source $ZSH/oh-my-zsh.sh

PROMPT="$PROMPT\$(vi_mode_prompt_info)"
RPROMPT="\$(vi_mode_prompt_info)$RPROMPT"
```

注意这里的 `\$`,它很关键:既能在定义时阻止插值,又允许它在每次提示符更新事件时执行。

## 光标样式

你可以通过修改以下变量的值,控制各个生效的 vim 模式所使用的光标样式。

```zsh
# defaults
VI_MODE_CURSOR_NORMAL=2
VI_MODE_CURSOR_VISUAL=6
VI_MODE_CURSOR_INSERT=6
VI_MODE_CURSOR_OPPEND=0
```

- 0, 1 - 闪烁的方块
- 2 - 实心方块
- 3 - 闪烁的下划线
- 4 - 实心的下划线
- 5 - 闪烁的竖线
- 6 - 实心的竖线

## 键位绑定

使用 `ESC` 或 `CTRL-[` 进入 `Normal mode`。

注意:使用 vi-mode 键位映射时,其中一些键位绑定是 zsh 默认就设置的。

### 历史

- `ctrl-p` : 历史中的上一条命令
- `ctrl-n` : 历史中的下一条命令
- `/`      : 向后搜索历史
- `n`      : 重复上一次 `/`

### Vim 编辑

- `vv`     : 在 Vim 中编辑当前命令行

注意:该功能过去绑定在 `v` 上。`v` 现在是默认的(`visual-mode`)。

### 移动

- `$`   : 到行尾
- `^`   : 到本行第一个非空白字符
- `0`   : 到本行第一个字符
- `w`   : 向前 [count] 个单词
- `W`   : 向前 [count] 个 WORDS
- `e`   : 向前移至第 [count] 个单词的结尾(含)
- `E`   : 向前移至第 [count] 个 WORDS 的结尾(含)
- `b`   : 向后 [count] 个单词
- `B`   : 向后 [count] 个 WORDS
- `t{char}`   : 向右移到第 [count] 个 {char} 之前
- `T{char}`   : 向左移到第 [count] 个 {char} 之前
- `f{char}`   : 移到右侧第 [count] 个 {char} 上
- `F{char}`   : 移到左侧第 [count] 个 {char} 上
- `;`   : 按最新方向重复 f、t、F 或 T [count] 次
- `,`   : 反方向重复最近一次 f、t、F 或 T

### 插入

- `i`   : 在光标前插入文本
- `I`   : 在本行第一个字符前插入文本
- `a`   : 在光标后追加文本
- `A`   : 在行尾追加文本
- `o`   : 在当前行下方插入新命令行
- `O`   : 在当前行上方插入新命令行

### 删除并插入

- `ctrl-h`      : *插入模式*下:删除光标前的字符
- `ctrl-w`      : *插入模式*下:删除光标前的单词
- `d{motion}`   : 删除 {motion} 移动经过的文本
- `dd`          : 删除整行
- `D`           : 从光标处删除直到行尾
- `c{motion}`   : 删除 {motion} 移动经过的文本并开始插入
- `cc`          : 删除整行并开始插入
- `C`           : 删除到行尾并开始插入
- `P`           : 在光标前插入剪贴板内容
- `p`           : 在光标后插入剪贴板内容
- `r{char}`     : 用 {char} 替换光标处的字符
- `R`           : 进入替换模式:每个输入的字符都会替换现有字符
- `x`           : 删除光标处及其后的 `count` 个字符
- `X`           : 删除光标前的 `count` 个字符

注意:删除/kill 命令(`dd`、`D`、`c{motion}`、`C`、`x`、`X`)和 yank 命令
(`y`、`Y`)会复制到剪贴板。之后可以用粘贴命令(`P`、`p`)把内容放回。

## 文本对象

支持标准文本对象,即 `i`("inside",内部)和 `a`("around",周围),例如对单词;因此,你可以用 `viw` 选中光标所在的单词,或用 `daw` 删除当前单词(包括周围的空格)。

对于其他文本对象,你可以依赖 Zsh 的内建功能并相应地启用它。
例如,对于引号字符串,你可以复制 <https://sourceforge.net/p/zsh/code/ci/master/tree/Functions/Zle/select-quoted> 中带注释的代码片段:把它放进你的 `.zsrhc` 文件,例如放在 source oh-my-zsh 之后:

```sh
autoload -U select-quoted
zle -N select-quoted
for m in visual viopp; do
    for c in {a,i}{\',\",\`}; do
        bindkey -M $m $c select-quoted
    done
done
```

现在,在普通模式下,你可以用 `vi"` 选中双引号字符串内的所有内容。
注意,即使你并不在引号字符串之内,这一操作也能生效。
例如,无论光标在哪里,你都可以用 `ci'` 替换当前行内单引号字符串中的所有内容。

## 已知问题

### `$KEYTIMEOUT` 过低

`$KEYTIMEOUT` 值过低(< 15)意味着需要多个字符的键位绑定(比如 `vv`)会很难触发。`$KEYTIMEOUT` 控制的是读取按键并触发相应键位绑定之前必须经过的毫秒数。对于多字符键位绑定,按键必须在超时之前完成,因此在低超时值下按键会显得太慢,结果触发了另一个键位绑定。

我们建议要么把 `$KEYTIMEOUT` 调高,要么把你想触发的键位绑定重新映射到某个按键序列。例如:

```zsh
bindkey -M vicmd 'V' edit-command-line # this remaps `vv` to `V` (but overrides `visual-mode`)
```
