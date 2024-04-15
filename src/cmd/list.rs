use std::error::Error;
use std::fs::File;
use std::io::Read;
use crate::util::load_config_file;

pub fn list(_dirs: Vec<&str>) -> Result<String, Box<dyn Error>> {
    // read config file
    let config_file = load_config_file();
    let mut file = File::open(config_file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut resp =String::from("Alias\tAbs Path\n");
    let mut odd_comma = true;
    for c in content.chars() {
        if c == ',' {
            if odd_comma {
                resp.push('\t');
            } else {
                resp.push('\n');
            }
            odd_comma = !odd_comma;
        } else {
            resp.push(c)
        }
    }

    Ok(resp)
}