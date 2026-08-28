#  Localstack 插件  #

用于与 LOCALSTACK 交互的 CLI 支持

##  说明  ##

✅ 启用方式:把「localstack」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

##  用法  ##

本插件提供一个命令 `lsk`,它的全部功能都通过这个命令对外提供。

## 命令

| 命令                                  | 说明                                                                  |
| :----------------------------------- | :-------------------------------------------------------------------- |
| `lsk sqs-send <queue> <message.json>` | 通过 sqs 把给定的消息发送到给定的队列                                  |

## 示例

![staging](sqs-send-result.png)
