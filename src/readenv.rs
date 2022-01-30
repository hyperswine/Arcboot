// read line by line a kernel.build file

use std::collections::HashMap;
use std::fs;

const __DEFAULT_ENV_PATH: &str = "kernel.build";
const BUILD_VARS: [&str; 6] = ["OUT_DIR", "ASM_FILES", "LINK_SCRIPT", "OUT_OBJ", "OUT_IMG", "LINK_OBJ"];

#[macro_export]
macro_rules! str {
    () => {
        String::new()
    };
    ($x:expr $(,)?) => {
        ToString::to_string(&$x)
    };
}

// dont worry about testing and correctness, as long as it works in the good case where you have a good kernel.build file
pub fn read_env(env_path: &str) -> HashMap<String, String> {
    // read from "kernel.build"
    let build_env = fs::read_to_string(env_path)
        .expect("Could not read file. Does it exist, or perhaps it is not readable?");
    // scan line by line
    let lines = build_env.lines();

    let mut var_map = HashMap::new();
    var_map.insert("OUT_DIR".to_string(), "".to_string());
    var_map.insert("ASM_FILES".to_string(), "".to_string());
    var_map.insert("LINK_SCRIPT".to_string(), "".to_string());
    var_map.insert("OUT_OBJ".to_string(), "".to_string());
    var_map.insert("OUT_IMG".to_string(), "".to_string());
    var_map.insert("LINK_OBJ".to_string(), "".to_string());

    for l in lines {
        // collect any of the list, always take the last one
        let l_str = str!(l);

        // take the stuff before '='
        // use the trait StringTools for these
        let var = l_str.prefix_before("=");
        let val = l_str.suffix_after("=");

        if var != "" && val != "" {
            // when debug is turned on
            println!("VAR FOUND: {} = {}", var, val);

            if BUILD_VARS.iter().any(|v| v == &var) {
                // get the replace that var of importance value
                var_map.insert(var.to_string(), val.to_string());
            }
        }
    }

    // return the map
    var_map
}

trait StringTools {
    fn prefix_before(&self, _char: &str) -> &str;
    fn suffix_after(&self, _char: &str) -> &str;
}

impl StringTools for String {
    fn prefix_before(&self, _char: &str) -> &str {
        // get rid of whitespaces trailing and before
        let _str = self.trim();

        // find the first '='
        let _indexer = _str.find("=");

        let _index = match _indexer {
            Some(ind) => ind,
            // if none, should technically return Result<&str, E>, assume format is right for now
            None => 0,
        };

        if _index == 0 {
            return "";
        }

        // take 0-index
        _str[.._index].trim()
    }
    fn suffix_after(&self, _char: &str) -> &str {
        let _str = self.trim();

        let _indexer = _str.find("=");

        let _index = match _indexer {
            Some(ind) => ind,
            None => 0,
        };

        if _index == 0 {
            return "";
        }

        _str[_index + 1..].trim()
    }
}
