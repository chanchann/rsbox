// node test

const { echo, log_hello, log_param, log_macro } = require('./pkg/demo1.js')

console.log(echo());

log_hello();

log_param("111");

log_macro("workld");