// read line by line a kernel.build file

use std::{fs, process::Command};
use std::collections::HashMap;
use std::str::Lines;

const default_env_path: &str = "kernel.build";
const vars_of_importance: [&str; 3] = ["OUT_DIR", "ASM_FILES", "LINK_SCRIPT"];

macro_rules! str {
    () => {
        String::new()
    };
    ($x:expr $(,)?) => {
        ToString::to_string(&$x)
    };
}

// dont worry about testing and correctness, as long as it works in the good case where you have a good kernel.build file
fn read_env() -> HashMap<String, String> {
    // read from "kernel.build"
    let build_env = fs::read_to_string(default_env_path).expect("Could not read file. Does it exist, or perhaps it is not readable?");
    // scan line by line
    let lines = build_env.lines();

    let var_map = HashMap::from([(str!("OUT_DIR"), str!("")), (str!("ASM_FILES"), str!("")), (str!("LINK_SCRIPT"), str!(""))]);

    for l in lines {
        // collect any of the list, always take the last one
        let l_str = str!(l);
        
        // take the stuff before '='
        // use the trait StringTools for these
        let var = l_str.prefix_before("=");
        let val = l_str.suffix_after("=");

        if vars_of_importance.iter().any(|v| v == &var) {
            // get the replace that var of importance value
            var_map[var.as_str()] = val;
        }

    }

    // return the map
    var_map
}

trait StringTools {
    fn prefix_before(&self, _char: &str) -> String;
    fn suffix_after(&self, _char: &str) -> String;
}

impl StringTools for String {
    fn prefix_before(&self, _char: &str) -> String {
        // get rid of whitespaces trailing and before
        self.trim();

        // find the first '='
        let _index: u32 = self.find("=").end;

        // let _index = match _index {
        //     Some(ind) => 
        // }
        
        // take 0-index
        String::from(&self[.._index])
    }
    fn suffix_after(&self, _char: &str) -> String {
        self.trim();
        let _index = self.find("=");
        String::from(&self[_index+1..])
    }
}
