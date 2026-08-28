# z - 快速跳转

本插件定义了 [z 命令](https://github.com/agkozak/zsh-z),它会记录你最常访问的目录,
让你只需极少的按键就能访问它们。

### 示例

假设你之前访问过 `~/.oh-my-zsh/plugins` 目录。在命令行的任意文件夹下,你都可以通过
对该文件夹的 regex 匹配快速访问它:

```bash
/usr/bin$ z plug  # Even 'z p' might suffice
~/.oh-my-zsh/plugins$
```

### 设置

✅ 启用方式:把「z」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

### 延伸阅读

关于 z 的高级用法和细节,请参阅 [MANUAL](./MANUAL.md)(复制自 [agkozak/zsh-z](https://github.com/agkozak/zsh-z))。
