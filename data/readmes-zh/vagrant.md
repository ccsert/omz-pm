# Vagrant 插件

本插件为 [Vagrant](https://www.vagrantup.com/) 的命令、任务名、box 名称提供自动补全,并内置了顺手的文档。

✅ 启用方式:把「vagrant」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 别名    | 命令                         |
|---------|------------------------------|
| `vgi`   | `vagrant init`               |
| `vup`   | `vagrant up`                 |
| `vd`    | `vagrant destroy`            |
| `vdf`   | `vagrant destroy -f`         |
| `vssh`  | `vagrant ssh`                |
| `vsshc` | `vagrant ssh-config`         |
| `vrdp`  | `vagrant rdp`                |
| `vh`    | `vagrant halt`               |
| `vssp`  | `vagrant suspend`            |
| `vst`   | `vagrant status`             |
| `vre`   | `vagrant resume`             |
| `vgs`   | `vagrant global-status`      |
| `vpr`   | `vagrant provision`          |
| `vr`    | `vagrant reload`             |
| `vrp`   | `vagrant reload --provision` |
| `vp`    | `vagrant push`               |
| `vsh`   | `vagrant share`              |
| `vba`   | `vagrant box add`            |
| `vbr`   | `vagrant box remove`         |
| `vbl`   | `vagrant box list`           |
| `vbo`   | `vagrant box outdated`       |
| `vbu`   | `vagrant box update`         |
| `vpli`  | `vagrant plugin install`     |
| `vpll`  | `vagrant plugin list`        |
| `vplun` | `vagrant plugin uninstall`   |
| `vplu`  | `vagrant plugin update`      |
