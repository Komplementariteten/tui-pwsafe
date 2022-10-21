#![allow(incomplete_features)]
#![feature(specialization)]
extern crate core;

use crate::model::SafeModel;
use crate::ui::view_models::run;
use std::env;

mod contracts;
mod model;
mod ui;

fn main() /* -> Result<(), io::Error> */
{
    let args = env::args();
    if let Some(file) = args.last() {
        let model = SafeModel::new(file.as_str());
        if let Err(e) = run(model) {
            panic!("{:?}", e);
        }
    }
}
