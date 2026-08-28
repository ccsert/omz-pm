# encode64

一个使用 `base64` 命令进行编码、解码的别名插件。

✅ 启用方式:把「encode64」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 函数与别名

| 函数           | 别名   | 说明                                   |
| -------------- | ------ | -------------------------------------- |
| `encode64`     | `e64`  | 将给定数据编码为 base64                |
| `encodefile64` | `ef64` | 将给定文件的内容编码为 base64          |
| `decode64`     | `d64`  | 将给定数据从 base64 解码               |

## 用法与示例

### 编码

- 从参数传入

  ```console
  $ encode64 "oh-my-zsh"
  b2gtbXktenNo
  $ e64 "oh-my-zsh"
  b2gtbXktenNo
  ```

- 通过管道传入

  ```console
  $ echo "oh-my-zsh" | encode64
  b2gtbXktenNo==
  $ echo "oh-my-zsh" | e64
  b2gtbXktenNo==
  ```

### 编码文件

把文件的内容编码为 base64,并将输出保存到文本文件。
**注意:** 会读取所提供的文件,并把编码后的内容保存为带 `.txt` 扩展名的新文件

- 从参数传入

  ```console
  $ encodefile64 ohmyzsh.icn
  ohmyzsh.icn's content encoded in base64 and saved as ohmyzsh.icn.txt
  $ ef64 "oh-my-zsh"
  ohmyzsh.icn's content encoded in base64 and saved as ohmyzsh.icn.txt
  ```

### 解码

- 从参数传入

  ```console
  $ decode64 b2gtbXktenNo
  oh-my-zsh%
  $ d64 b2gtbXktenNo
  oh-my-zsh%
  ```

- 通过管道传入

  ```console
  $ echo "b2gtbXktenNoCg==" | decode64
  oh-my-zsh
  $ echo "b2gtbXktenNoCg==" | d64
  oh-my-zsh
  ```
