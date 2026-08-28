# genpass

本插件为 ZSH 提供三个各具特色的密码生成器。每个生成器都至少有 128 位的安全强度,
并且从密码学上安全的 `/dev/urandom` 生成密码。每个生成器还可以接受一个可选的
数字参数,一次生成多个密码。

✅ 启用方式:把「genpass」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

你也可以直接调用密码生成器(它们被实现为独立的可执行文件),当你需要在脚本里
生成密码时,这样会很方便:

    ~/.oh-my-zsh/plugins/genpass/genpass-apple 3

## genpass-apple

生成可发音的伪词口令,采用 "cvccvc" 的辅音/元音结构,灵感来自
[Apple 的 iCloud 钥匙串密码生成器][1]。每个密码恰好含 1 个数字,放在「单词」的
边缘位置,并且恰好含 1 个大写字母,以满足大多数密码安全要求。

    % genpass-apple
    gelcyv-foqtam-fotqoh-viMleb-lexduv-6ixfuk

    % genpass-apple 3
    japvyz-qyjti4-kajrod-nubxaW-hukkan-dijcaf
    vydpig-fucnul-3ukpog-voggom-zygNad-jepgad
    zocmez-byznis-hegTaj-jecdyq-qiqmiq-5enwom

[1]: https://developer.apple.com/password-rules/

## genpass-monkey

使用 [Crockford 的 base32][2] 生成视觉上无歧义的随机无意义字符串。

    % genpass-monkey
    xt7gn976e7jj3fstgpy27330x3

    % genpass-monkey 3
    n1qqwtzgejwgqve9yzf2gxvx4m
    r2n3f5s6vbqs2yx7xjnmahqewy
    296w9y9rts3p5r9yay0raek8e5

[2]: https://www.crockford.com/base32.html

## genpass-xkcd

从 `/usr/share/dict/words` 生成口令短语,灵感来自那篇[著名(也略有误导性)的
XKCD 漫画][3]。每条口令短语前面都会加一个数字,表示短语中单词的数量,以满足要求
包含数字的密码安全规则。每个单词不超过 6 个字符。

    % genpass-xkcd
    9-eaten-Slav-rife-aired-hill-cordon-splits-welsh-napes

    % genpass-xkcd 3
    9-worker-Vlad-horde-shrubs-smite-thwart-paw-alters-prawns
    9-tutors-stink-rhythm-junk-snappy-hooray-barbs-mewl-clomp
    9-vital-escape-Angkor-Huff-wet-Mayra-abbés-putts-guzzle

[3]: https://xkcd.com/936/
