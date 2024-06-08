// node test

const { echo, log_hello, log_param, log_macro, 
        new_user, UserModel } = require('./pkg/demo1.js')

console.log(echo());

log_hello();

log_param("111");

log_macro("workld");

const user = new_user(121)
log_param(user.uid.toString())

const user1 = new UserModel();
log_param(user1.uid.toString())

user.uid = 10;
log_param(user.uid.toString())
