### 项目说明

* [项目演示地址](http://c0de.me)
* 我想试试，如果没有框架，我还会写代码吗？学习下如何利用最基础的一些东西来开发一个网站。
* 边学习 rust 同时学习网站开发相关的一些知识，如 简单的实现 http 协议，socket 编程 等等
* rust 是一门很难的语言，如果只是在别人框架基础上做一些简单的事情，可能很难学好，所以有了这个项目，纯属兴趣。

#### v0.1.0 构思

1. 暂时不用数据库，所有文章 md 文件保存在服务端，每个文件夹一个栏目。后期版本改成数据库直接存数据库即可。
1. 基于 socket 实现简单的 GET 和 POST。
1. 不同的网页对应不同的页面模板，在页面模板中嵌入 md 文件内容。
1. 每个文章有个对应的 json 文件，记录文章的标题，描述，关键词，作者，创建时间，修改时间，标签信息，url 中的别名。
1. 每个目录下有个对应的 json 文件，保存当前分类的一些信息，如：当前分类 url 中的别名，使用的模板，分类名称，分类标签，分类文章数量。
1. 上述 json 文件在每次启动服务器的时候动态读取到程序内存中，有修改则更新到 json 中。
1. 把 md 文件编译成 html 文件，缓存起来，客户端请求时，先读取缓存，找不到缓存文件再编译。缓存目录中保存一个 json 文件，用于存储 md 文件和 html 文件对应关系，用户请求的时候则会从该文件获取缓存信息，同样程序启动就读取该文件信息并时刻保持同步。
1. 指定一个 config.json 文件，用于指定网站监听端口，附件保存目录，有哪些栏目。
1. 实现客户端发布文章。

