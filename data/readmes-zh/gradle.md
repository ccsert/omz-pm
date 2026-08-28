# Gradle 插件

本插件为 [Gradle](https://gradle.org/) 提供补全和别名。

✅ 启用方式:把「gradle」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

本插件会创建一个名为 `gradle-or-gradlew` 的函数,并把 `gradle` 别名指向它,用于判断当前
项目目录下是否存在 gradlew 文件:如果存在 `gradlew` 就使用它,否则改用 `gradle`。这样
执行 Gradle 任务时不必关心用的到底是 `gradle` 还是 `gradlew`。它还支持从根项目目录内的
任意目录调用。

示例:

```zsh
gradle test
gradle build
```

## 补全

本插件使用 [Gradle 项目提供的补全](https://github.com/gradle/gradle-completion),
该补全以 MIT 许可证分发。
