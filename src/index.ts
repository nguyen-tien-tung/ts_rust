console.log([1, 2, 3].map((i) => i + 1));

import * as fs from 'fs';
import * as path from 'node:path';

fs.readFile(path.resolve('projects/lines'), 'utf-8', (err, data) => {
  if (err) throw err;
  data.split('\n').forEach((line) => console.log(line));
});
