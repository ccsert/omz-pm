# pip 插件

本插件为 Python 包管理器 [pip](https://pip.pypa.io/en/latest/) 提供补全。

✅ 启用方式:把「pip」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## pip 缓存

pip 插件会缓存 PyPI 索引中可用的 pip 包名称。
要触发缓存过程,可以尝试补全 `pip install`,也可以直接运行 `zsh-pip-cache-packages`。

要重置缓存,运行 `zsh-pip-clear-cache`;下次自动补全 `pip install` 时会重新构建缓存。

## 别名

| 别名     | 命令                                                                              | 说明                                          |
| :--------|:----------------------------------------------------------------------------------|:--------------------------------------------- |
| pipi     | `pip install`                                                                     | 安装包                                        |
| pipig    | `pip install "git+https://github.com/user/repo.git"`                              | 从 GitHub 仓库安装包                          |
| pipigb   | `pip install "git+https://github.com/user/repo.git@branch"`                       | 从 GitHub 分支安装包                          |
| pipigp   | `pip install "git+https://github.com/user/repo.git@refs/pull/PR_NUMBER/head"`     | 从 GitHub pull request 安装包                 |
| pipu     | `pip install --upgrade`                                                           | 升级包                                        |
| pipun    | `pip uninstall`                                                                   | 卸载包                                        |
| pipgi    | `pip freeze \| grep`                                                              | 在已安装的包中做 grep 过滤                    |
| piplo    | `pip list --outdated`                                                             | 列出过时的包                                  |
| pipreq   | `pip freeze > requirements.txt`                                                   | 生成 requirements 文件                        |
| pipir    | `pip install -r requirements.txt`                                                 | 从 `requirements.txt` 文件安装包              |
| pipupall | `pip list --outdated \| awk 'NR > 2 { print $1 }' \| xargs pip install --upgrade` | 更新所有已安装的包                            |
| pipunall | `pip list --format freeze \| cut -d= -f1 \| xargs pip uninstall`                  | 卸载所有已安装的包                            |
