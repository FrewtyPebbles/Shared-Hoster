use std::{env, collections::HashMap, process::exit};
// Handles the help flag.
pub fn help(args: &HashMap<String, String>) {
    if !args.keys().any(|arg| arg.as_str() == "help") {return};
    // ^ Return if no help flag specified.

    let flag = args.get("help").unwrap();

    // Help flag specified.
    match flag.as_str() {
		// Now print the help message for the specified flag.

        "max_servers"|"m_s" => {
            println!("--------------------------\nmax_servers (default=10) :\n--------------------------\n\n\tThe \"max_servers\" flag is used to specify an integer representing how many http servers to allow on this particular vps.  The resources available are split by that integer and partitioned to each http server instance.\n")
        }

        "debug"|"d"|"dbg" => {
            println!("-----------------------\ndebug (default=false) :\n-----------------------\n\n\tThe \"debug\" flag is a developer flag used to enable debugging features.  You do not specify a value for this flag.  If this flag is included, debug mode will be enabled.  If the flag is not included, debug mode will be disabled.\n")
        }

        _ => {
            println!("-----------------\nAvailable Flags :\n-----------------\n\n\t- \"debug\" | \"d\" | \"dbg\"\n\t\tEnable debug mode.\n\n\t- \"max_servers\" | \"m_s\"\n\t\tSpecify the maximum number of http server instances.\n")
        }

    }

    // Exit the program after using the help flag.
    exit(0);
}

// Used to parse CLI arguments into a hash map.
pub fn get_args() -> HashMap<String, String> {
    let args: Vec<String> = env::args().collect();
    
    let mut config: HashMap<String, String> = HashMap::new();

	// Insert default values for flags.
	config.insert("debug".to_string(), "false".to_string());
	config.insert("max_servers".to_string(), "10".to_string());

    let mut flag = "";
    for arg in args {
        // Detect that argument is a flag.
        if arg.starts_with("-") {

            // Process the argument for matching.
            let processed_arg = arg
                .strip_prefix("-")
                .unwrap().to_lowercase();

            // Get current flag.
            match processed_arg.as_str() {

                "max_servers"|"m_s" => {
                    // Sets the max servers, default is 10.
                    flag = "max_servers";
                }

                "help"|"h" => {
                    // Provides help for the specified flag.
                    flag = "help";
                }

                "debug"|"d"|"dbg" => {
                    // Enables debug mode.
                    config.insert("debug".to_string(), "true".to_string());
                }

                _ => {
                    // Invalid flag, close program.
                    panic!("\"{processed_arg}\" is not a valid flag!")
                }

            }

            continue;
        } else if flag != "" {
            // Insert the value for the current flag.
            config.insert(flag.to_string(), arg);
            
            flag = "";
        }
    }
    if flag == "help" {
        // No help flag parameter specified.
        config.insert(flag.to_string(), "".to_string());

    } else if flag != "" {
        // No required flag parameter specified.
        panic!("The flag \"{flag}\" must be followed by a value use \"-help {flag}\" for more info.")

    }

    return config;
}