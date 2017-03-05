/* @flow */

import createCommand from '../utils/create-command';
import * as pkg from '../../package.json';

export default createCommand(() => Promise.resolve(`v${pkg.version}`));
