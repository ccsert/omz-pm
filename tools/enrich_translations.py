#!/usr/bin/env python3
"""离线增强 data/translations.json:
1. 为每条词典条目打分类标签(cat)——手工指定 + 关键词规则,烘进 JSON;
2. 为常用插件补充实战用法(usage)与别名中文注解(aliases);
3. 修正若干事实错误。
可重复执行(idempotent)。"""
import json
import re
import sys
from pathlib import Path

DATA = Path(__file__).resolve().parent.parent / "data" / "translations.json"

# ---------------- 分类 ----------------
# 手工指定(每类别一行,空格分隔插件名)
CATS = {
    "版本控制": "git git-auto-fetch git-commit git-escape-magic git-extras git-flow git-flow-avh git-hubflow git-lfs git-prompt gitfast github gitignore jj mercurial svn svn-fast-info fossil repo gas branch tig",
    "目录与跳转": "z zoxide autojump fasd jump wd scd pj dircycle dirhistory dirpersist last-working-dir zsh-interactive-cd",
    "历史与搜索": "history history-substring-search per-directory-history fzf percol web-search frontend-search man tldr",
    "快捷键与效率": "alias-finder aliases colemak copybuffer copyfile copypath encode64 globalias isodate jsontools magic-enter safe-paste fancy-ctrl-z urltools genpass qrcode timer bgnotify command-not-found sudo thefuck singlechar common-aliases gnu-utils sprunge magic-enter",
    "文件与压缩": "cp extract universalarchive torrent rsync",
    "容器与云运维": "docker docker-compose podman k9s kind kompose kops kube-ps1 kubectl kubectx microk8s minikube kn istioctl skaffold helm fluxcd argocd timoni svcat nomad toolbox multipass lxd vagrant vagrant-prompt aws azure cloudfoundry doctl gcloud hcloud heroku heroku-alias scw sfdx oc jfrog kamal tugboat localstack terraform opentofu pulumi vault ansible chef knife knife_ssh salt kate none",
    "编程语言与框架": "python pip pipenv poetry poetry-env pylint pep8 autopep8 conda conda-env virtualenv virtualenvwrapper uv node nodenv npm nvm volta deno yarn bun jake-node nestjs ng react-native ember-cli gatsby coffee ruby rvm rbenv gem bundler rails rake rake-fast laravel laravel4 laravel5 symfony symfony2 symfony6 yii yii2 cake cakephp3 composer drush eecms hanami homestead invoke spring zeus thor lein cabal scala sbt gradle grails mvn ant bazel mix mix-fast rebar swiftpm xcode meteor nanoc bridgetown flutter ionic perl cpanm paver celery golang rust hasura pod glassfish apache2-macports posh none",
    "包管理与环境": "asdf mise sdk brew cask macports debian ubuntu dnf yum snap suse fnm jenv chruby pyenv nvm",
    "网络与远程": "ssh ssh-agent keychain shell-proxy mosh nmap tailscale wakeonlan httpie ngrok ipfs rsync droplr transfer",
    "编辑器与终端": "emacs vim-interaction vundle sublime sublime-merge textmate marked2 marktext textastic kate forklift vscode iterm2 kitty foot fbterm screen tmux tmux-cssh tmuxinator terminitor bbedit",
    "提示符与外观": "colored-man-pages colorize grc starship shrink-path emoji emoji-clock emotty",
    "数据库与数据": "postgres redis-cli mysql-macports mongocli mongo-atlas dbt",
    "安全与密码": "pass pass-cli rbw otp gpg-agent sigstore",
    "测试与质量": "molecule kitchen codeclimate qodana pre-commit",
    "趣味与语录": "chucknorris hitchhiker octozen lol hitokoto catimg rand-quote",
    "系统管理": "macos battery systemd firewalld ufw supervisor profiles systemadmin term_tab",
    "文档与笔记": "geeknote dnote",
}
MANUAL = {}
for cat, names in CATS.items():
    for n in names.split():
        if n == "none":
            continue
        MANUAL[n] = cat
