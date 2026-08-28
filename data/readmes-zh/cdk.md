# AWS CDK 插件

本插件为 [AWS Cloud Development Kit (CDK)](https://aws.amazon.com/cdk/) CLI 提供别名和自动补全。

## 用法

✅ 启用方式:把「cdk」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 依赖要求

- 已安装 [AWS CDK CLI](https://docs.aws.amazon.com/cdk/v2/guide/getting_started.html)(`npm install -g aws-cdk`)

## 别名

| 别名         | 命令              | 说明                               |
| ------------ | ----------------- | ---------------------------------- |
| `cdkl`       | `cdk list`        | 列出应用中的所有栈                  |
| `cdksynth`   | `cdk synth`       | 合成 CloudFormation 模板            |
| `cdkdiff`    | `cdk diff`        | 比较已部署栈与本地栈的差异          |
| `cdkdeploy`  | `cdk deploy`      | 将栈部署到 AWS                      |
| `cdkdestroy` | `cdk destroy`     | 销毁已部署的栈                      |
| `cdkboot`    | `cdk bootstrap`   | 引导(bootstrap)CDK 环境           |
| `cdkdoc`     | `cdk docs`        | 打开 CDK 文档                       |
| `cdkinit`    | `cdk init`        | 初始化一个新的 CDK 项目             |
| `cdkwatch`   | `cdk watch`       | 监视变更并自动部署                  |
| `cdkctx`     | `cdk context`     | 管理缓存的上下文值                  |
| `cdkack`     | `cdk acknowledge` | 确认一条通知                        |
| `cdkver`     | `cdk --version`   | 打印 CDK 版本                       |
