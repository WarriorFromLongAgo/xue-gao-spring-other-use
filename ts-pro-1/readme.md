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




# 2
yarn add typescript --dev
npx tsc --init
yarn global add npx

在tsconfig.json文件中添加或更新配置：
{
"compilerOptions": {
"target": "es5",
"module": "commonjs",
"strict": true,
"esModuleInterop": true,
"skipLibCheck": true,
"forceConsistentCasingInFileNames": true
},
"include": ["./**/*.ts"],
"exclude": ["node_modules"]
}

使用Yarn来安装Jest、ts-jest和@types/jest：
yarn add jest ts-jest @types/jest --dev

在项目根目录下创建一个jest.config.js文件，并添加以下内容：
module.exports = {
preset: 'ts-jest',
testEnvironment: 'node',
testMatch: ['**/?(*.)+(spec|test).ts?(x)']
};



编写测试文件：
确保你的测试文件是TypeScript文件，并且使用.ts扩展名，例如avalanchejs_1.test.ts。你的测试代码应该放在这个文件中：
import { base58check } from './base58/base58';

describe('address', () => {
it('create address', () => {
const key = '24jUJ9vZexUM6expyMcT48LBx27k1m7xpraoV62oSQAHdziao5';
const privKey = base58check.decode(key);
console.log("privKey = ", privKey);
});
});

在你的package.json中添加测试脚本：
{
"scripts": {
"test": "jest"
}
}

yarn test







