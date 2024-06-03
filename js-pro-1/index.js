const _ = require('lodash');
// import _ from 'lodash';

const result = _.filter([1, 2, 3, 4, 5], n => n % 2 === 0);
console.log(result); // 输出：[2, 4]


