# aws

本插件为 [awscli v2](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/index.html)
提供补全支持,并附带几个用于管理 AWS profiles/regions 并在提示符中显示它们的小工具。
[awscli v1](https://docs.aws.amazon.com/cli/latest/userguide/cliv2-migration.html) 已不再受支持。

✅ 启用方式:把「aws」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 插件命令

* `asp [<profile>]`:把 `$AWS_PROFILE` 和 `$AWS_DEFAULT_PROFILE`(旧版)设为 `<profile>`。
  同时会把 `$AWS_EB_PROFILE` 设为 `<profile>`,供 Elastic Beanstalk CLI 使用。它还会设置 `$AWS_PROFILE_REGION`,用于在 `aws_prompt_info` 中显示。
  不带参数运行 `asp` 可清除当前 profile。
* `asp [<profile>] login`:如果你的 aws profile 中已配置 AWS SSO,则会在选择 profile 之后运行 `aws sso login` 命令。
* `asp [<profile>] login [<sso_session>]`:在 `asp [<profile>] login` 的基础上,如果你的 aws profile 中还配置了 SSO session,则会在选择 profile 之后运行 `aws sso login --sso-session <sso_session>` 命令。
* `asp [<profile>] logout`:如果你的 aws profile 中已配置 AWS SSO,则会在选择 profile 之后运行 `aws sso logout` 命令。

* `asr [<region>]`:把 `$AWS_REGION` 和 `$AWS_DEFAULT_REGION`(旧版)设为 `<region>`。
  不带参数运行 `asr` 可清除当前 profile。

* `acp [<profile>] [<mfa_token>]`:在 `asp` 功能的基础上,它通过承担 `<profile>` 配置中指定的角色来真正切换
   profile。它支持 MFA,并会在获取到的情况下设置 `$AWS_ACCESS_KEY_ID`、`$AWS_SECRET_ACCESS_KEY` 和
   `$AWS_SESSION_TOKEN`。它要求角色按照
   [官方指南](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-role.html)进行配置。
   不带参数运行 `acp` 可清除当前 profile。

* `agp`:获取 `$AWS_PROFILE` 的当前值。

* `agr`:获取 `$AWS_REGION` 的当前值。

* `aws_change_access_key`:更改某个 profile 的 AWS 访问密钥。

* `aws_profiles`:列出 `$AWS_CONFIG_FILE`(默认:`~/.aws/config`)中可用的 profile。
  用于为 `asp` 函数提供补全。

* `aws_regions`:列出可用的 region。
  用于为 `asr` 函数提供补全。

## 插件选项

* 如果不想让插件修改你的 RPROMPT,可在 zshrc 文件中设置 `SHOW_AWS_PROMPT=false`。
  有些主题会直接覆盖 RPROMPT 的值而不是向其追加,因此需要先把这些主题修好,
  才能看到 AWS profile/region 提示。

* 如果希望 aws profile 在多个 shell 会话之间保持,可在 zshrc 文件中设置 `AWS_PROFILE_STATE_ENABLED=true`。
  该选项可能会拖慢 shell 的启动速度。
  状态文件路径默认为 `/tmp/.aws_current_profile`,也就是说状态在重启或被系统清理后不会保留。
  你可以通过 `AWS_STATE_FILE` 环境变量来控制状态文件的路径。

## 主题

插件会创建一个 `aws_prompt_info` 函数,你可以在主题中使用它,用来显示当前的
`$AWS_PROFILE` 和 `$AWS_REGION`。显示效果由下面四个变量控制:

* ZSH_THEME_AWS_PROFILE_PREFIX:设置 AWS_PROFILE 的前缀,默认为 `<aws:`。

* ZSH_THEME_AWS_PROFILE_SUFFIX:设置 AWS_PROFILE 的后缀,默认为 `>`。

* ZSH_THEME_AWS_REGION_PREFIX:设置 AWS_REGION 的前缀,默认为 `<region:`。

* ZSH_THEME_AWS_REGION_SUFFIX:设置 AWS_REGION 的后缀,默认为 `>`。

* ZSH_THEME_AWS_DIVIDER:设置 ZSH_THEME_AWS_PROFILE_SUFFIX 与 ZSH_THEME_AWS_REGION_PREFIX 之间的分隔符,默认为 ` `(单个空格)。

## 配置

AWS 官方的[配置与凭证文件设置](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html)

### 场景:使用 source profile 和 MFA 认证的 IAM 角色

`~/.aws/credentials` 中的 source profile 凭证:

```ini
[source-profile-name]
aws_access_key_id = ...
aws_secret_access_key = ...
```

`~/.aws/config` 中的角色配置:

```ini
[profile source-profile-name]
mfa_serial = arn:aws:iam::111111111111:mfa/myuser
region = us-east-1
output = json

[profile profile-with-role]
role_arn = arn:aws:iam::9999999999999:role/myrole
mfa_serial = arn:aws:iam::111111111111:mfa/myuser
source_profile = source-profile-name
region = us-east-1
output = json
```
