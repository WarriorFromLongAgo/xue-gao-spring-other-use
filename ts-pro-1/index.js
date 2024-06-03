"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const message = "Hello, TypeScript!";
console.log(message);
const lodash_1 = __importDefault(require("lodash"));
const numbers = [1, 2, 3, 4, 5];
const evenNumbers = lodash_1.default.filter(numbers, number => number % 2 === 0);
console.log(evenNumbers); // 输出：[2, 4]
