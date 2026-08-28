# gcloud

本插件为 [Google Cloud SDK CLI](https://cloud.google.com/sdk/gcloud/) 提供补全支持。

✅ 启用方式:把「gcloud」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

它依赖于你已使用[这里](https://cloud.google.com/sdk/install)列出的受支持安装方式之一安装了 SDK。

## 插件选项

* 如果你的 GCloud SDK 安装在非标准位置,请在加载 oh-my-zsh 之前,在你的 `zshrc` 文件里设置 `CLOUDSDK_HOME`。
如果插件发现该变量已设置,就会把它作为你的 SDK 的基础路径。

* 如果你的 `PATH` 中没有 `python2`,你还需要在 `.zshrc` 的末尾设置 `CLOUDSDK_PYTHON` 环境变量。
SDK 在运行命令时会用它来调用兼容的解释器。
