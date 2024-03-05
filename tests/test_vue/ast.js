const { parse } = require('@vue/compiler-sfc');
const fs = require('fs');
const path = require('path');

// 指定要测试的Vue文件路径
const vueFilePath = path.resolve(__dirname, 'test.vue');
console.log(vueFilePath);
const vueFileContent = fs.readFileSync(vueFilePath, 'utf-8');

// 开始计时
console.time('parseTime');

// 解析Vue文件
const { descriptor } = parse(vueFileContent);

// 停止计时并输出结果
console.timeEnd('parseTime');

// 如果你想查看AST，可以选择输出descriptor的内容
// console.log(descriptor);
