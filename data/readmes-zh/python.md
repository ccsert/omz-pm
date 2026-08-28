# Python 插件

本插件为常用的 [Python](https://www.python.org/) 命令添加了若干别名。

✅ 启用方式:把「python」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

| 命令             | 说明                                                                                   |
| ---------------- | -------------------------------------------------------------------------------------- |
| `py`             | 运行 `python3`。仅当 `py` 未安装时才会设置该别名。                                      |
| `pyfind`         | 在当前目录递归查找 .py 文件                                                             |
| `pyclean [dirs]` | 从给定的目录列表(或当前目录)中删除字节码和缓存文件                                     |
| `pygrep <text>`  | 在当前目录的 `*.py` 文件中递归查找 `text`                                               |
| `pyuserpaths`    | 把用户 site-packages 目录加入 `PYTHONPATH`(针对 Python 2 和 3)                          |
| `pyserver`       | 在当前目录启动 HTTP 服务器(用 `--directory` 可指定其他目录)                             |

## 虚拟环境

本插件提供三个实用工具,用来管理 Python 3.3+ 的 [venv](https://docs.python.org/3/library/venv.html)
虚拟环境:

- `mkv [name]`:在当前目录创建名为 `name` 的新虚拟环境。
  **默认值**:若设置了 `$PYTHON_VENV_NAME` 则用它,否则为 `venv`。

- `vrun [name]`:激活当前目录下名为 `name` 的虚拟环境。
  **默认值**:`$PYTHON_VENV_NAMES` 中第一个存在的。

- `auto_vrun`:进入包含 `<venv-name>/bin/activate` 的目录时自动激活 venv 虚拟环境,
  离开该目录时自动停用(在子目录中保持 venv 激活)。
  - 要启用该特性,请在加载 oh-my-zsh 之前设置 `PYTHON_AUTO_VRUN=true`。
  - 插件会按顺序激活 `$PYTHON_VENV_NAMES` 中第一个存在的虚拟环境。
    默认虚拟环境名为 `venv`。要使用其他名字,请设置
    `PYTHON_VENV_NAME=<venv-name>`。例如:`PYTHON_VENV_NAME=".venv"`

### 设置

你可以在 `.zshrc` 文件中、Oh My Zsh 被加载之前设置这些变量。
例如:

```sh
PYTHON_VENV_NAME=".venv"
PYTHON_VENV_NAMES=($PYTHON_VENV_NAME venv)
...
plugins=(... python)
source "$ZSH/oh-my-zsh.sh"
```


## `$PYTHON_VENV_NAME`

**默认值**:`venv`。

虚拟环境的首选名称,例如通过 `mkv` 创建时使用的名字。

## `$PYTHON_VENV_NAMES`

**默认值**:`$PYTHON_VENV_NAME venv .venv`。

由 `vrun` 和 `auto_vrun` 按顺序检查的虚拟环境名称数组。
也就是说,这两个函数会加载该列表中第一个存在的虚拟环境。
重复的名称会被忽略。
