/* @flow */

import * as path from 'path';

type Transformer = (path: string, data: string) => string;

export default function load(id: string): Transformer {
  let absolutePath;

  if (path.isAbsolute(id)) {
    absolutePath = id;
  } else {
    absolutePath = path.join(process.cwd(), id);
  }

  // $FlowIgnore
  return require(absolutePath);
}
