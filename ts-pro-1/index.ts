const message: string = "Hello, TypeScript!";
console.log(message);

import _ from 'lodash';

const numbers = [1, 2, 3, 4, 5];
const evenNumbers = _.filter(numbers, number => number % 2 === 0);
console.log(evenNumbers); // 输出：[2, 4]