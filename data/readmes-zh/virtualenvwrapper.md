# Virtualenvwrapper 插件

本插件加载 Python 的 [virtualenvwrapper](https://virtualenvwrapper.readthedocs.io/en/latest/) shell 工具。

✅ 启用方式:把「virtualenvwrapper」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 用法

本插件支持在 cd 进入名称匹配的 git 仓库时自动激活对应的 virtualenv:

```
➜  github $ cd ansible
(ansible) ➜  ansible git:(devel) $ cd docs
(ansible) ➜  docs git:(devel) $ cd ..
(ansible) ➜  ansible git:(devel) $ cd ..
➜  github $
```

我们可以在目录里放一个内容为其他 virtualenv 名称的 `.venv` 文件,来覆盖这一行为:

```
➜  github $ cat ansible/.venv
myvirtualenv
➜  github $ cd ansible
(myvirtualenv) ➜  ansible git:(devel) $ cd ..
➜  github $
```

我们可以通过在 source Oh My Zsh 之前设置 `DISABLE_VENV_CD=1` 来禁用该行为:

```zsh
DISABLE_VENV_CD=1
plugins=(... virtualenvwrapper)
source $ZSH/oh-my-zsh.sh
```
