# extract 插件

本插件定义了一个名为 `extract` 的函数,用于解压你传给它的压缩归档文件,它支持种类繁多的
归档文件格式。

这样一来,你不必知道解压某种文件需要哪条具体命令,只要执行 `extract <文件名>`,
函数就会搞定剩下的事。

✅ 启用方式:把「extract」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 支持的文件扩展名

| 扩展名            | 说明                                    |
| :---------------- | :-------------------------------------- |
| `7z`              | 7zip 压缩文件                           |
| `apk`             | Android 应用文件                        |
| `aar`             | Android 库文件                          |
| `bz2`             | Bzip2 压缩文件                          |
| `cab`             | Microsoft cabinet 归档                  |
| `cpio`            | cpio 归档                               |
| `deb`             | Debian 软件包                           |
| `ear`             | 企业应用归档(Enterprise Application aRchive) |
| `exe`             | Windows 可执行文件                      |
| `gz`              | Gzip 压缩文件                           |
| `ipa`             | iOS 应用包                              |
| `ipsw`            | iOS 固件文件                            |
| `jar`             | Java 归档(Java Archive)               |
| `lrz`             | LRZ 归档                                |
| `lz4`             | LZ4 归档                                |
| `lzma`            | LZMA 归档                               |
| `obscpio`         | OBS 上使用的 cpio 归档                  |
| `pk3`             | Quake 游戏使用的改名 Zip 归档           |
| `pk4`             | Quake 游戏使用的改名 Zip 归档           |
| `pk7`             | Quake 游戏使用的改名 7zip 文件          |
| `rar`             | WinRAR 归档                             |
| `rpm`             | RPM 软件包                              |
| `sublime-package` | Sublime Text 插件包                     |
| `tar`             | tar 包(tarball)                       |
| `tar.bz2`         | 经 bzip2 压缩的 tar 包                  |
| `tar.gz`          | 经 gzip 压缩的 tar 包                   |
| `tar.lrz`         | 经 lrzip 压缩的 tar 包                  |
| `tar.lz`          | 经 lzip 压缩的 tar 包                   |
| `tar.lz4`         | 经 lz4 压缩的 tar 包                    |
| `tar.xz`          | 经 lzma2 压缩的 tar 包                  |
| `tar.zma`         | 经 lzma 压缩的 tar 包                   |
| `tar.zst`         | 经 zstd 压缩的 tar 包                   |
| `tbz`             | 经 bzip 压缩的 tar 包                   |
| `tbz2`            | 经 bzip2 压缩的 tar 包                  |
| `tgz`             | 经 gzip 压缩的 tar 包                   |
| `tlz`             | 经 lzma 压缩的 tar 包                   |
| `txz`             | 经 lzma2 压缩的 tar 包                  |
| `tzst`            | 经 zstd 压缩的 tar 包                   |
| `vsix`            | VS Code 扩展的 zip 文件                 |
| `war`             | Web 应用归档(基于 Java)               |
| `whl`             | Python wheel 文件                       |
| `xpi`             | Mozilla XPI 模块文件                    |
| `xz`              | LZMA2 归档                              |
| `Z`               | Z 归档(LZW)                           |
| `zip`             | Zip 归档                                |
| `zlib`            | zlib 归档                               |
| `zst`             | Zstandard 文件(zstd)                  |
| `zpaq`            | Zpaq 文件                               |

关于归档格式的更多信息,参见[归档格式列表](https://en.wikipedia.org/wiki/List_of_archive_formats)。
