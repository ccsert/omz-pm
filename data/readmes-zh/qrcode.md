# QRCode 插件

在命令行生成二维码。通过 curl 使用 [QRcode.show](https://qrcode.show) 服务。

别名            | 命令
--------------- | --------
`qrcode [text]` | `curl -d "text" qrcode.show`
`qrsvg  [text]` | `curl -d "text" qrcode.show -H "Accept: image/svg+xml"`
