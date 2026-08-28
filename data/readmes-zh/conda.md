# conda 插件

conda 插件为 `conda` 提供[别名](#aliases),`conda` 通常通过 [anaconda](https://www.anaconda.com/)
或 [miniconda](https://docs.conda.io/en/latest/miniconda.html) 安装。

✅ 启用方式:把「conda」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名      | 命令                                     | 说明                                                                            |
| :------- | :-------------------------------------- | :------------------------------------------------------------------------------ |
| `cna`    | `conda activate`                        | 激活指定的 conda 环境                                                          |
| `cnab`   | `conda activate base`                   | 激活 base conda 环境                                                          |
| `cncf`   | `conda env create -f`                   | 从 YAML 文件创建新的 conda 环境                                                 |
| `cncn`   | `conda create -y -n`                    | 用给定名称创建新的 conda 环境                                                    |
| `cnconf` | `conda config`                          | 查看或修改 conda 配置                                                          |
| `cncp`   | `conda create -y -p`                    | 用给定前缀创建新的 conda 环境                                                    |
| `cncr`   | `conda create -n`                       | 用给定名称创建新的虚拟环境                                                       |
| `cncss`  | `conda config --show-source`            | 显示 conda 配置来源的位置                                                        |
| `cnde`   | `conda deactivate`                      | 停用当前的 conda 环境                                                          |
| `cnel`   | `conda env list`                        | 列出所有可用的 conda 环境                                                        |
| `cni`    | `conda install`                         | 安装给定的软件包                                                                |
| `cniy`   | `conda install -y`                      | 无需确认安装给定的软件包                                                            |
| `cnl`    | `conda list`                            | 列出当前环境中已安装的软件包                                                        |
| `cnle`   | `conda list --export`                   | 导出当前环境中已安装软件包的列表                                                      |
| `cnles`  | `conda list --explicit > spec-file.txt` | 把当前环境中已安装软件包的列表导出到 spec 文件                                              |
| `cnr`    | `conda remove`                          | 移除给定的软件包                                                                |
| `cnrn`   | `conda remove -y -all -n`               | 移除指定环境中的所有软件包                                                          |
| `cnrp`   | `conda remove -y -all -p`               | 移除指定前缀中的所有软件包                                                          |
| `cnry`   | `conda remove -y`                       | 无需确认移除给定的软件包                                                            |
| `cnsr`   | `conda search`                          | 在 conda 仓库中搜索软件包                                                         |
| `cnu`    | `conda update`                          | 更新 conda 包管理器                                                            |
| `cnua`   | `conda update --all`                    | 更新所有已安装的软件包                                                             |
| `cnuc`   | `conda update conda`                    | 更新 conda 包管理器                                                            |
