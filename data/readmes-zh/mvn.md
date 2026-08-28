# mvn 插件

mvn 插件提供了许多[实用别名](#aliases),并为
[Apache Maven](https://maven.apache.org/) 命令(`mvn`)提供自动补全。

✅ 启用方式:把「mvn」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

插件会把 mvn 别名指向一个函数:如果找到 `mvnw`(即 [Maven Wrapper](https://github.com/takari/maven-wrapper)),
就调用它,否则调用 mvn 命令。

| 别名                 | 命令                                            |
|:---------------------|:------------------------------------------------|
| `mvn!`               | `mvn -f <root>/pom.xml`                         |
| `mvnag`              | `mvn archetype:generate`                        |
| `mvnboot`            | `mvn spring-boot:run`                           |
| `mvnqdev`            | `mvn quarkus:dev`                               |
| `mvnc`               | `mvn clean`                                     |
| `mvncd`              | `mvn clean deploy`                              |
| `mvnce`              | `mvn clean eclipse:clean eclipse:eclipse`       |
| `mvnci`              | `mvn clean install`                             |
| `mvncie`             | `mvn clean install eclipse:eclipse`             |
| `mvncini`            | `mvn clean initialize`                          |
| `mvncist`            | `mvn clean install -DskipTests`                 |
| `mvncisto`           | `mvn clean install -DskipTests --offline`       |
| `mvncom`             | `mvn compile`                                   |
| `mvncp`              | `mvn clean package`                             |
| `mvnct`              | `mvn clean test`                                |
| `mvncv`              | `mvn clean verify`                              |
| `mvncvst`            | `mvn clean verify -DskipTests`                  |
| `mvnv`               | `mvn verify`                                    |
| `mvnvst`             | `mvn verify -DskipTests`                        |
| `mvndp`              | `mvn deploy`                                    |
| `mvndocs`            | `mvn dependency:resolve -Dclassifier=javadoc`   |
| `mvndt`              | `mvn dependency:tree`                           |
| `mvne`               | `mvn eclipse:eclipse`                           |
| `mvnfmt`             | `mvn fmt:format`                                |
| `mvnjetty`           | `mvn jetty:run`                                 |
| `mvnp`               | `mvn package`                                   |
| `mvns`               | `mvn site`                                      |
| `mvnsrc`             | `mvn dependency:sources`                        |
| `mvnt`               | `mvn test`                                      |
| `mvntc`              | `mvn tomcat:run`                                |
| `mvntc7`             | `mvn tomcat7:run`                               |
| `mvn-updates`        | `mvn versions:display-dependency-updates`       |

## mvn-color

它是一个包装 mvn 命令的函数,用于给输出上色。你可以在需要用 `mvn` 命令的地方改用它。
例如:不用 `mvn test`,而用 `mvn-color test`。

自 [Maven 3.5.0](https://maven.apache.org/docs/3.5.0/release-notes.html) 起,mvn 命令
本身就带彩色输出了,所以这个函数很快就会从插件中移除。

### 已知问题

它有一个 bug:会吞掉 mvn 提示用户输入的部分,例如使用
`archetype:generate` 时。参见 [#5052](https://github.com/ohmyzsh/ohmyzsh/issues/5052)。
