## 安装mysql && 修改mysql密码

https://blog.csdn.net/weixin_45626288/article/details/133220238

## 创建新用户

```
create user 'chanchan'@'%' identified by 'passwd';
grant all privileges on test.* to 'chanchan'@'%' identified by 'passwd';
flush privileges;

show grants for 'chanchan'@'%';
```

##