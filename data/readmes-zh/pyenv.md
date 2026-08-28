# pyenv

本插件会查找 [pyenv](https://github.com/pyenv/pyenv)(一个简单的 Python 版本管理系统),找到后就加载它。如果找到 pyenv-virtualenv(一个用于管理 virtualenv 的 pyenv 插件),也会一并加载。如果找到了 venv,则 pyenv 不会加载。

✅ 启用方式:把「pyenv」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

如果你在启动时收到 `Found pyenv, but it is badly configured.` 错误,可能需要确保 `pyenv` 在 oh-my-zsh 的 pyenv 插件加载之前完成初始化。做法是:在 `.zshrc` 文件中,把下面几行放在 `plugins=(...)` 行更靠前的位置:

```zsh
export PYENV_ROOT="$HOME/.pyenv"
export PATH="$PYENV_ROOT/bin:$PATH"
eval "$(pyenv init --path)"
```

## 设置

- `ZSH_PYENV_QUIET`:如果设为 `true`,当插件发现 `pyenv` 配置不正确时,将不打印任何消息。

- `ZSH_PYENV_VIRTUALENV`:如果设为 `false`,插件在找到 pyenv-virtualenv 时将不加载它。

- `ZSH_THEME_PYENV_NO_SYSTEM`:如果设为 `true`,当插件找到系统或默认 Python 版本时,将不显示它。
- `ZSH_THEME_PYENV_PREFIX`:在提示符中显示在 Python 版本之前的前缀。

- `ZSH_THEME_PYENV_SUFFIX`:在提示符中显示在 Python 版本之后的后缀。

## 函数

- `pyenv_prompt_info`:显示 pyenv 正在使用的 Python 版本;如果没有找到 pyenv,则显示全局 Python 版本。
