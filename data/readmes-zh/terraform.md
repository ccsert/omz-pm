# Terraform 插件

Terraform 插件。Terraform 是 Hashicorp 出品的用于安全高效地管理基础设施的工具。本插件为
`terraform` 提供补全,还提供别名和一个提示符函数。

✅ 启用方式:把「terraform」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 依赖要求

- [Terraform](https://terraform.io/)

## 别名

| 别名    | 命令                                   |
|---------|----------------------------------------|
| `tf`    | `terraform`                            |
| `tfa`   | `terraform apply`                      |
| `tfa!`  | `terraform apply -auto-approve`        |
| `tfap`  | `terraform apply -parallelism=1`       |
| `tfc`   | `terraform console`                    |
| `tfd`   | `terraform destroy`                    |
| `tfd!`  | `terraform destroy -auto-approve`      |
| `tfdp`  | `terraform destroy -parallelism=1`     |
| `tff`   | `terraform fmt`                        |
| `tffr`  | `terraform fmt -recursive`             |
| `tfi`   | `terraform init`                       |
| `tfir`  | `terraform init -reconfigure`          |
| `tfiu`  | `terraform init -upgrade`              |
| `tfiur` | `terraform init -upgrade -reconfigure` |
| `tfo`   | `terraform output`                     |
| `tfp`   | `terraform plan`                       |
| `tfv`   | `terraform validate`                   |
| `tfs`   | `terraform state`                      |
| `tft`   | `terraform test`                       |
| `tfsh`  | `terraform show`                       |
| `tfw`   | `terraform workspace`                  |
| `tfwl`  | `terraform workspace list`             |
| `tfws`  | `terraform workspace select`           |

## 提示符函数

你可以把当前 Terraform workspace 加进提示符,方法是把 `$(tf_prompt_info)`、
`$(tf_version_prompt_info)` 添加到你的 `PROMPT` 或 `RPROMPT` 变量中。

```sh
RPROMPT='$(tf_prompt_info)'
RPROMPT='$(tf_version_prompt_info)'
```

你还可以用以下变量为 workspace 指定 PREFIX 和 SUFFIX:

```sh
ZSH_THEME_TF_PROMPT_PREFIX="%{$fg[white]%}"
ZSH_THEME_TF_PROMPT_SUFFIX="%{$reset_color%}"
ZSH_THEME_TF_VERSION_PROMPT_PREFIX="%{$fg[white]%}"
ZSH_THEME_TF_VERSION_PROMPT_SUFFIX="%{$reset_color%}"
```
