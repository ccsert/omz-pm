# Poetry 插件

本插件会自动为你安装 [Poetry](https://python-poetry.org/) 的补全,并在你的 Poetry 版本变化时让补全保持最新。

✅ 启用方式:把「poetry」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名  | 命令                                                | 说明                                                                                    |
|:----- |--------------------------------------------------- |:--------------------------------------------------------------------------------------- |
| pad   | `poetry add`                                        | 把依赖包添加到 `pyproject.toml` 并安装                                                  |
| pbld  | `poetry build`                                      | 构建 source 和 wheel 归档                                                               |
| pch   | `poetry check`                                      | 校验 `pyproject.toml` 的内容及其与 `poetry.lock` 的一致性                               |
| pcmd  | `poetry list`                                       | 显示所有可用的 Poetry 命令                                                              |
| pconf | `poetry config --list`                              | 让你能够编辑 poetry 的配置项与仓库设置                                                  |
| pexp  | `poetry export --without-hashes > requirements.txt` | 把锁文件导出为 `requirements.txt`                                                       |
| pin   | `poetry init`                                       | 以交互方式创建 `pyproject.toml`                                                         |
| pinst | `poetry install`                                    | 读取 `pyproject.toml`,解析依赖并安装                                                    |
| plck  | `poetry lock`                                       | 锁定 `pyproject.toml` 中的依赖,但不安装                                                |
| pnew  | `poetry new`                                        | 创建适用于大多数 Python 项目的目录结构                                                  |
| ppath | `poetry env info --path`                            | 获取当前激活的 virtualenv 的路径                                                        |
| pplug | `poetry self show plugins`                          | 列出所有已安装的 Poetry 插件                                                            |
| ppub  | `poetry publish`                                    | 把构建好的(由 `poetry build` 命令生成)包发布到远程仓库                                 |
| prm   | `poetry remove`                                     | 从 `pyproject.toml` 移除依赖包并卸载                                                    |
| prun  | `poetry run`                                        | 在项目的 virtualenv 中执行给定命令                                                      |
| psad  | `poetry self add`                                   | 添加 Poetry 插件并安装其依赖使之可用                                                    |
| psh   | `poetry shell`                                      | 在虚拟环境中启动一个 shell。如果尚不存在,则会先创建                                    |
| pshw  | `poetry show`                                       | 列出所有可用的依赖                                                                      |
| pslt  | `poetry show --latest`                              | 列出依赖的最新版本                                                                      |
| psup  | `poetry self update`                                | 把 Poetry 更新到最新版本(默认)或指定版本                                              |
| psync | `poetry install --sync`                             | 让你的环境与 `poetry.lock` 保持同步                                                     |
| ptree | `poetry show --tree`                                | 以树状结构列出依赖                                                                      |
| pup   | `poetry update`                                     | 获取依赖的最新版本并更新 `poetry.lock`                                                  |
| pvinf | `poetry env info`                                   | 获取当前激活的 virtualenv 的基本信息                                                    |
| pvoff | `poetry config virtualenvs.create false`            | 禁用自动创建 virtualenv                                                                 |
| pvrm  | `poetry env remove`                                 | 删除已有的 virtualenv                                                                   |
| pvrma | `poetry env remove --all`                           | 删除所有已存在的 virtualenv                                                             |
| pvu   | `poetry env use`                                    | 在已有的 virtualenv 之间切换                                                            |