MANUAL.pop("kate", None)          # kate 是编辑器
MANUAL["kate"] = "编辑器与终端"
MANUAL["rsync"] = "网络与远程"     # 传输优先于文件
MANUAL["n98-magerun"] = "补全增强"  # Magento CLI 补全,非安全类
MANUAL["transfer"] = "网络与远程"
MANUAL["sprunge"] = "快捷键与效率"

# 关键词规则:按顺序首个命中生效(匹配 名称+summary+detail)
RULES = [
    (r"git|subversion|mercurial|分支|版本控制", "版本控制"),
    (r"kubernetes|docker|集群|helm|k8s|云平台|云服务|kops|devops|ci ?/?cd|部署|基础设施", "容器与云运维"),
    (r"python|pip|node|npm|ruby|java|scala|rust|golang|php|elixir|swift|flutter|框架|react|angular|ionic|meteor", "编程语言与框架"),
    (r"包管理|版本管理|sdkman|homebrew|apt|dnf|yum|snap|zypper", "包管理与环境"),
    (r"数据库|postgres|mysql|mongo|redis", "数据库与数据"),
    (r"密码|密钥|签名|gpg|agent|证书|vault", "安全与密码"),
    (r"ssh|代理|proxy|内网|防火墙|vpn|网络|上传|分享", "网络与远程"),
    (r"目录|跳转|书签|路径", "目录与跳转"),
    (r"历史|搜索|检索", "历史与搜索"),
    (r"编辑器|终端|vim|emacs|sublime|vscode|tmux|kitty|iterm", "编辑器与终端"),
    (r"补全|completion", "补全增强"),
    (r"提示符|prompt|主题|外观|颜色|高亮", "提示符与外观"),
    (r"别名|快捷|键位|剪贴板|复制|粘贴|编码|解码|时间戳|密码生成", "快捷键与效率"),
    (r"压缩|解压|归档|文件", "文件与压缩"),
    (r"测试|lint|质量|检查", "测试与质量"),
    (r"笔记|文档|手册|man|帮助", "文档与笔记"),
    (r"笑话|语录|名言|emoji|表情|禅|娱乐|卖萌|二维码", "趣味与语录"),
    (r"系统|服务|macos|linux|防火墙|进程", "系统管理"),
    (r"任务|时间|待办|追踪", "快捷键与效率"),
]

def classify(name: str, entry) -> str:
    if name in MANUAL:
        return MANUAL[name]
    text = f"{name} {entry.get('summary','')} {entry.get('detail','')}"
    for pat, cat in RULES:
        if re.search(pat, text, re.I):
            return cat
    return "其他工具"

