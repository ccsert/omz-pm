# grunt 插件

本插件为 [grunt](https://github.com/gruntjs/grunt) 提供补全。

✅ 启用方式:把「grunt」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 启用缓存

如果想使用缓存,请在 `.zshrc` 中设置如下内容:
```zsh
zstyle ':completion:*' use-cache yes
```

## 设置

* 显示 grunt 文件路径:
  ```zsh
  zstyle ':completion::complete:grunt::options:' show_grunt_path yes
  ```
* 缓存过期天数(默认:7):
  ```zsh
  zstyle ':completion::complete:grunt::options:' expire 1
  ```
* 目标 gruntfile 发生变化时不更新选项缓存。
  ```zsh
  zstyle ':completion::complete:grunt::options:' no_update_options yes
  ```

注意:如果修改了 zstyle 设置,应删除缓存文件并重启 zsh。

```zsh
$ rm ~/.zcompcache/grunt
$ exec zsh
```
