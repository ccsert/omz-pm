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
# 语料按分类拆在 tools/usage/<issue号>-<分类>.json(大分类再拆 -a/-b 并行补充),
# 格式: {"插件名": {"usage": "2~5 行实战用法", "aliases": {"别名": "一句话中文解释"}}}
# 合并规则:同一插件出现在多个文件视为错误;usage 非空的条目才会写入词典。
USAGE_DIR = Path(__file__).resolve().parent / "usage"

def load_usage() -> dict:
    merged, origin = {}, {}
    for path in sorted(USAGE_DIR.glob("*.json")):
        for name, extra in json.loads(path.read_text(encoding="utf-8")).items():
            if name in merged:
                sys.exit(f"usage 语料重复: {name} 同时在 {origin[name]} 与 {path.name}")
            merged[name] = extra
            origin[name] = path.name
    return merged

USAGE = load_usage()

def validate_usage(corpus: dict) -> None:
    """写作标准(#21):usage 2~5 行且含具体命令/快捷键;有别名时给 3~8 条注解。"""
    import re
    problems = []
    for name, extra in corpus.items():
        if not extra.get("usage"):
            continue
        u = extra["usage"]
        if not (20 <= len(u) <= 600 and re.search(r"[\u4e00-\u9fff]", u)):
            problems.append(f"{name}: usage 长度/语言异常({len(u)} 字符)")
        if not re.search(r"[A-Za-z]", u):
            problems.append(f"{name}: usage 缺少具体命令/快捷键")
        n = len(extra.get("aliases", {}))
        if any(not v.strip() or not re.search(r"[\u4e00-\u9fff]", v) for v in extra.get("aliases", {}).values()):
            problems.append(f"{name}: 别名注解必须是非空中文")
        if n and not (3 <= n <= 8):
            problems.append(f"{name}: 别名注解 {n} 条(建议 3~8,确有出入请复核)")
    if problems:
        for x in problems:
            print(f"⚠ {x}")

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

def check_sources(corpus: dict) -> None:
    """本地核验:别名/函数名必须能在插件源码(~/.oh-my-zsh)里找到,防幻觉。
    函数定义、别名定义、别名循环展开都算命中;找不到只警告不阻断。"""
    import os
    omz = Path(os.environ.get("OMZ_ZSH", Path.home() / ".oh-my-zsh"))
    if not (omz / "plugins").exists():
        print("(跳过源码核验:未找到 Oh My Zsh 安装)")
        return
    for name, extra in corpus.items():
        names = list(extra.get("aliases", {}))
        if not names:
            continue
        hits = {n: False for n in names}
        # 插件目录内全部文本文件(别名可能定义在被 source 的无后缀文件里),
        # 以及该插件 README 译文(表内容已对源文核验,同样算数)
        candidates = list((omz / "plugins" / name).rglob("*"))
        readme = DATA.parent / "readmes-zh" / f"{name}.md"
        if readme.is_file():
            candidates.append(readme)
        for f in candidates:
            if not f.is_file():
                continue
            try:
                src = f.read_text(encoding="utf-8", errors="replace")
            except OSError:
                continue
            for n in names:
                if re.search(rf"(?<![\w-]){re.escape(n)}(?![\w-])", src):
                    hits[n] = True
        missing = [n for n, ok in hits.items() if not ok]
        if missing:
            print(f"⚠ {name}: 源码中未找到 {', '.join(missing)}(核对拼写或删除)")


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
    validate_usage(USAGE)
    if "--check-sources" in sys.argv:
        check_sources(USAGE)
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
    with_usage = sum(1 for e in data.values() if e.get("usage"))
    print(f"usage/aliases 增强: {used} 个插件;词典覆盖率 {with_usage}/{len(data)}")

if __name__ == "__main__":
    main()
