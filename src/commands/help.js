/* @flow */

import createCommand from '../utils/create-command';

export default createCommand(() => Promise.resolve(`
  Usage: cowboy <command>

  Commands:
    refactor <script> <pattern>      Transform files that match a glob pattern
    version                          Output version information
    help                             Output usage information
`));
