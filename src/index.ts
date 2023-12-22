// console.log([1, 2, 3].map((i) => i + 1));

// import * as fs from 'fs';
// import * as path from 'node:path';

// fs.readFile(path.resolve('projects/lines'), 'utf-8', (err, data) => {
//   if (err) throw err;
//   data.split('\n').forEach((line, i) => (i % 2 == 0 ? console.log(line) : null));
// });

type Custom = {
  age: number;
  name: string;
};

type Item = number | string | Custom;

const append = (items: Item[]) => items.push('Hello Fem!');

const myItems: Item[] = [12, 'hehe', { age: 12, name: 'Test' }];

append(myItems);
const numbers: number[] = [1, 2, 3, 4];
append(numbers);
// WTF ?
console.log(numbers);
