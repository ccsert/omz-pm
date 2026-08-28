# Bazel 插件

本插件为 [bazel](https://bazel.build) 提供自动补全和一组别名。bazel 是一款开源构建与测试工具,
可扩展地支持多语言、多平台项目。

✅ 启用方式:把「bazel」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

本插件内含一份 [git 仓库中补全脚本][1] 的副本。

[1]: https://github.com/bazelbuild/bazel/blob/master/scripts/zsh_completion/_bazel

## 别名

| 别名  | 命令          | 说明                      |
| ----- | ------------- | ------------------------- |
| bzb   | `bazel build` | `bazel build` 命令        |
| bzt   | `bazel test`  | `bazel test` 命令         |
| bzr   | `bazel run`   | `bazel run` 命令          |
| bzq   | `bazel query` | `bazel query` 命令        |

## 函数

| 函数     | 说明                             |
| -------- | -------------------------------- |
| sri-hash | 生成 bzlmod 所需的 SRI 哈希      |
