# uv 插件

本插件会自动为你安装 [uv](https://github.com/astral-sh/uv) 的补全,并保持它们最新。
它还为常见用法添加了便捷的别名。

✅ 启用方式:把「uv」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名  | 命令                                                                                     | 说明                                                                  |
| :---- | ---------------------------------------------------------------------------------------- | :-------------------------------------------------------------------- |
| uva   | `uv add`                                                                                 | 向项目添加软件包                                                       |
| uvexp | `uv export --format requirements-txt --no-hashes --output-file requirements.txt --quiet` | 把锁文件导出为 `requirements.txt`                                      |
| uvi   | `uv init`                                                                                | 在当前工作区和环境中初始化一个新项目。                                   |
| uvinw | `uv init --no-workspace`                                                                 | 在新的工作区和环境中初始化一个新项目                                     |
| uvl   | `uv lock`                                                                                | 锁定依赖                                                               |
| uvlr  | `uv lock --refresh`                                                                      | 重新构建锁文件,但不升级依赖                                             |
| uvlu  | `uv lock --upgrade`                                                                      | 把依赖锁定到最新的兼容版本                                               |
| uvp   | `uv pip`                                                                                 | 管理 pip 软件包                                                         |
| uvpi  | `uv python install`                                                                      | 安装特定版本的 python                                                   |
| uvpl  | `uv python list`                                                                         | 列出已安装的所有 python 版本                                             |
| uvpp  | `uv python pin`                                                                          | 把当前项目固定为使用特定 Python 版本                                     |
| uvpu  | `uv python uninstall`                                                                    | 移除特定版本的 python                                                   |
| uvpy  | `uv python`                                                                              | 管理 Python 安装                                                        |
| uvr   | `uv run`                                                                                 | 在项目环境中运行命令                                                     |
| uvrm  | `uv remove`                                                                              | 从项目中移除软件包                                                       |
| uvs   | `uv sync`                                                                                | 把环境与锁文件同步                                                       |
| uvsr  | `uv sync --refresh`                                                                      | 「强制」把环境与锁文件同步(忽略缓存)                                    |
| uvsu  | `uv sync --upgrade`                                                                      | 同步环境,允许升级并忽略锁文件                                            |
| uvtr  | `uv tree`                                                                                | 显示当前项目环境的完整依赖树                                              |
| uvup  | `uv self update`                                                                         | 把 UV 工具更新到最新版本                                                 |
| uvv   | `uv venv`                                                                                | 管理虚拟环境                                                             |
