# heroku-alias 插件

Heroku CLI 的完整别名列表。

✅ 启用方式:把「heroku-alias」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 依赖要求

- [Heroku CLI](https://devcenter.heroku.com/articles/heroku-cli)

| 🚀 最近大版本更新 | 📡 来源                                                                      |
| ---------- | --------------------------------------------------------------------------- |
| 02/06/2020 | [heroku cli 文档](https://devcenter.heroku.com/articles/heroku-cli-commands) |

## 别名

### 通用

| 别名  | 命令                               |
| ----- | ---------------------------------- |
| h     | heroku                             |
| hauto | heroku autocomplete $(echo $SHELL) |
| hl    | heroku local                       |

### 配置

| 别名   | 命令                   |
| ------ | ---------------------- |
| hc     | heroku config          |
| hca    | heroku config -a       |
| hcr    | heroku config -r       |
| hcs    | heroku config:set      |
| hcu    | heroku config:unset    |

此外,你还可以使用 `hcfile` 函数从文件中设置多个配置变量,
它会要求你提供一个平台(Platform)以及一个用于读取配置的配置文件。

### 应用与收藏

| 别名  | 命令                         |
| ----- | ---------------------------- |
| ha    | heroku apps                  |
| hpop  | heroku create                |
| hkill | heroku apps:destroy          |
| hlog  | heroku apps:errors           |
| hfav  | heroku apps:favorites        |
| hfava | heroku apps:favorites:add    |
| hfavr | heroku apps:favorites:remove |
| hai   | heroku apps:info             |
| hair  | heroku apps:info -r          |
| haia  | heroku apps:info -a          |

## 认证

| 别名  | 命令                    |
| ----- | ----------------------- |
| h2fa  | heroku auth:2fa         |
| h2far | heroku auth:2fa:disable |

## 访问权限

| 别名  | 命令                 |
| ----- | -------------------- |
| hac   | heroku access        |
| hacr  | heroku access -r     |
| haca  | heroku access -a     |
| hadd  | heroku access:add    |
| hdel  | heroku access:remove |
| hup   | heroku access:update |

### 附加组件

| 别名  | 命令                  |
| ----- | --------------------- |
| hads  | heroku addons -A      |
| hada  | heroku addons -a      |
| hadr  | heroku addons -r      |
| hadat | heroku addons:attach  |
| hadc  | heroku addons:create  |
| hadel | heroku addons:destroy |
| hadde | heroku addons:detach  |
| hadoc | heroku addons:docs    |

### 登录

| 别名 | 命令               |
| ---- | ------------------ |
| hin  | heroku login       |
| hout | heroku logout      |
| hi   | heroku login -i    |
| hwho | heroku auth:whoami |

### 授权

| 别名   | 命令                         |
| ------ | ---------------------------- |
| hth    | heroku authorizations        |
| hthadd | heroku authorizations:create |
| hthif  | heroku authorizations:info   |
| hthdel | heroku authorizations:revoke |
| hthrot | heroku authorizations:rotate |
| hthup  | heroku authorizations:update |

### 插件

| 别名 | 命令           |
| ---- | -------------- |
| hp   | heroku plugins |

### 日志

| 别名 | 命令            |
| ---- | --------------- |
| hg   | heroku logs     |
| hgt  | heroku log tail |

### 数据库

| 别名  | 命令                       |
| ----- | -------------------------- |
| hpg   | heroku pg                  |
| hpsql | heroku pg:psql             |
| hpb   | heroku pg:backups          |
| hpbc  | heroku pg:backups:capture  |
| hpbd  | heroku pg:backups:download |
| hpbr  | heroku pg:backups:restore  |

### 证书

| 别名  | 命令                |
| ----- | ------------------- |
| hssl  | heroku certs        |
| hssli | heroku certs:info   |
| hssla | heroku certs:add    |
| hsslu | heroku certs:update |
| hsslr | heroku certs:remove |
