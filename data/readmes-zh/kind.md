# Kind 插件

本插件为 [Kind](https://kind.sigs.k8s.io/) 工具添加自动补全,以及一些方便使用的别名。

✅ 启用方式:把「kind」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名    | 命令                         |
| ------- | ---------------------------- |
| `kicc`  | `kind create cluster`        |
| `kiccn` | `kind create cluster --name` |
| `kigc`  | `kind get clusters`          |
| `kidc`  | `kind delete cluster`        |
| `kidcn` | `kind delete cluster --name` |
| `kidca` | `kind delete clusters -A`    |
| `kigk`  | `kind get kubeconfig`        |