# ---------------- 实战用法 + 别名中文注解 ----------------
USAGE = {
    "git": {
        "usage": "启用后无需任何配置即可使用缩写别名。日常高频组合:gst 看状态 → ga/gaa 加暂存 → gc! 写提交 → gp 推送;gco - 切回上一分支;gd 看未暂存差异;glg 看简洁提交图。完整别名列表见下方「别名」区。",
        "aliases": {
            "gst": "查看工作区状态",
            "gss": "简洁模式的状态(一行一个文件)",
            "ga": "加入暂存区(git add)",
            "gaa": "全部加入暂存(含新文件)",
            "gco": "切换分支(git checkout)",
            "gcb": "新建并切换到分支",
            "gb": "列出本地分支",
            "gba": "列出含远程的所有分支",
            "gc!": "把上一次提交改为当前暂存(amend)",
            "gcm": "提交到 main/master",
            "gd": "查看差异",
            "gds": "差异 + 暂存区对比",
            "gf": "拉取远程更新(fetch)",
            "gfa": "拉取所有远程并清理已删分支",
            "glg": "简洁提交历史(带改动统计)",
            "gm": "合并分支(merge)",
            "grb": "变基(rebase)",
            "gps": "推送到远程(push)",
            "gpl": "拉取并合并(pull)",
            "gup": "变基方式拉取(pull --rebase)",
        },
    },
    "fzf": {
        "usage": "装好后立刻可用三个快捷键:\n· Ctrl+R —— 模糊搜索历史命令,输入片段即筛选,回车执行;\n· Ctrl+T —— 模糊选择当前目录下的文件/目录,把路径插入到命令行光标处(如先敲 vim 再按 Ctrl+T);\n· Alt+C —— 模糊选择并 cd 进入某个子目录。\n可用 FZF_DEFAULT_OPTS 调外观,FZF_CTRL_T_OPTS 等变量定制各弹窗。",
    },
    "z": {
        "usage": "无需配置。正常 cd 使用一段时间后(它记录访问记录),直接:z proj 跳到最常去的名字含 proj 的目录;z 不带参数列出权重表。匹配的是「 frecency 」频率×最近度,用得越多越准。",
    },
    "zoxide": {
        "usage": "需先安装 zoxide(brew install zoxide)。启用后 cd 被智能接管:cd proj 等价于 zoxide 智能匹配;zi 可弹出交互选择列表。",
    },
    "autojump": {
        "usage": "需先安装 autojump。正常使用若干目录后:j <关键词> 直接跳到最匹配的历史目录;jo <关键词> 用文件管理器打开该目录;j --stat 查看权重统计。",
    },
    "wd": {
        "usage": "目录书签三步走:\n1) 在某项目目录里执行 wd add myproj 登记书签;\n2) 任何位置执行 wd myproj 秒回该目录;\n3) wd rm myproj 删除,wd list 查看全部书签。",
    },
    "jump": {
        "usage": "类似 wd 的书签跳转:jump mark api 把当前目录标记为 api,之后 jump api 跳回;jump list 列出标记,jump del api 删除。",
    },
    "fasd": {
        "usage": "需先安装 fasd。它会统计文件/目录的访问频率:z conf 跳目录、f conf 选文件、v conf 用编辑器打开(需先设置 $EDITOR)。",
    },
    "extract": {
        "usage": "一条命令解压一切:extract 压缩包.tar.gz、extract xxx.zip、extract xxx.rar…格式自动识别,参数不用记;加到 PATH 的 unar/7z 等越多支持的格式越全。",
    },
    "universalarchive": {
        "usage": "一条命令压缩:ca 输出名.tar.gz 目录/ 或 ca backup.zip 文件们…输出格式的后缀决定压缩算法(zip/tar.gz/tar.xz/zstd 等都支持)。",
    },
    "copypath": {
        "usage": "copypath 把当前目录绝对路径复制进剪贴板;copypath 某文件 复制该文件的绝对路径,粘到聊天里/别的终端直接可用。",
    },
    "copyfile": {
        "usage": "copyfile 文件.txt 把整个文件内容复制到剪贴板(等价 cat 文件 | clipcopy)。",
    },
    "copybuffer": {
        "usage": "命令行里正在输入一条很长的命令、却想先去别的窗口跑?按 Ctrl+O 把当前输入整行复制到系统剪贴板。",
    },
    "dirhistory": {
        "usage": "纯键盘目录导航:\n· Alt+← / Alt+→ —— 目录历史后退 / 前进;\n· Alt+↑ —— 进入上级目录;\n· Alt+↓ —— 进入第一个子目录。",
    },
    "dircycle": {
        "usage": "用 Ctrl+Shift+← / Ctrl+Shift+→ 在最近访问过的目录栈里循环切换(等效 pushd/popd 但一键完成)。",
    },
    "history-substring-search": {
        "usage": "输入任意片段(不必是开头,如 status)后按 ↑/↓,在所有包含该片段的历史命令间上下翻找,回车执行。",
    },
    "per-directory-history": {
        "usage": "默认仍用全局历史;按 Ctrl+G 切到「当前目录历史」模式(提示符会变色),此时 ↑↓ 只翻这个目录里执行过的命令。适合在同一个目录里重复某类操作。",
    },
    "safe-paste": {
        "usage": "零学习成本:粘贴多行脚本时不会因回车符被立即执行,而是整段进入编辑区,检查无误后再回车。防呆神器。",
    },
    "fancy-ctrl-z": {
        "usage": "在 vim/less 等程序里按 Ctrl+Z 挂起;回到 shell 干点别的事后,再按一次 Ctrl+Z 直接回到刚才挂起的程序,不用敲 fg。",
    },
    "sudo": {
        "usage": "敲完一条命令才发现要 root?在空命令行或刚敲完的行上连按两次 Esc,自动在行首补上 sudo。",
    },
    "thefuck": {
        "usage": "需先安装 thefuck 包。敲错命令(如忘了 sudo、把 apt-get 用在 mac 上)后按 ESC ESC,自动给出修正命令并执行;也可手动输入 fuck。",
    },
    "magic-enter": {
        "usage": "空行按回车不再空转:git 仓库里自动显示 git status,普通目录自动 ls;可用 MAGIC_ENTER_GIT_COMMAND / MAGIC_ENTER_OTHER_COMMAND 自定义。",
    },
    "command-not-found": {
        "usage": "输入不存在的命令时,自动提示「该命令在哪个软件包里、如何安装」。macOS 走 Homebrew,Linux 走各自的包管理数据库。",
    },
    "colored-man-pages": {
        "usage": "零配置:启用后所有 man 手册页自动上色——标题、选项、正文分色,长手册一眼抓到重点。",
    },
    "jsontools": {
        "usage": "管道式 JSON 工具,搭配 curl 食用:curl .../api | pp_json 美化输出;echo '{\"a\":1}' | isjson 校验合法性;另有 urlencode 系列、to_json/from_json 等,接口调试不再裸看一行长 JSON。",
    },
    "encode64": {
        "usage": "echo -n 'hello' | e64 编码;d64 解码(反向);全称 encode64 / decode64。常用于 http Basic 认证等小场景。",
    },
    "urltools": {
        "usage": "urlencode '中文参数' 把文本转成 %XX 百分号编码;decodeurl 反向还原。拼带中文/空格/特殊字符的 URL 参数不再乱码。",
    },
    "isodate": {
        "usage": "isodate 显示当前 ISO8601 时间与 Unix 时间戳;isodate <秒数> 把时间戳翻译成人话。写日志/对表很方便。",
    },
    "genpass": {
        "usage": "genpass 32 生成 32 位随机密码(大小写+数字);genpass 32 -n 纯字母数字版(避免特殊字符转义问题),生成即复制到剪贴板。",
    },
    "qrcode": {
        "usage": "qrcode 'https://…' 直接在终端里打出二维码(ASCII 图形),手机扫码即开。依赖 qrencode(brew install qrencode)。",
    },
    "web-search": {
        "usage": "google rust book、ddg zsh array、github omz、wiki 北京…回车即打开浏览器进入对应站点搜索页,省去开浏览器敲网址。",
    },
    "frontend-search": {
        "usage": "面向前端的全站搜索别名:npms redux 搜 npm、mdn fetch 搜 MDN、so css grid 搜 StackOverflow 等,浏览器直达结果页。",
    },
    "timer": {
        "usage": "启用后每条命令跑完,提示符旁自动显示耗时(超阈值才显示);TIMER_THRESHOLD 控阈值秒数,TIMER_FORMAT 控样式。跑长构建时一眼看到卡在哪。",
    },
    "bgnotify": {
        "usage": "挂着编译/训练切去别的窗口?命令结束且耗时超过阈值时,系统弹桌面通知(可带声音)。BGNOTIFY_THRESHOLD 设秒数。",
    },
    "ssh-agent": {
        "usage": "开机后第一个 zsh 会自动拉起 ssh-agent 并加载你的密钥,之后所有终端窗口共享,不用每次输密钥口令。配置风格:zstyle ':omz:plugins:ssh-agent' identities id_ed25519。",
    },
    "keychain": {
        "usage": "需先安装 keychain。比 ssh-agent 更省心:跨会话复用已解密密钥,重启后也只需输一次口令;支持 gpg-agent 一并托管。",
    },
    "dotenv": {
        "usage": "项目目录里放 .env 文件,cd 进来自动加载其中环境变量(export 形式),离开自动卸载;敏感项目可用 source_if_not_found? 关闭提示。",
    },
    "last-working-dir": {
        "usage": "零配置:重开终端时自动回到上次关闭时的目录,不用每次 cd 半天。",
    },
    "vi-mode": {
        "usage": "命令行变成 vi:默认插入态照常打字;按 ESC 进普通态——h/l 移动、w/b 跳词、dd 删整行、cw 改词、/ 搜历史,和 vim 手感一致。",
    },
    "zsh-navigation-tools": {
        "usage": "Ctrl+R 打开全屏历史浏览器(可多选);另有 n-cd、n-aliases、n-env、n-kill(选进程杀)等面板工具,F1~F4 快捷唤起,方向键+回车操作。",
    },
    "themes": {
        "usage": "theme agnoster 立即切换主题看效果(可配合 themes 命令补全);theme -l 列出全部主题;满意后写进 ~/.zshrc 的 ZSH_THEME= 持久化。",
    },
    "kubectl": {
        "usage": "k 等于 kubectl;k get 别名 kg 系列;kgp=查 pods、kgs=查 services、kdp=描述 pod、kdel=删除;上下文/命名空间切换建议搭配 kubectx 插件。",
        "aliases": {
            "k": "kubectl 本体",
            "kgp": "列出 Pod",
            "kgs": "列出 Service",
            "kgd": "列出 Deployment",
            "kdp": "描述某个 Pod 详情",
            "kdel": "删除资源",
            "kl": "查看 Pod 日志",
        },
    },
    "docker": {
        "usage": "d = docker;dps 一屏看容器(名称/镜像/端口);dcu=compose up、dcd=down(需配合 docker-compose 插件);dip 容器名 查它的 IP。",
        "aliases": {
            "d": "docker 本体",
            "dps": "列出运行中容器",
            "dpsa": "列出全部容器(含已停止)",
            "di": "docker images",
            "dex": "进入容器内 shell(exec)",
            "dip": "查看容器 IP",
            "drmf": "删除全部已停止容器",
        },
    },
    "brew": {
        "usage": "bi 包名 = 安装;bu = 升级全部;bo = 只升级过期的;brm = 卸载;bl 列已装;bs 搜索;binfo 包名 看详情。全部别名只是 brew 子命令的缩写。",
        "aliases": {
            "bi": "brew install 安装",
            "bubu": "brew update && upgrade 全量升级",
            "brm": "brew uninstall 卸载",
            "bl": "brew list 已安装列表",
            "bs": "brew search 搜索",
        },
    },
    "pip": {
        "usage": "pipi 包 = 安装;pipu = 升级;pipun = 卸载;pipreq 把当前环境依赖导出成 requirements.txt(配合虚拟环境最干净)。",
    },
    "python": {
        "usage": "py = python3;ipy = ipython;pyserver 在当前目录起一个 http.server,局域网共享文件秒变「本地网盘」;pyclean 清理 __pycache__ 与 .pyc。",
    },
    "vscode": {
        "usage": "code . 用 VS Code 打开当前目录(需先在 VS Code 里装 Shell Command);另有直接打开 settings.json、keybindings.json 等配置文件的快捷命令。",
    },
    "macos": {
        "usage": "常用别名:showfiles / hidefiles 开关 Finder 隐藏文件;preview 文件 用「预览」打开;cdf 跳到 Finder 当前正打开的目录;music 系列控制音乐播放。",
        "aliases": {
            "showfiles": "Finder 显示隐藏文件",
            "hidefiles": "Finder 隐藏隐藏文件",
            "preview": "用「预览」打开文件",
            "cdf": "cd 到 Finder 当前目录",
            "spot": "用 Spotlight 搜索文件",
        },
    },
    "aws": {
        "usage": "提供 aws 补全外,还有两个实用函数:aws_change_access_key 更新访问密钥;aws_profiles 列出 ~/.aws/config 里的全部 profile,ap <名> 切换 AWS_PROFILE。",
    },
    "ssh": {
        "usage": "启用后 ssh <Tab> 自动补全主机名,来源是 ~/.ssh/config 的 Host 条目与 known_hosts。配合 config 文件给服务器起别名,不用再记 IP。",
    },
    # ────── 文件与压缩(#17)──────
    "cp": {
        "usage": "提供 cpv 函数:cpv 源文件 目标,内部走 rsync 并自动带上安全参数 —— 保留权限/属主/属组,目标已存在时先做备份而不是直接覆盖。日常覆盖式拷贝建议用它替代裸 cp。",
        "aliases": {"cpv": "带安全参数的 rsync 拷贝(保留权限/属主/属组,覆盖前备份)"},
    },
    "torrent": {
        "usage": "把磁力链接转成本地 .torrent 种子文件:magnet_to_torrent 'magnet:?xt=urn:btih:…'(参数加引号防 & 截断)。依赖 python3 与本机可用的 BT 工具链。",
        "aliases": {"magnet_to_torrent": "磁力链接 → .torrent 种子文件"},
    },
    # ────── 文档与笔记(#18)──────
    "dnote": {
        "usage": "为命令行笔记工具 Dnote 提供全量补全:输入 dnote 后按 Tab 可补全子命令、书名与笔记名(如 dnote a<Tab> 补全 add)。需先安装 dnote CLI。",
    },
    "geeknote": {
        "usage": "为 Evernote 命令行客户端 Geeknote 提供补全,并定义 gn 别名替代冗长的 geeknote 命令(gn login、gn create --title …)。",
        "aliases": {"gn": "geeknote 本体"},
    },
    # ────── 测试与质量(#16)──────
    "codeclimate": {
        "usage": "为代码质量平台 Code Climate 的 CLI 提供子命令补全(codeclimate engines list 等)。需安装 codeclimate CLI。",
    },
    "kitchen": {
        "usage": "为基础设施测试框架 Test Kitchen 提供补全:kitchen<Tab> 可补全 list/create/converge/verify/destroy/login 等子命令与套件名。需安装 kitchen。",
    },
    "molecule": {
        "usage": "为 Ansible 角色测试框架 Molecule 提供别名与补全。典型循环:mol 建实例 → mcon 跑配置 → mvf 跑测试,mls 随时看实例状态。",
        "aliases": {
            "mol": "molecule 本体",
            "mcr": "create:用 provisioner 启动测试实例",
            "mcon": "converge:对实例执行配置",
            "mls": "list:查看实例状态",
            "mvf": "verify:对实例运行自动化测试",
        },
    },
    "qodana": {
        "usage": "为 JetBrains Qodana 代码质量 CLI 提供补全;补全脚本带缓存并在插件加载时自动更新,终端启动无明显开销。需安装 qodana CLI。",
    },
    "pre-commit": {
        "usage": "pre-commit 钩子框架的常用别名。日常:git add 后跑 prcra 做全文件检查;装了 prek(Rust 版 pre-commit)时会自动用它替代,速度更快。",
        "aliases": {
            "prc": "pre-commit 本体(有 prek 时自动切 prek)",
            "prcau": "autoupdate:更新钩子版本",
            "prcr": "run:运行钩子",
            "prcra": "run --all-files:全仓库检查",
            "prcrf": "run --files <文件>:只检查指定文件",
        },
    },
    # ────── 数据库与数据(#15)──────
    "dbt": {
        "usage": "数据分析工具 dbt 的别名集,围绕「只处理改动过的模型」:dbtrtm 一条命令跑完 改动模型+测试;dbtfrt 全量重建;dbtcds 生成并本地预览文档站点。",
        "aliases": {
            "dbtlm": "列出改动过的模型",
            "dbtrm": "只运行改动过的模型",
            "dbttm": "只测试改动过的模型",
            "dbtrtm": "运行并测试改动过的模型",
            "dbtrs": "clean+deps+seed 重新灌种子数据",
            "dbtfrt": "全量刷新运行并测试",
            "dbtcds": "生成并启动文档站点",
        },
    },
    "mongo-atlas": {
        "usage": "为 MongoDB Atlas 官方 CLI(atlas 命令)提供子命令与参数补全:atlas <Tab>。需安装 atlas CLI。",
    },
    "mongocli": {
        "usage": "MongoDB 运维 CLI 的快捷前缀:ma 进 Atlas、mcm 进 Cloud Manager、mom 进 Ops Manager、miam 进 IAM 子命令,后接 Tab 补全。",
        "aliases": {
            "ma": "mongocli atlas",
            "mcm": "mongocli cloud-manager",
            "mom": "mongocli ops-manager",
            "miam": "mongocli iam",
        },
    },
    "mysql-macports": {
        "usage": "MacPorts 安装的 MySQL 服务管理四件套:mysqlstart / mysqlstop / mysqlrestart / mysqlstatus(status 会提示输 root 密码)。仅适合 MacPorts 安装方式。",
        "aliases": {
            "mysqlstart": "启动 MySQL 服务",
            "mysqlstop": "停止 MySQL 服务",
            "mysqlrestart": "重启 MySQL 服务",
            "mysqlstatus": "ping 检查服务是否存活",
        },
    },
    "postgres": {
        "usage": "Homebrew 版 PostgreSQL 的服务管理别名:startpost/stoppost/restartpost/reloadpost/statuspost。注意数据目录硬编码为 /usr/local/var/postgres(仅适合 Intel Mac 的 Homebrew 路径,Apple Silicon 为 /opt/homebrew,不匹配时勿用)。",
        "aliases": {
            "startpost": "启动 postgres 并写日志",
            "stoppost": "快速停止 postgres",
            "restartpost": "重启 postgres",
            "reloadpost": "重载配置(部分配置需重启才生效)",
            "statuspost": "查看运行状态",
        },
    },
    "redis-cli": {
        "usage": "为 redis-cli 提供命令与参数补全(基于 Homebrew 补全改造)。需本机安装 redis。",
    },
    # ────── 趣味与语录(#14)──────
    "catimg": {
        "usage": "catimg 图片.png 直接在终端里把图片渲染成彩色字符画(可加第二参数调分辨率)。依赖 ImageMagick 的 magick/convert。",
        "aliases": {"catimg": "终端显示图片"},
    },
    "chucknorris": {
        "usage": "chuck 随机来一条查克·诺里斯式程序员笑话;chuck_cow 让笑话装进 cowthink 的奶牛气泡里。依赖系统 fortune/strfile(缺 strfile 时首次会报提示),适合当 MOTD。",
        "aliases": {
            "chuck": "随机查克·诺里斯笑话",
            "chuck_cow": "笑话装进奶牛气泡",
        },
    },
    "hitchhiker": {
        "usage": "hitchhiker 随机输出《银河系漫游指南》语录;hitchhiker_cow 同样配奶牛气泡。依赖 fortune。",
        "aliases": {
            "hitchhiker": "随机语录",
            "hitchhiker_cow": "语录 + cowthink 气泡",
        },
    },
    "hitokoto": {
        "usage": "hitokoto 从 hitokoto.cn 随机拉一句中文「一言」(需联网)。可以塞进 .zshrc 末尾,让每次开终端都有新句子。",
        "aliases": {"hitokoto": "随机一言(联网)"},
    },
    "lol": {
        "usage": "猫语别名包,纯属好玩:wtf 看内核日志、nomnom 杀进程、icanhas 建目录、rtfm 打开手册、moar 翻页、visible/invisible 是 echo/cat。装上后同事看你屏幕会困惑。",
        "aliases": {
            "wtf": "dmesg 内核日志",
            "nomnom": "killall 杀进程",
            "icanhas": "mkdir",
            "rtfm": "man",
            "moar": "more",
            "tldr": "less",
        },
    },
    "octozen": {
        "usage": "display_octozen 从 GitHub 拉一条 Octocat 禅语显示在终端(需联网,2 秒超时不阻塞)。可加进 .zshrc 当开机禅语。",
        "aliases": {"display_octozen": "显示 GitHub 禅语(联网)"},
    },
    "rand-quote": {
        "usage": "quote 从 quotationspage.com 随机拉一条英文名人名言(需联网),适合随手放在提示信息里。",
        "aliases": {"quote": "随机英文名言(联网)"},
    },
    # ────── 安全与密码(#13)──────
    "gpg-agent": {
        "usage": "零配置启用。修复 GPG 常见翻车点:每次命令执行前刷新 GPG_TTY(否则 pinentry 弹不出密码框);若 gpg-agent 开了 enable-ssh-support,自动把 SSH_AUTH_SOCK 指到 gpg 的 ssh 套接字(让 ssh 用 GPG 密钥)。做 git 签名提交必配。",
    },
    "otp": {
        "usage": "把双因素验证器搬进终端(依赖 oathtool 与 GPG):otp_add_device 登记一个新 MFA 密钥(密钥用你的 GPG 公钥加密后存到 ~/.otp);之后 ot <名字> 生成一次性验证码并自动进剪贴板,粘贴即用。",
        "aliases": {
            "otp_add_device": "登记新的 GPG 加密 MFA 密钥",
            "ot": "生成验证码并复制到剪贴板",
        },
    },
    "pass": {
        "usage": "为标准 Unix 密码管理器 pass 提供完整补全(含密码条目名 Tab 补全)。多密码库场景可按 README 配 compdef + zstyle prefix + 包装函数,让 workpass 等私有库各自补全。",
    },
    "pass-cli": {
        "usage": "为 Proton Pass 官方 CLI 提供补全(pass <Tab>)。需安装 Proton Pass CLI。",
    },
    "rbw": {
        "usage": "为 Bitwarden 第三方客户端 rbw 提供补全;亮点是 rbwpw <服务名>:把对应密码复制进剪贴板并在 20 秒后自动清空,免去看明文的尴尬。需先 rbw login/unlock。",
        "aliases": {"rbwpw": "取密码进剪贴板,20 秒后自动清空"},
    },
    "sigstore": {
        "usage": "为软件供应链签名三件套提供补全:cosign(镜像签名/验签)、sget(带验签的拉取)、rekor(透明日志查询)。",
    },
    "lpass": {
        "usage": "为 LastPass 密码管理器 CLI 提供补全:lpass <Tab> 补全 show/generate/login 等子命令与条目名。需安装 lastpass-cli 并已登录。",
    },
    "vault": {
        "usage": "为 HashiCorp Vault 密钥管理 CLI 提供子命令与路径补全(vault kv get secret/foo <Tab>)。需安装 vault。",
    },

    "colored-man-pages": {},
    "command-not-found": {},
}

