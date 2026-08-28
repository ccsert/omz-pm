# jsontools

一套处理 JSON 数据的实用命令行工具。

✅ 启用方式:把「jsontools」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 依赖要求

本插件按以下顺序选用其中一个工具来处理 JSON 数据:

- `node`
- `python3`
- `ruby`

其中任意一个都必须在插件加载前位于 `$PATH` 中,否则插件会提前退出,这些函数将不可用。

## 用法

用法很简单……把你的 JSON 数据通过管道送进相应的 jsontool 即可:

- `pp_json`:以美化过的格式打印 json。
- `is_json`:如果是合法 json 则返回 true;否则返回 false。
- `urlencode_json`:对给定的 json 返回 URL 编码后的字符串。
- `urldecode_json`:对给定的 URL 编码字符串返回解码后的 json。

### 支持 NDJSON(换行分隔的 JSON)

本插件还支持 [NDJSON](https://github.com/ndjson/ndjson-spec) 输入,也就是说所有函数
都有一个逐行读取并处理输入的替代版本。这些函数的名称相同,只是把 `json` 换成了 `ndjson`:

> `pp_ndjson`、`is_ndjson`、`urlencode_ndjson`、`urldecode_ndjson`。

### 示例

- **pp_json**:

```console
# curl json data and pretty print the results
curl https://coderwall.com/bobwilliams.json | pp_json
```

- **is_json**:

```console
# validate if file's content conforms to a valid JSON schema
$ is_json < data.json
true
# shows true / false and returns the proper exit code
$ echo $?
0
```

- **urlencode_json**:

```console
# json data directly from the command line
$ echo '{"b":2, "a":1}' | urlencode_json
%7B%22b%22:2,%20%22a%22:1%7D
```

- **urldecode_json**:

```console
# url encoded string to decode
$ echo '%7B%22b%22:2,%20%22a%22:1%7D' | urldecode_json
{"b":2, "a":1}
```

- **pp_ndjson**:

```console
# echo two separate json objects and pretty print both
$ echo '{"a": "b"}\n{"c": [1,2,3]}' | pp_ndjson
{
    "a": "b"
}
{
    "c": [
        1,
        2,
        3
    ]
}
```
