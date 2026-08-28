# pass

本插件为 [pass](https://www.passwordstore.org/) 密码管理器提供自动补全。

✅ 启用方式:把「pass」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 配置

### 多仓库

如果你使用多个仓库,可以这样配置补全:
```zsh
compdef _pass workpass
zstyle ':completion::complete:workpass::' prefix "$HOME/work/pass"
workpass() {
  PASSWORD_STORE_DIR=$HOME/work/pass pass $@
}
```
