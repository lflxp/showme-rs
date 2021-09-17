# RUN

> cargo run -- monitor -l -c -N -d -s -i 1

```
warning: `clapdemo` (bin "clapdemo") generated 22 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/clapdemo monitor -l -c -n -d -s -i 1`
Value for config: default.conf
No verbose info
-------- -----load-avg---- ----cpu-usage--- ---swap--- ----net(A)---- ------------------------io-usage---------------------- 
  time  |  1m    5m   15m | usr sys idl iow|   si   so|   recv   send| readc writec    srkB    swkB queue  await svctm %util|
18:07:10|loadcpuswapnetdisk
18:07:11|loadcpuswapnetdisk
18:07:12|loadcpuswapnetdisk
18:07:13|loadcpuswapnetdisk
```