mkdir my-ts-project
cd my-ts-project

yarn init -y

yarn add typescript @types/node --dev

npx tsc --init

touch index.ts

const message: string = "Hello, TypeScript!";
console.log(message);

yarn add lodash
yarn add @types/lodash --dev

import _ from 'lodash';

const numbers = [1, 2, 3, 4, 5];
const evenNumbers = _.filter(numbers, number => number % 2 === 0);
console.log(evenNumbers); // 输出：[2, 4]

"scripts": {
"build": "tsc"
}
yarn build

node dist/index.js

"scripts": {
"start": "node dist/index.js"
}

yarn start

