# Laravel

本插件为 Laravel 的 [Artisan](https://laravel.com/docs/artisan) 和 [Bob](http://daylerees.github.io/laravel-bob/) 命令行接口添加别名和自动补全。

```
plugins=(... laravel)
```

| 别名  | 说明                |
|:-:|:-:|
| `artisan`  | `php artisan`  |
| `pas`  | `php artisan serve` |
| `pad`  | `php artisan dev` |
| `pats`  | `php artisan test` |

## 数据库

| 别名  | 说明 |
|:-:|:-:|
| `pam`  |  `php artisan migrate` |
| `pamf`  |  `php artisan migrate:fresh` |
| `pamfs`  |  `php artisan migrate:fresh --seed` |
| `pamr`  |  `php artisan migrate:rollback` |
| `pads`  |  `php artisan db:seed` |
| `padw`  |  `php artisan db:wipe` |

## 生成器

| 别名  | 说明 |
|:-:|:-:|
| `pamm`  |  `php artisan make:model` |
| `pamc`  |  `php artisan make:controller` |
| `pams`  |  `php artisan make:seeder` |
| `pamt`  |  `php artisan make:test` |
| `pamfa`  |  `php artisan make:factory` |
| `pamp`  |  `php artisan make:policy` |
| `pame`  |  `php artisan make:event` |
| `pamj`  |  `php artisan make:job` |
| `paml`  |  `php artisan make:listener` |
| `pamn`  |  `php artisan make:notification` |
| `pamcl` | `php artisan make:class` |
| `pamen` | `php artisan make:enum` |
| `pami`  | `php artisan make:interface` |
| `pamtr` | `php artisan make:trait` |

## 清除

| 别名  | 说明 |
|:-:|:-:|
| `pacac`  |  `php artisan cache:clear` |
| `pacoc`  |  `php artisan config:clear` |
| `pavic`  |  `php artisan view:clear` |
| `paroc`  |  `php artisan route:clear` |
| `paopc`  |  `php artisan optimize:clear` |

## 队列

| 别名  | 说明 |
|:-:|:-:|
| `paqf`  |  `php artisan queue:failed` |
| `paqft`  |  `php artisan queue:failed-table` |
| `paql`  |  `php artisan queue:listen` |
| `paqr`  |  `php artisan queue:retry` |
| `paqt`  |  `php artisan queue:table` |
| `paqw`  |  `php artisan queue:work` |
