## arcanist

本插件为 [arcanist](https://github.com/phacility/arcanist) 提供了许多实用的别名。

✅ 启用方式:把「arcanist」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名    | 命令                               |
| ------- | ---------------------------------- |
| ara     | `arc amend`                        |
| arb     | `arc branch`                       |
| arbl    | `arc bland`                        |
| arco    | `arc cover`                        |
| arci    | `arc commit`                       |
| ard     | `arc diff`                         |
| ardc    | `arc diff --create`                |
| ardp    | `arc diff --preview`               |
| ardnu   | `arc diff --nounit`                |
| ardnupc | `arc diff --nounit --plan-changes` |
| ardpc   | `arc diff --plan-changes`          |
| are     | `arc export`                       |
| arh     | `arc help`                         |
| arho    | `arc hotfix`                       |
| arl     | `arc land`                         |
| arli    | `arc lint`                         |
| arls    | `arc list`                         |
| arpa    | `arc patch`                        |

## 函数

下列函数让你可以直接复制浏览器地址栏里的整个 URL 来使用,而不必只截取其中的 revision id,粘贴起来更方便。
例如:`ardu` 既接受 `https://arcanist-url.com/<REVISION>`,也接受 `<REVISION>`。

| 函数                      | 命令                              |
| ------------------------- | --------------------------------- |
| ardu [URL or revision_id] | `arc diff --update` [revision_id] |
| arpa [URL or revision_id] | `arc patch` [revision_id]         |
