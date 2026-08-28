# .NET CLI 插件

本插件为 [.NET CLI](https://dotnet.microsoft.com/) 提供自动补全和实用的别名。

✅ 启用方式:把「dotnet」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名  | 命令              | 说明                                                              |
|-------|------------------|-------------------------------------------------------------------|
| dn    | dotnet new       | 创建新的 .NET 项目或文件。                                        |
| dr    | dotnet run       | 构建并运行 .NET 项目的产出物。                                    |
| dt    | dotnet test      | 使用 .NET 项目中指定的测试运行器执行单元测试。                    |
| dw    | dotnet watch     | 监视源文件变更并重启 dotnet 命令。                                |
| dwr   | dotnet watch run | 监视源文件变更并重启 `run` 命令。                                 |
| dwt   | dotnet watch test| 监视源文件变更并重启 `test` 命令。                                |
| ds    | dotnet sln       | 修改 Visual Studio 解决方案文件。                                 |
| da    | dotnet add       | 向 .NET 项目添加包或引用。                                        |
| dp    | dotnet pack      | 创建 NuGet 包。                                                   |
| dng   | dotnet nuget     | 提供其他 NuGet 命令。                                             |
| db    | dotnet build     | 构建 .NET 项目                                                    |
| dres  | dotnet restore   | 为项目还原依赖项和项目专用工具。                                  |
