/* @flow */

import createCommand from '../utils/create-command';
import load from '../utils/loader';

// $FlowIgnore
const cowboy = require('../native');

export default createCommand((script: string, pattern: string) => (
  new Promise((resolve) => {
    const transformer = load(script);
    const total = cowboy.refactor(pattern, transformer);

    resolve(`${total} files refactored!`);
  })
));
