import path from 'path';

import json from 'rollup-plugin-json';
import babel from 'rollup-plugin-babel';
import resolve from 'rollup-plugin-node-resolve';

const pkg = require('./package.json');

export default {
  dest: path.join(__dirname, 'dist', 'index.js'),
  entry: path.join(__dirname, 'src', 'index.js'),
  format: 'cjs',
  plugins: [
    resolve(),
    babel(),
    json(),
  ],
  external: Object.keys(pkg.dependencies).concat([
    'path',
  ]),
  preferConst: true,
};
