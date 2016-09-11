CLI中英文互译工具
==============

第一个版本，还比较残，之后有空会支持文本翻译。

用法比较简单&粗暴:
```
$ traitor PHP是世界上最好的语言
PHP is the best language in the world.
$ traitor "God is girl"
上帝是女孩
```

由于调用了度娘的API，所以需要跟百度申请AK，然后配置~/.traitor，大概长这样:
```
[traitor]                                                                                                  
APP_ID = "20160911000028543"                                                                               
SECRET = "SpfYQRoC_CL2ibESzbu3"
```