本插件能以非常不打扰的方式显示命令的执行时间。

Timer 可以通过以下两个变量来调整:
* `TIMER_PRECISION` 用于控制小数位数(默认 `1`)
* `TIMER_FORMAT` 用于调整显示格式(默认 `'/%d'`)
* `TIMER_THRESHOLD` 用于设置触发计时显示的最短执行时间(默认 `0`)

会话示例:

    me@here:~$ sleep 1                                         /1.0s
    me@here:~$ sleep 73                                     /1m13.0s
    me@here:~$ TIMER_FORMAT='[%d]'; TIMER_PRECISION=2        [0.00s]
    me@here:~$ head -c50 < /dev/urandom | hexdump
    0000000 b2 16 20 f0 29 1f 61 2d 8a 29 20 8c 8c 39 5a ab
    0000010 21 47 0e f9 ee a4 76 46 71 9e 4f 6b a4 c4 51 cb
    0000020 f9 1f 7e b9 6f 2c ae dd cf 40 6d 64 a8 fb d3 db
    0000030 09 37
    0000032                                                  [0.02s]
