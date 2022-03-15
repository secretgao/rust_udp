# rust_udp
#### demo 教程
* 开启两个命令行终端 进入cd target/debug 目录
* 一个终端执行
```shell
./rust_udp 127.0.0.1:9520 127.0.0.1:9530 zhangshan 
```
* 另一个终端执行
```shell
./rust_udp 127.0.0.1:9530 127.0.0.1:9520 lisi 
```

* 两个终端就可以进行聊天了
