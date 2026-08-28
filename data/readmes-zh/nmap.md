# Nmap 插件

为 [Nmap](https://nmap.org/) 添加了一些实用别名,类似 zenmap 中的扫描配置档(profile)。

✅ 启用方式:把「nmap」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。

## 别名

- `nmap_open_ports`:扫描目标上开放的端口。
- `nmap_list_interfaces`:列出命令所在主机的所有网络接口。
- `nmap_slow`:慢速扫描,避免刷爆目标的日志。
- `nmap_fin`:使用 TCP FIN 扫描来探测主机是否在线。
- `nmap_full`:激进的全量扫描,扫描所有端口,并尝试判断操作系统与服务版本。
- `nmap_check_for_firewall`:TCP ACK 扫描,用于检查防火墙是否存在。
- `nmap_ping_through_firewall`:使用 SYN 和 ACK 探测(而不是仅用 ping)进行主机发现,以避开防火墙限制。
- `nmap_fast`:对最常用的 300 个端口进行快速扫描。
- `nmap_detect_versions`:在所有端口上检测服务与操作系统的版本。
- `nmap_check_for_vulns`:使用 vulscan 脚本检查目标服务是否存在漏洞。
- `nmap_full_udp`:与 full 相同,但通过 UDP 进行。
- `nmap_traceroute`:尝试使用最常用的端口进行路由跟踪(traceroute)。
- `nmap_full_with_scripts`:与 nmap_full 相同,但还会运行所有脚本。
- `nmap_web_safe_osscan`:相对「安全」一些的操作系统版本扫描,因为只连接 HTTP 和 HTTPS 端口不会显得太具攻击性。
- `nmap_ping_scan`:ICMP 扫描,寻找活动主机。
