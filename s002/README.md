

`--release`つけないと、singleで323秒、、cargo runだと遅いんだね、、
```bash
$ cargo run --release
--- 2.2.15 ---
start worker
fin worker
100
  チャンネル
(10, 20)
start!
single_threaded: 69.322715500秒
end
multi_threaded: 35.26998292秒
---
```
