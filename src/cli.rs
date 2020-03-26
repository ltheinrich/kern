use std::collections::BTreeMap;
use std::str::FromStr;

/// Command represents an command parsed from the command-line
///
/// # Example
/// ```
/// use kern::Command;
/// use std::env;
///
/// let args: Vec<String> = env::args().collect();
/// let command = Command::from(&args, &["option"]);
/// ```
#[derive(Clone, Debug)]
pub struct Command<'a> {
    /// Command name
    command: &'a str,

    /// Map of parameters
    parameters: BTreeMap<&'a str, &'a str>,

    /// List of options
    options: Vec<&'a str>,

    /// List of arguments
    arguments: Vec<&'a str>,
}

// Command implementation
impl<'a> Command<'a> {
    /// Get command name
    pub fn command(&self) -> &'a str {
        // return comand name
        self.command
    }

    /// Get all parameters
    pub fn parameters(&self) -> &BTreeMap<&'a str, &'a str> {
        // return map of parameters
        &self.parameters
    }

    /// Get all arguments
    pub fn arguments(&self) -> &Vec<&'a str> {
        // return arguments list
        &self.arguments
    }

    /// Get specific parameter or default
    pub fn parameter<T: FromStr>(&self, name: &str, default: T) -> T {
        // return specific parameter or default
        match self.parameters.get(name) {
            Some(parameter) => parameter.parse().unwrap_or(default),
            None => default,
        }
    }

    /// Get specific parameter or default as &str
    pub fn param(&self, name: &str, default: &'a str) -> &str {
        // return specific parameter or default as &str
        match self.parameters.get(name) {
            Some(parameter) => parameter,
            None => default,
        }
    }

    /// Get all options
    pub fn options(&self) -> &Vec<&str> {
        // return options list
        &self.options
    }

    /// Check if option provided
    pub fn option(&self, name: &str) -> bool {
        // return whether the option is provided
        self.options.contains(&name)
    }

    /// Get argument at specific index or default
    pub fn argument<T: FromStr>(&self, index: usize, default: T) -> T {
        // return argument at specific index or default
        match self.arguments.get(index) {
            Some(argument) => argument.parse().unwrap_or(default),
            None => default,
        }
    }

    /// Get argument at specific index or default as &str
    pub fn arg(&self, index: usize, default: &'a str) -> &str {
        // return argument at specific index as &str
        match self.arguments.get(index) {
            Some(argument) => argument,
            None => default,
        }
    }

    /// Create a new Command from raw command line arguments without options
    pub fn without_options(raw: &'a [String]) -> Self {
        // return Command
        Self::from(raw, &[])
    }

    /// Create a new Command from raw command line arguments
    /// Provide the arguments list as &[&str]
    pub fn from(raw: &'a [String], filter_options: &[&str]) -> Self {
        // define command name
        let command = match raw.get(0) {
            Some(command) => command,
            None => "",
        };

        // define variables
        let mut parameters: BTreeMap<&str, &str> = BTreeMap::new();
        let mut options: Vec<&str> = Vec::new();
        let mut arguments: Vec<&str> = Vec::new();

        // define iteration parameters
        let mut parameter = "";
        let mut is_parameter = false;

        // iterate through raw arguments
        for (index, argument) in raw.iter().enumerate() {
            // check if first argument (command name)
            if index == 0 {
                // skip
                continue;
            }

            // check if previous argument is a parameter
            if is_parameter {
                // insert parameter into map
                parameters.insert(parameter, argument);

                // empty parameter, compile safe
                parameter = "";

                // next on is not a parameter
                is_parameter = false;
            } else {
                // closure to process parameters using equal sign
                let process_split = |parameters: &mut BTreeMap<&'a str, &'a str>,
                                     parameter: &mut &'a str,
                                     is_parameter: &mut bool,
                                     argument: &'a str| {
                    // split argument
                    let splits = argument.splitn(2, '=');

                    // loop through one or two splitted parameters
                    for split in splits {
                        // check if second
                        if *is_parameter {
                            // insert parameter into map
                            parameters.insert(parameter, split);

                            // proceed with next argument
                            *is_parameter = false;
                        } else {
                            // store parameter name
                            *parameter = split;

                            // next on is a parameter
                            *is_parameter = true;
                        }
                    }
                };

                // check if argument is a parameter
                if argument.starts_with("--") {
                    // remove preceding characters
                    let cut = match argument.len() {
                        len if len >= 3 => &argument[2..],
                        _ => argument,
                    };

                    // check if option
                    if filter_options.contains(&cut) {
                        // add to options
                        options.push(cut);

                        // continue with next argument
                        continue;
                    }

                    // process parameter
                    process_split(&mut parameters, &mut parameter, &mut is_parameter, cut);
                // check if argument is an option or short parameter
                } else if argument.starts_with('-') {
                    // remove preceding character
                    let cut: &'a str = match argument.len() {
                        len if len >= 2 => &argument[1..],
                        _ => argument,
                    };

                    // check if option
                    if filter_options.contains(&cut) {
                        // add to options
                        options.push(cut);

                        // continue with next argument
                        continue;
                    } else if cut.len() >= 2 && !cut.contains('=') {
                        // add all options to options
                        for i in 0..cut.len() {
                            // add only one character
                            options.push(match cut.get(i..=i) {
                                Some(option) => option,
                                None => continue,
                            });
                        }

                        // continue with next argument
                        continue;
                    } else if cut == "-" {
                        // process as argument
                        arguments.push(cut);
                        continue;
                    }

                    // process parameter
                    process_split(&mut parameters, &mut parameter, &mut is_parameter, cut);
                } else {
                    // add to arguments
                    arguments.push(argument);
                }
            }
        }

        // last parameter without value must be option
        if is_parameter {
            // add parameter to options
            options.push(parameter);
        }

        // return Command
        Self {
            command,
            parameters,
            options,
            arguments,
        }
    }
}
