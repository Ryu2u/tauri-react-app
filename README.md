
## Tauri-React-App

一个基于Rust Tauri框架开发的一个即时通讯聊天软件

该项目仅作为Rust学习项目

后端服务器为Java SpringBoot + Netty，SpringBoot用来接收Http响应，Netty用于WebSocket连接


#### 已开发的功能
- [x] 前端react页面的编写(一开始使用的前端框架为Angular，后来看着React不错，正好一起学学,把前端的代码使用React重构了下)
- [x] 取消Tauri默认的边框，使用前端自己实现一个窗口栏
- [x] 添加系统托盘
- [x] 改变系统托盘图标
- [x] Tauri 多窗口实现
- [x] 系统托盘闪烁并播放提示音功能
- [x] 后端服务器的搭建
- [x] ProtoBuf Java端和Rust端文件的生成
- [x] 后端数据库设计
- [x] 结合Tauri 和Tokio，将main 改为异步
- [x] 使用`tokio-tungstenite`库作为Rust WebSocket库
- [x] 测试WebSocket是否能够和后端通信
- [x] 测试ProtoBuf 前端和后端的发送和接收，测试序列化和反序列化是否正常
- [x] 使用`reqwest`库作为Http发送端,前端仅作为接收数据和展示数据
- [x] 封装`reqwest`库的http get post 函数，并实现自定义的错误处理
- [x] 登录页面逻辑功能编写
- [x] 后端编写登陆接口
- [x] 登录接口使用Rsa非对称加密进行传输用户名和密码
- [x] Java端编写Rsa 工具类
- [x] 测试Java端Rsa 功能，编写测试
- [x] Rust端使用`rsa`库作为Rsa加密库
- [x] 查询文档，测试Rust rsa秘钥的生成、加密、解密
- [x] 测试登录请求，响应是否正常
- [x] Java端和Rust端身份验证使用jwt令牌的方式(本来的想法是将令牌存在cookie中，但是桌面端的webview无法使用cookie，暂时使用Rust state进行存储令牌，后期考虑适用sql进行存储)
- [x] 实现Rust端，错误的统一处理，若令牌失效则弹出提示框，并关闭所有窗口，返回登录界面


#### 未完成的功能
