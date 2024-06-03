mkdir my-js-project
cd my-js-project

yarn init -y

yarn add lodash

touch index.js



const _ = require('lodash');

const result = _.filter([1, 2, 3, 4, 5], n => n % 2 === 0);
console.log(result); // 输出：[2, 4]


node index.js


"scripts": {
"start": "node index.js"
}

yarn start

