# percol

提供一些实用函数,让 [percol](https://github.com/mooz/percol) 可以配合 zsh 历史记录工作,
并可选地配合 [jump 插件](https://github.com/ohmyzsh/ohmyzsh/tree/master/plugins/jump)使用。

✅ 启用方式:把「percol」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 依赖要求

- `percol`:用 `pip install percol` 安装。

- (_可选_)[`jump`](https://github.com/ohmyzsh/ohmyzsh/tree/master/plugins/jump) 插件:需要在
  `percol` 插件之前启用。

## 用法

- <kbd>CTRL-R</kbd>(绑定到 `percol_select_history`):可以用它配合 percol 检索你的历史记录。

- <kbd>CTRL-B</kbd>(绑定到 `percol_select_marks`):可以用它配合 percol 检索你的 jump 书签。
