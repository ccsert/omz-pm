# Paver

本插件为 [Paver](https://pythonhosted.org/Paver/) 的 `paver` 命令行工具提供补全。

✅ 启用方式:把「paver」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

补全函数会在当前工作目录下创建一个名为 `.paver_tasks` 的 paver 任务缓存文件。当 `pavement.py` 发生变化时,它会重新生成该缓存。
