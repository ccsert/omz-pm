# Molecule 插件

本插件为 [Molecule](https://ansible.readthedocs.io/projects/molecule/) 添加别名和补全。
Molecule 是一个旨在帮助开发和测试 Ansible 角色的项目。

✅ 启用方式:把「molecule」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名 | 命令              | 说明                                                                               |
| :--- | :---------------- | ---------------------------------------------------------------------------------- |
| mol  | molecule          | Molecule 用于帮助开发和测试 Ansible 角色。                                         |
| mcr  | molecule create   | 使用 provisioner 启动实例。                                                        |
| mcon | molecule converge | 使用 provisioner 配置实例(依赖、创建、准备、收敛)。                               |
| mls  | molecule list     | 列出实例的状态。                                                                   |
| mvf  | molecule verify   | 对实例运行自动化测试。                                                             |
