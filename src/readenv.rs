// read line by line a kernel.build file

use std::{fs, process::Command};
use std::collections::HashMap;
use std::str::Lines;

const default_env_path: &str = "kernel.build";
const vars_of_importance: [&str] = ["OUT_DIR", "ASM_FILES", "LINK_SCRIPT"];

// dont worry about testing and correctness, as long as it works in the good case where you have a good kernel.build file
fn read_env() -> HashMap {
    // read from "kernel.build"
    let build_env = fs::read_to_string(default_env_path).expect("Could not read file. Does it exist, or perhaps it is not readable?")
    // scan line by line
    let lines = build_env.lines();

    let var_map = HashMap::from([("OUT_DIR", [""]), ("ASM_FILES", [""]), ("LINK_SCRIPT", [""])]);

    for l in lines {
        // collect any of the list, always take the last one
        
        // take the stuff before '='
        // use the trait StringTools for these
        let var = l.prefix_before("=");
        let val = l.suffix_after("=");

        if vars_of_importance.values().any(|v| v == &var) {
            // get the replace that var of importance value
            var_map[var] = val;
        }

    }

    // return the map
    var_map
}

trait StringTools {
    fn prefix_before(_char: u8) -> String;
    fn suffix_after(_char: u8) -> String;
}

impl StringTools for String {
    fn prefix_before(&self, _char: u8) -> String {
        // get rid of whitespaces trailing and before
        self.trim();

        // find the first '='
        let _index = self.find("=");
        
        // take 0-index
        String::from(&self[.._index])
    }
    fn suffix_after(&self, _char: u8) -> String {
        self.trim();
        let _index = self.find("=");
        String::from(&self[_index+1..])
    }
}
