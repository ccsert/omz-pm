# Isodate 插件

本插件为 [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) 日期格式提供补全,
并为常用的 date 命令提供了一些别名。

✅ 启用方式:把「isodate」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

**维护者:** [@Frani](https://github.com/frani)

## 别名

| 别名          | 命令                                 | 说明                                                         |
| ------------- | ------------------------------------ | ------------------------------------------------------------ |
| isodate       | `date +%Y-%m-%dT%H:%M:%S%z`          | 以 UTC 偏移量和 ISO 8601-2 扩展格式显示当前日期              |
| isodate_utc   | `date -u +%Y-%m-%dT%H:%M:%SZ`        | 以 UTC 和 ISO 8601-2 扩展格式显示当前日期                    |
| isodate_basic | `date -u +%Y%m%dT%H%M%SZ`            | 以 UTC 和 ISO 8601 基本格式显示当前日期                      |
| unixstamp     | `date +%s`                           | 以 Unix 时间戳显示当前日期(自 Unix 纪元起的秒数)          |
| date_locale   | `date +"%c"`                         | 以默认区域设置的格式显示当前日期                             |
