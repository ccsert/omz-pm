# Cake

本插件为 [CakePHP](https://cakephp.org/) 提供自动补全。

✅ 启用方式:把「cake」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 注意

本插件会在当前工作目录下生成一个缓存文件,记录找到的所有 cake 任务,名为 `.cake_task_cache`。
当 Cakefile 比缓存文件新时,缓存会重新生成。建议把这个 cake 缓存文件加入你的
`.gitignore`。
