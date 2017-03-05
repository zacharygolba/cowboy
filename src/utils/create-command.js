/* @flow */

type Handler = (...args: Array<any>) => Promise<any>;
type Command = (...args: Array<any>) => void;

export default function createCommand(handler: Handler): Command {
  return (...args) => {
    handler(...args)
      .then(data => typeof data !== 'undefined' && console.log(data))
      .catch((err) => {
        console.error(err);
        process.exit(1);
      });
  };
}
