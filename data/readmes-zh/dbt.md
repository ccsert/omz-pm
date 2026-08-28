# dbt 插件

`dbt plugin` 为实用的 [dbt](https://docs.getdbt.com/) 命令及[别名](#aliases)添加了若干别名。

✅ 启用方式:把「dbt」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名   | 命令                                             | 说明                                                 |
| ------ | ------------------------------------------------ | ---------------------------------------------------- |
| dbtlm  | `dbt ls -s state:modified`                       | 仅列出已修改的模型                                   |
| dbtrm  | `dbt run -s state:modified`                      | 仅运行已修改的模型                                   |
| dbttm  | `dbt test -m state:modified`                     | 仅测试已修改的模型                                   |
| dbtrtm | `dbtrm && dbttm`                                 | 仅运行并测试已修改的模型                             |
| dbtrs  | `dbt clean; dbt deps; dbt seed`                  | 重新播种(seed)数据                                  |
| dbtfrt | `dbtrs; dbt run --full-refresh; dbt test`        | 执行一次带测试的全新完整运行                         |
| dbtcds | `dbt docs generate; dbt docs serve`              | 生成文档但不编译                                     |
| dbtds  | `dbt docs generate --no-compile; dbt docs serve` | 生成并提供文档服务,跳过文档的重新编译               |

## 维护者

- [msempere](https://github.com/msempere)
