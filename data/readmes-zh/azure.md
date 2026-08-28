# azure

本插件为 [azure cli](https://docs.microsoft.com/en-us/cli/azure/) 提供补全支持,
并附带几个用于管理 azure 订阅、在提示符中显示它们的小工具。

✅ 启用方式:把「azure」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 插件命令


* `az_subscriptions`:列出 `AZURE_CONFIG_DIR`(默认:`~/.azure/`)中可用的订阅。
  用于为 `azss` 函数提供补全。

* `azgs`:获取 `$azure_subscription` 的当前值。

* `azss [<subscription>]`:设置 `$azure_subscription`。


注意:azure 把活动订阅的状态保存在 ${AZURE_CONFIG_DIR:-$HOME/.azure/azureProfile.json} 中,因此提示符命令需要 `jq` 已启用来解析该文件。如果 jq 不在 path 中,提示符将不显示任何内容

## 主题

插件会创建一个 `azure_prompt_info` 函数,你可以在主题里使用它,它会显示
当前的 `$azure_subscription`。它用两个变量来控制显示方式:

- ZSH_THEME_AZURE_PREFIX:设置 azure_subscription 的前缀。默认为 `<az:`。

- ZSH_THEME_azure_SUFFIX:设置 azure_subscription 的后缀。默认为 `>`。


```
RPROMPT='$(azure_prompt_info)'
```

## 开发

在 ubuntu 上可以用下面的命令获得一个可用的环境:

` docker run -it -v $(pwd):/mnt -w /mnt ubuntu bash`

```
apt install -y curl jq zsh git vim
sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
curl -sL https://aka.ms/InstallAzureCLIDeb | bash
```
