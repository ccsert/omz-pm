# scd - 智能切换目录

定义 `scd` shell 函数,只需敲几个键就能切换到任意目录。

`scd` 会保存访问过的目录的历史,作为已知路径的索引。shell 中每执行一次 `cd` 命令,
目录索引都会更新,也可以通过运行 `scd -a` 手动填充。要切换到某个目录,
`scd` 只需要目标路径的少量片段来与索引匹配。如果匹配到多个结果,会显示一个选择菜单,
并优先考虑最近访问过的路径。`scd` 还能创建永久的目录别名,
它们会作为命名目录出现在 zsh 会话中。

## 安装说明

除了 oh-my-zsh,`scd` 还可以在 *bash*、*dash* 或 *tcsh* shell 中使用,
也有 Vim 插件 [scd.vim](https://github.com/pavoljuhas/scd.vim) 和
[IPython](https://ipython.org) 扩展版本。安装详情见
https://github.com/pavoljuhas/smart-change-directory 。

## 概要

```sh
scd [options] [pattern1 pattern2 ...]
```

## 模式

模式可以使用 *extendedglob* 选项下所有可用的 zsh [glob 操作符](
http://zsh.sourceforge.net/Doc/Release/Expansion.html#Glob-Operators)。
指定的模式必须匹配绝对路径,且其中至少一个要匹配路径的尾部。
另外还识别如下几种特殊模式:

<dl><dt>
^PAT</dt><dd>
  PAT 必须匹配路径的开头,例如「^/home」</dd><dt>
PAT$</dt><dd>
  要求 PAT 匹配路径的结尾,例如「man$」</dd><dt>
./</dt><dd>
  只匹配当前目录的子目录</dd><dt>
:PAT</dt><dd>
  要求 PAT 匹配尾部的路径组件,例如「:doc」、「:re/doc」</dd>
</dl>


## 选项

<dl><dt>
-a, --add</dt><dd>
  把当前目录或指定目录加入目录索引。</dd><dt>

--unindex</dt><dd>
  把当前目录或指定目录从索引中移除。</dd><dt>

-r, --recursive</dt><dd>
  递归地应用 <em>--add</em> 或 <em>--unindex</em> 选项。</dd><dt>

--alias=ALIAS</dt><dd>
  为当前目录或指定目录创建别名,并保存到
  <em>~/.scdalias.zsh</em>。</dd><dt>

--unalias</dt><dd>
  从 <em>~/.scdalias.zsh</em> 中删除当前目录或指定目录的 ALIAS 定义。
  使用「OLD」可清除指向不存在目录的别名。</dd><dt>

-A, --all</dt><dd>
  显示所有目录,包括被 <em>~/.scdignore</em> 中的模式排除的目录。
  同时忽略目录别名的唯一匹配,以及针对较不可能路径的过滤。</dd><dt>

-p, --push</dt><dd>
  使用「pushd」切换到目标目录。</dd><dt>

--list</dt><dd>
  显示匹配的目录后退出。</dd><dt>

-v, --verbose</dt><dd>
  在选择菜单中显示目录的排名。</dd><dt>

-h, --help</dt><dd>
  显示这份选项摘要后退出。</dd>
</dl>


## 示例

```sh
# Index recursively some paths for the very first run
scd -ar ~/Documents/

# Change to a directory path matching "doc"
scd doc

# Change to a path matching all of "a", "b" and "c"
scd a b c

# Change to a directory path that ends with "ts"
scd "ts$"

# Show selection menu and ranking of 20 most likely directories
scd -v

# Alias current directory as "xray"
scd --alias=xray

# Jump to a previously defined aliased directory
scd xray
```

## 文件

<dl><dt>
~/.scdhistory</dt><dd>
    带时间戳的已访问目录索引。</dd><dt>

~/.scdalias.zsh</dt><dd>
    scd 生成的目录别名定义。</dd><dt>

~/.scdignore</dt><dd>
    <a href="http://zsh.sourceforge.net/Doc/Release/Expansion.html#Glob-Operators">
    glob 模式</a>,用于指定 scd 搜索中要忽略的路径,例如,
    <code>/mnt/backup/*</code>。模式每行指定一个,匹配时假定开启了
    <em>extendedglob</em> zsh 选项。以「#」开头的行会作为注释跳过。
    在 <em>--all</em> 模式下不应用 .scdignore 中的模式。</dd>
</dl>


## 环境变量

<dl><dt>
SCD_HISTFILE</dt><dd>
    scd 索引文件的路径(默认为 ~/.scdhistory)。</dd><dt>

SCD_HISTSIZE</dt><dd>
    索引中的最大条目数(5000)。当索引超出 <em>SCD_HISTSIZE</em>
    达 20% 以上时会被裁剪。</dd><dt>

SCD_MENUSIZE</dt><dd>
    目录选择菜单的最大条目数(20)。</dd><dt>

SCD_MEANLIFE</dt><dd>
    目录可能性按指数衰减的平均寿命,单位为秒(86400)。</dd><dt>

SCD_THRESHOLD</dt><dd>
    目录累积可能性的阈值。与最佳匹配相比可能性更低的目录会被排除(0.005)。
    </dd><dt>

SCD_SCRIPT</dt><dd>
    scd 把最终 <code>cd</code> 命令写入的命令脚本文件。当 scd 在自己的
    进程中运行而不是作为 shell 函数运行时,必须定义该变量。如何使用
    <em>SCD_SCRIPT</em> 中的输出由 scd 的调用方决定。</dd>
</dl>
