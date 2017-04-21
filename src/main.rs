use std::env;
use std::str;
use std::fs::File;
use std::io::Read;
use std::error::Error;

const PATH_LIST_SEPARATOR: &'static str = ":";
const PATH_CONFIG: &'static str = ".addpath";
const ENV_HOME: &'static str = "HOME";
const ENV_PATH: &'static str = "PATH";

fn get_home() -> String {
	match env::var(ENV_HOME) {
		Ok(val) => val,
		Err(_) => panic!("Couldn't get ${} location", ENV_HOME),
	}
}

fn get_config(file: &String) -> String {
	let mut file = match File::open(&file) {
		Err(why) => panic!("Couldn't open config file: {}", why.description()),
		Ok(file) => file,
	};

	let mut config_string = String::new();

	match file.read_to_string(&mut config_string) {
		Err(why) => panic!("Couldn't read config file: {}", why.description()),
		Ok(_) => config_string,
	}
}

fn get_path() -> String {
	match env::var(ENV_PATH) {
		Ok(val) => val,
		Err(_) => panic!("Couldn't read ${} variable", ENV_PATH),
	}
}

fn process_path(path: &str, config: &str) -> String {
	let existing: Vec<&str> = path.split(PATH_LIST_SEPARATOR).collect();
	let pending: Vec<&str> = config.split('\n').collect();
	let mut result = Vec::<&str>::new();

	for item in &existing {
		if item.len() != 0 {
			result.push(item)
		}
	}

	for item in &pending {
		if item.len() == 0 {
			continue;
		}

		if existing.iter().any(|e| e == item) {
			continue;
		}

		result.push(item);
	}

	result.join(PATH_LIST_SEPARATOR)
}

fn main() {
	let config_file = format!("{}/{}", get_home(), PATH_CONFIG);
	let config_string = get_config(&config_file);
	let env_path = get_path();
	let env_path_new = process_path(&env_path, &config_string);

	print!("{}", env_path_new);
}

#[cfg(test)]
mod tests {
	use process_path;

	#[test]
	fn test_1() {
		let path = "/bin:/usr/bin:::";
		let config = "\
			/usr/bin\n\
			/opt/path1\n\
			/home/user/bin\n\
		";

		assert_eq!(process_path(&path, &config),
			"/bin:/usr/bin:/opt/path1:/home/user/bin");
	}
}