# ---------------- 事实修正 ----------------
FIXES = {
    "copybuffer": {
        "summary": "Ctrl+O 复制当前输入行",
        "detail": "按 Ctrl+O 把当前正在输入、还没回车的命令整行复制到系统剪贴板,方便拿到别的窗口先跑。",
    },
    "gitignore": {
        "summary": "gitignore.io 模板生成 .gitignore",
        "detail": "提供 gi 命令,按模板组合生成 .gitignore:gi macos,python >> .gitignore 即可得两套模板合集,模板来自 gitignore.io 在线服务。",
    },
    "emacs": {
        "summary": "Emacs 客户端别名",
        "detail": "为 emacsclient 提供一系列别名(当前窗口打开、新窗口打开、终端态打开等),配合 Emacs 守护进程秒开文件。",
    },
    "node": {
        "summary": "Node 工具链补全(懒加载)",
        "detail": "为 node/npm/npx/yarn 等命令提供补全,并采用懒加载:首次按 Tab 才真正生成补全,显著加快 shell 启动。",
    },
    "ember-cli": {
        "summary": "Ember CLI 别名与补全",
        "detail": "为 Ember.js 的 ember 命令提供补全和 serve/build/test 等常用子命令的缩写别名。",
    },
    "terraform": {
        "summary": "Terraform 别名与补全",
        "detail": "为基础设施即代码工具 Terraform 提供补全和一系列 tf 系列缩写别名(init/plan/apply 等常用子命令)。",
    },
}

def main():
    data = json.loads(DATA.read_text(encoding="utf-8"))
    if len(data) < 300:
        sys.exit(f"词典条目异常: {len(data)}")

    # 1) 事实修正
    for name, patch in FIXES.items():
        if name in data:
            data[name].update(patch)

    # 2) 分类
    cats = {}
    for name, entry in data.items():
        c = classify(name, entry)
        entry["cat"] = c
        cats.setdefault(c, 0)
        cats[c] += 1

    # 3) 实战用法/别名注解(usage 非空的才写入)
    used = 0
    for name, extra in USAGE.items():
        if not extra:
            continue
        if name not in data:
            sys.exit(f"USAGE 里的插件不存在: {name}")
        if "usage" in extra:
            data[name]["usage"] = extra["usage"]
        if "aliases" in extra:
            data[name]["aliases"] = extra["aliases"]
        used += 1

    DATA.write_text(
        json.dumps(data, ensure_ascii=False, indent=1, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    print(f"分类完成: {len(data)} 条, {len(cats)} 个类别")
    for c, n in sorted(cats.items(), key=lambda x: -x[1]):
        print(f"  {c}: {n}")
    print(f"usage/aliases 增强: {used} 个插件")

if __name__ == "__main__":
    main()
