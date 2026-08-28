# otp 插件

本插件让你可以用 [`oathtool`](https://www.nongnu.org/oath-toolkit/man-oathtool.html) 创建一次性密码,
能够替代 MFA 设备。oathtool 的密钥保存在一个 GPG 加密文件中,因此只有能解密该文件的用户
才能生成验证码。

✅ 启用方式:把「otp」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

提供的别名:

- `otp_add_device`:为一个 oathtool 密钥创建新的加密存储并保存到磁盘上。加密密钥时,
  它会要求输入一个 GPG 用户 ID(即你 GPG 密钥的邮箱地址)。随后需要粘贴 OTP 密钥,
  并在最后一个空行上按下 CTRL+D 结束输入。

- `ot`:基于给定密钥生成 MFA 验证码并复制到剪贴板
  (在 Linux 上依赖 xsel,在 MacOS X 上则使用 pbcopy)。

本插件使用 `$HOME/.otp` 存放其内部文件。
