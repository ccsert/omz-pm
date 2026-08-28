# `transfer` 插件

[`transfer.sh`](https://transfer.sh) 是一个在命令行上即可轻松使用的文件分享服务

✅ 启用方式:把「transfer」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

- 传输一个文件:`transfer file.txt`。

- 传输整个目录(会自动压缩):`transfer dir`。

### 加密 / 解密

- 用对称加密算法加密并上传文件,输出 ASCII 封装(ASCII armored)格式:

  ```zsh
  transfer file -ca
  ```

- 用对称加密和 gpg 输出加密并上传目录:

  ```zsh
  transfer directory -ca
  ```

- 解密文件:

  ```zsh
  gpg -d file -ca
  ```

- 解密目录:

  ```zsh
  gpg -d your_archive.tgz.gpg | tar xz
  ```
