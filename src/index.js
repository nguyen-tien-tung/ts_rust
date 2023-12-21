"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
console.log([1, 2, 3].map(function (i) { return i + 1; }));
var fs = require("fs");
var path = require("node:path");
fs.readFile(path.resolve('projects/lines'), 'utf-8', function (err, data) {
    if (err)
        throw err;
    data.split('\n').forEach(function (line, i) { return (i % 2 == 0 ? console.log(line) : null); });
});
