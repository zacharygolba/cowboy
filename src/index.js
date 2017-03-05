/* @flow */

import cli from 'commander';

import refactor from './commands/refactor';
import version from './commands/version';
import help from './commands/help';

cli
  .command('refactor <script> <pattern>')
  .description('Transform files matching a glob pattern')
  .action(refactor);

cli
  .command('version')
  .description('Output version information')
  .action(version);

cli
  .command('help')
  .description('Output usage information')
  .action(help);

cli.parse(process.argv);

// $FlowIgnore
if (!cli.args.length) {
  help();
}
