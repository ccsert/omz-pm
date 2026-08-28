# emoji 插件

在 Zsh 中便捷地使用 Unicode emoji 的支持。

## 特性

本插件让你能在 `zsh` 中用人类可读的标识符处理 Unicode emoji 字符。它提供若干全局变量,把 emoji 名称映射到实际字符、把国家名映射到国旗,还提供一些命名的 emoji 分组。它也提供了用于显示这些 emoji 的相关函数。

#### 变量

变量              | 说明
----------------- | --------------------------------
  $emoji          | 把 emoji 名称映射到字符(不含国旗)
  $emoji_flags    | 把国家名映射到国旗字符(使用区域指示符)
  $emoji_groups   | emoji 的命名分组。键是组名;值是以空白分隔的字符名列表

你可以在运行时修改 `$emoji_groups` 来定义新的 emoji 分组。特殊组名 `all` 由插件保留使用。你不应修改 `$emoji` 或 `$emoji_flags`。

#### 函数

函数             | 说明
---------------- | -------------------------------
  random_emoji   | 打印一个随机的 emoji 字符
  display_emoji  | 显示 emoji 及其名称

## 用法与示例

要输出指定的 emoji,使用:
```
$> echo $emoji[<name>]
```
例如:
```
$> echo $emoji[mouse_face]
```

要输出一个随机 emoji,使用:
```
$> random_emoji
```
要输出某个特定分组中的随机 emoji,使用:
```
$> random_emoji <group>
```
例如:
```
$> random_emoji fruits
$> random_emoji animals
$> random_emoji vehicles
$> random_emoji faces
```

已定义的组名可以用 `echo ${(k)emoji_groups}` 查看。

要列出所有可用 emoji 及其名称,使用:
```
$> display_emoji
$> display_emoji faces
$> display_emoji people
```

要在提示符(prompt)中使用 emoji:
```
PROMPT="$emoji[penguin]  > ""
PROMPT='$(random_emoji fruits)  > '
surfer=$emoji[surfer]
PROMPT="$surfer  > "
```

## 技术细节

emoji 名称和码点来自 Unicode 技术报告 \#51,其中提供了 Unicode 中 emoji 支持的相关信息。可在此查阅:https://www.unicode.org/reports/tr51/index.html 。

分组定义由本 OMZ 插件添加,并非基于外部定义。

`$emoji*` 各映射中的值就是 emoji 字符本身,不是转义序列或其他需要解释的形式。它们可用于任何上下文,不要求 `echo` 或 `print` 之类的命令支持转义序列。

主 `$emoji` 映射中的 emoji 都是独立的字符序列,可以各自单独输出,不必担心组合字符的问题。这些值实际上可能是多个码点组成的序列,而非单个码点,序列中也可能会包含组合字符。但它们的排布方式保证其作用不会越出该序列。

例外是肤色/发型变体选择符。它们也被包含在主 `$emoji` 映射中,因为它们既可以单独显示,也可以作为组合字符使用。(如果它们跟在的字符不是可与之组合的 emoji 字符,就会显示为色块。)


## 实验性特性

这里定义了一些额外的变量和函数,但它们是实验性的,随时可能变化。不应依赖它们一定可用。它们主要供 emoji 插件的开发者使用,帮助决定未来的版本要纳入哪些内容。

变量:

变量              | 说明
----------------- | --------------------------------
  $emoji_skintone | 肤色修饰符(来自 Unicode 8.0)


#### 肤色变体选择

这里包含对 Unicode 8.0 引入的肤色变体选择符(Variation Selectors)的实验性支持,让你可以为涉及人物的 emoji 选择不同肤色。

注意:这确实是实验性功能。肤色选择符是相对较新的特性,并非所有系统都支持。而且本插件中的支持也仍在开发中,不一定在所有地方都能用。事实上,我还没在任何地方让它跑通过。-apjanke

「变体选择符」是改变前一个字符外观的组合字符。在人物 emoji 后面紧跟输出一个变体选择符,即可改变其肤色。也可以单独输出一个变体选择符,显示该肤色的色块。

`$emoji_skintone` 关联数组把肤色 ID 映射到变体选择符字符。使用时把它紧跟在笑脸或其他人物 emoji 后面输出即可。

```
echo $emoji[waving_hand]$emoji_skintone[5]
```

注意 `$emoji_skintone` 是关联数组,其键是「Fitzpatrick Skin Type」分组的*名称*,而不是普通数组的线性索引。名称为 `1_2`、`3`、`4`、`5` 和 `6`。(类型 1 和 2 合并为同一种颜色。)详情见 [Unicode TR 51 的 Diversity 一节](https://www.unicode.org/reports/tr51/index.html#Diversity)。

#### Gemoji 支持

[gemoji 项目](https://github.com/github/gemoji)似乎是短名称及其他官方 Unicode 报告未收录的 emoji 元数据的事实上的主要来源。因此,我们的 emoji 列表收录了它的一些别名,让你的使用更方便:

```
echo $emoji[grinning_face_with_smiling_eyes]
echo $emoji[smile]
```

这两条命令输出同一个 emoji(😄)。第一个名称是 Unicode 参考文档中的官方名称,第二个则是 Gemoji 数据库里的别名。

## TODO

以下是插件未来版本中可以增强的事项。

* 引入 CLDR 数据用于排序与分组
* 短 :bracket: 风格名称(来自 gemoji)
* ZWJ 组合函数?
