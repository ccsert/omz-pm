# Pulumi

这是一个面向 [**Pulumi CLI**](https://www.pulumi.com/docs/iac/cli/) 的 **Oh My Zsh 插件**,
Pulumi CLI 是一个用于构建、部署和管理云基础设施的 Infrastructure as Code(IaC)工具。

本插件提供:

- 🚀 面向常用 Pulumi 命令的简短、直观的别名
- 🎯 对 Pulumi 的自动补全支持

✅ 启用方式:把「pulumi」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## ⚡ 别名

| 别名     | 命令                   | 说明                          |
| -------- | ---------------------- | ----------------------------- |
| `pul`    | `pulumi`               | Pulumi CLI 的快捷方式         |
| `pulcs`  | `pulumi config set`    | 设置 Pulumi 配置              |
| `puld`   | `pulumi destroy`       | 销毁所有资源                  |
| `pullog` | `pulumi logs -f`       | 实时跟踪 Pulumi 日志          |
| `pulp`   | `pulumi preview`       | 显示计划中的变更              |
| `pulr`   | `pulumi refresh`       | 从云端刷新状态                |
| `puls`   | `pulumi stack`         | 显示 stack 详情               |
| `pulsh`  | `pulumi stack history` | 显示 stack 历史               |
| `pulsi`  | `pulumi stack init`    | 初始化一个新 stack            |
| `pulsl`  | `pulumi stack ls`      | 列出可用的 stack              |
| `pulso`  | `pulumi stack output`  | 显示 stack 的输出             |
| `pulss`  | `pulumi stack select`  | 切换 stack                    |
| `pulu`   | `pulumi up`            | 部署基础设施                  |

## 🎯 自动补全

如果 `pulumi gen-completion zsh` 可用,本插件会**自动加载 Pulumi 的自动补全**。

## 🛠️ 参与贡献

欢迎提交 issue 或 PR 来改进! 🚀
