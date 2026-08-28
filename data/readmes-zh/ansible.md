# ansible 插件

`ansible plugin` 为实用的 [ansible](https://docs.ansible.com/ansible/latest/index.html) 命令和[别名](#aliases)添加了若干别名。

✅ 启用方式:把「ansible」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 命令                                       | 说明                                                                |
|:-------------------------------------------|:--------------------------------------------------------------------|
| `ansible-version` / `aver`                 | 显示此主机上安装的 ansible 版本                                      |
| `ansible-role-init <role name>` / `arinit` | 按照 Ansible Galaxy 标准创建 Ansible Role                            |
| `a`                                        | 命令 `ansible`                                                       |
| `aconf`                                    | 命令 `ansible-config`                                                |
| `acon`                                     | 命令 `ansible-console`                                               |
| `ainv`                                     | 命令 `ansible-inventory`                                             |
| `aplaybook`                                | 命令 `ansible-playbook`                                              |
| `adoc`                                     | 命令 `ansible-doc`                                                   |
| `agal`                                     | 命令 `ansible-galaxy`                                                |
| `apull`                                    | 命令 `ansible-pull`                                                  |
| `aval`                                     | 命令 `ansible-vault`                                                 |

## 维护者

### [Deepankumar](https://github.com/deepan10)

[https://github.com/deepan10/oh-my-zsh/tree/features/ansible-plugin](https://github.com/deepan10/oh-my-zsh/tree/features/ansible-plugin)
