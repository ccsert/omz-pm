# dotenv

当你 `cd` 进项目根目录时,自动从 `.env` 文件加载项目的 ENV 变量。

将配置存储在环境变量中是[十二要素应用(twelve-factor app)](https://www.12factor.net)的原则之一。任何可能随部署环境变化的东西,比如数据库的资源句柄或外部服务的凭证,都应该从代码中抽取出来,放进环境变量。

✅ 启用方式:把「dotenv」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

在项目根目录下创建 `.env` 文件,把你的 ENV 变量放进去。

例如:

```sh
export AWS_S3_TOKEN=d84a83539134f28f412c652b09f9f98eff96c9a
export SECRET_KEY=7c6c72d959416d5aa368a409362ec6e2ac90d7f
export MONGO_URI=mongodb://127.0.0.1:27017
export PORT=3001
```

`export` 可以省略,下面这种格式也可以:

```sh
AWS_S3_TOKEN=d84a83539134f28f412c652b09f9f98eff96c9a
SECRET_KEY=7c6c72d959416d5aa368a409362ec6e2ac90d7f
MONGO_URI=mongodb://127.0.0.1:27017
PORT=3001
```

你甚至可以混用两种格式,不过这恐怕不是个好主意。

用引号包裹的字符串可以表示多行值:

```sh
PRIVATE_KEY="-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEA...
-----END RSA PRIVATE KEY-----"
```

文件中先定义的变量,可以被后面的条目引用:

```sh
BASE_URL=https://example.com
API_URL=$BASE_URL/api
ASSETS_URL=${BASE_URL}/assets
```

注意:只有同一个 `.env` 文件内定义的变量才会这样展开——已经存在的 shell 环境变量**不会**被替换。

## 设置

### ZSH_DOTENV_FILE

你也可以用变量 `ZSH_DOTENV_FILE` 修改要加载的文件名。
如果该变量未设置,插件默认使用 `.env`。
例如,下面的设置会让插件查找并加载名为 `.dotenv` 的文件:

```zsh
# in ~/.zshrc, before Oh My Zsh is sourced:
ZSH_DOTENV_FILE=.dotenv
```

### ZSH_DOTENV_PROMPT

如果不想看到确认提示,可以在 zshrc 文件中设置 `ZSH_DOTENV_PROMPT=false`。
你也可以在出现提示时选择 `Always` 选项,从此始终允许在该目录中 source .env 文件。
更多细节见下一节。

### ZSH_DOTENV_ALLOWED_LIST, ZSH_DOTENV_DISALLOWED_LIST

插件的默认行为是每次都询问是否 source 一个 dotenv 文件,共有 **Y**es、**N**o、**A**lways 和 N**e**ver 四个选项。如果选 Always,.env 文件所在目录会被加入允许列表;如果选 Never,则会被加入禁止列表。只要目录出现在这两个列表之一,插件就不再询问确认,而是相应地直接 source 该 .env 文件,或者不做任何处理继续。

允许列表和禁止列表默认分别保存在 `$ZSH_CACHE_DIR/dotenv-allowed.list` 和
`$ZSH_CACHE_DIR/dotenv-disallowed.list`。如果想改位置,
可以修改 `$ZSH_DOTENV_ALLOWED_LIST` 和 `$ZSH_DOTENV_DISALLOWED_LIST` 变量,如下:

```zsh
# in ~/.zshrc, before Oh My Zsh is sourced:
ZSH_DOTENV_ALLOWED_LIST=/path/to/dotenv/allowed/list
ZSH_DOTENV_DISALLOWED_LIST=/path/to/dotenv/disallowed/list
```

该文件只是一个目录列表,每行一个目录。如果想改变之前的决定,直接编辑该文件,删掉你想更改的那个目录所在行即可。

注意:如果一个目录同时出现在允许列表和禁止列表中,则以禁止列表为准,_也就是说_该 .env 文件永远不会被 source。

## 命名管道(FIFO)支持

除了常规文件,插件还支持以 UNIX 命名管道(FIFO)形式提供的 `.env` 文件。
当 [1Password Environments](https://developer.1password.com/docs/environment/)
这类密钥管理工具把 `.env` 文件挂载为命名管道、即时注入密钥而不落盘时,这一功能很有用。

无需任何额外配置——插件会自动检测并 source 命名管道。

## 测试

测试使用 [zunit](https://github.com/zunit-zsh/zunit)。按照其[文档](https://github.com/zunit-zsh/zunit#installation)安装后,运行:

```sh
cd plugins/dotenv && zunit
```

> [注意!]
> zunit 还需要安装 [Revolver](https://github.com/molovo/revolver)。

## 版本控制

**强烈建议把 `.env` 文件加入 `.gitignore`**,因为它通常包含凭证、密钥、密码等敏感信息。你不会想提交这个文件的,它只应存在于本地。

## 安全

加载 `.env` 文件时,插件会应用多项尽力而为的安全防护:

- **大小限制** —— 超过 10 MiB 的文件会被拒绝,以防止 DoS 攻击。
- **语法检查** —— 在设置任何变量之前,先用 `zsh -fn` 校验文件。
- **禁止命令替换** —— 包含 `$(...)` 或反引号结构的条目会被跳过。
- **禁改变量** —— 无论 `.env` 文件内容如何,以下变量都不会被覆盖:`NODE_OPTIONS`、`BASH_ENV`、
  `ENV`、`ZDOTDIR`、`ZSH`、`LD_PRELOAD`、
  `LD_LIBRARY_PATH`、`DYLD_INSERT_LIBRARIES`、`GIT_CONFIG_GLOBAL`、`GIT_DIR`、`GIT_EDITOR`,
  `GIT_EXTERNAL_DIFF`、`GIT_EXEC_PATH`、`GIT_PAGER`、`GIT_SSH`、`GIT_SSH_COMMAND`,
  `GIT_SSL_NO_VERIFY`、`GIT_TEMPLATE_DIR`、`VISUAL`、`PAGER`、`EDITOR`,以及所有 zsh 特殊参数。

这些措施都只是**尽力而为**——`.env` 文件的内容仍由你自己负责。不要把这个插件当作安全边界。

如果你需要更高级、功能更丰富的 ENV 管理方案,可以看看这些优秀的项目:

* [direnv](https://github.com/direnv/direnv)
* [zsh-autoenv](https://github.com/Tarrasch/zsh-autoenv)
