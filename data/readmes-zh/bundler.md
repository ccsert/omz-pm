# Bundler 插件

本插件为 bundler 的基础命令提供补全,并附带一组别名和辅助函数,让 bundler 用起来更省心。

✅ 启用方式:把「bundler」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名   | 命令                  | 说明                                                      |
| ------ | ----------------- | -------------------------------------------------------- |
| `ba`   | `bundle add`      | 把 gem 添加进 Gemfile 并运行 bundle install               |
| `bck`  | `bundle check`    | 检查依赖是否已由已安装的 gem 满足                          |
| `bcn`  | `bundle clean`    | 清理 bundler 目录中未使用的 gem                            |
| `be`   | `bundle exec`     | 在 bundle 的上下文中执行命令                                |
| `bi`   | `bundle install`  | 安装 Gemfile 中指定的依赖                                   |
| `bl`   | `bundle list`     | 列出 bundle 中的所有 gem                                   |
| `bo`   | `bundle open`     | 打开 bundle 中某个 gem 的源码目录                           |
| `bout` | `bundle outdated` | 列出有更新版本可用的已安装 gem                              |
| `bp`   | `bundle package`  | 把所需的 .gem 文件打包进你的应用                            |
| `bu`   | `bundle update`   | 把 gem 更新到最新的可用版本                                 |
| `bua`  | `bundle update --all` | 把所有 gem 更新到最新的可用版本                        |

## Gem wrapper

本插件为常见的 gem 添加了一层 wrapper,它会:

- 先在 `./bin/` 下寻找 binstub,如果存在就执行它。
- 否则调用 `bundle exec <gem>`。

默认被 wrapper 包裹的常见 gem(按可执行文件名):

`annotate`、`cap`、`capify`、`cucumber`、`foodcritic`、`guard`、`hanami`、`irb`、`jekyll`、`kitchen`、`knife`、`middleman`、`nanoc`、`pry`、`puma`、`rackup`、`rainbows`、`rake`、`rspec`、`rubocop`、`shotgun`、`sidekiq`、`spec`、`spork`、`spring`、`strainer`、`tailor`、`taps`、`thin`、`thor`、`unicorn` 以及 `unicorn_rails`。

### 设置

你可以向被包裹命令的列表中添加或移除 gem。
请**使用可执行文件的确切名称**,而不是 gem 名。

#### 添加要被包裹的 gem(`BUNDLED_COMMANDS`)

把这行加到 `.zshrc` 中插件列表之前:

```sh
BUNDLED_COMMANDS=(rubocop)
plugins=(... bundler ...)
```

这会为 `rubocop` gem(即可执行文件)添加 wrapper。

#### 排除不被包裹的 gem(`UNBUNDLED_COMMANDS`)

把这行加到 `.zshrc` 中插件列表之前:

```sh
UNBUNDLED_COMMANDS=(foreman spin)
plugins=(... bundler ...)
```

这会把 `foreman` 和 `spin` 两个 gem(即它们的可执行文件)排除在 wrapper 之外。

### 被排除的 gem

以下 gem 不应该用 `bundle exec` 来调用。具体原因请参见 GitHub 上的 [issue #2923](https://github.com/ohmyzsh/ohmyzsh/pull/2923):

- `berks`
- `foreman`
- `mailcatcher`
- `rails`
- `ruby`
- `spin`
