#[macro_use]
extern crate neon;
extern crate glob;

mod errors;

use std::io::{Read, Write};
use std::fs::File;
use std::path::{PathBuf};

use glob::glob;
use neon::vm::{Call, JsResult, Throw};
use neon::js::{JsFunction, JsInteger, JsString, JsUndefined};

use errors::RefactorError;

fn read_file(path_buf: &PathBuf) -> Result<(&PathBuf, String), RefactorError> {
    let mut file = try!(File::open(path_buf.as_path()));
    let mut data = String::from("");

    try!(file.read_to_string(&mut data));
    drop(file);

    Ok((path_buf, data))
}

fn refactor(call: Call) -> JsResult<JsInteger> {
    let scope = call.scope;
    let pattern = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
    let transformer = try!(try!(call.arguments.require(scope, 1)).check::<JsFunction>());

    let total = glob(&pattern)
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<PathBuf>>()
        .iter()
        .filter(|path_buf| path_buf.is_file())
        .map(read_file)
        .filter_map(Result::ok)
        .map(|(path_buf, data)| -> Result<(), Throw> {
            let path = JsString::new(scope, path_buf.to_str().unwrap()).unwrap();
            let old_data = JsString::new(scope, &data).unwrap();
            let new_data = try!(try!(transformer
                .call(scope, JsUndefined::new(), vec![path, old_data]))
                .check::<JsString>())
                .value();

            let mut file = File::create(path_buf).unwrap();

            file.write_all(&new_data.into_bytes()).unwrap();

            Ok(())
        })
        .count() as i32;

    Ok(JsInteger::new(scope, total))
}

register_module!(m, {
    m.export("refactor", refactor)
});
