extern crate kern;

use kern::cli::Command;

// Test for Command::from
#[test]
fn from() {
    // define possible options
    let options = ["option1", "option2", "option3"];

    // initialize arguments list and add arguments
    let mut arguments = Vec::new();
    arguments.push("command".into());
    arguments.push("--param1=value1".into());
    arguments.push("--param2=value2".into());
    arguments.push("-short-param1=short-value1".into());
    arguments.push("--option1".into());
    arguments.push("-short-param2=short-value2".into());
    arguments.push("--option2".into());
    arguments.push("-mso".into());
    arguments.push("--param3".into());
    arguments.push("value3".into());
    arguments.push("--param4".into());
    arguments.push("value4".into());
    arguments.push("some".into());
    arguments.push("more".into());
    arguments.push("arguments".into());

    // parse command
    let command = Command::from(&arguments, &options);

    // check values
    assert_eq!(command.get_command(), "command");
    assert_eq!(*command.get_parameter("param1").unwrap(), "value1");
    assert_eq!(*command.get_parameter("param2").unwrap(), "value2");
    assert_eq!(*command.get_parameter("param3").unwrap(), "value3");
    assert_eq!(*command.get_parameter("param4").unwrap(), "value4");
    assert_eq!(
        *command.get_parameter("short-param1").unwrap(),
        "short-value1"
    );
    assert_eq!(
        *command.get_parameter("short-param2").unwrap(),
        "short-value2"
    );
    assert_eq!(command.is_option("option1"), true);
    assert_eq!(command.is_option("option2"), true);
    assert_eq!(command.is_option("option3"), false);
    assert_eq!(command.is_option("m"), true);
    assert_eq!(command.is_option("s"), true);
    assert_eq!(command.is_option("o"), true);
    assert_eq!(*command.get_argument(0).unwrap(), "some");
    assert_eq!(*command.get_argument(1).unwrap(), "more");
    assert_eq!(*command.get_argument(2).unwrap(), "arguments");

    // check lengths
    assert_eq!(command.get_parameters().len(), 6);
    assert_eq!(command.get_options().len(), 5);
    assert_eq!(command.get_arguments().len(), 3);
}

// Test for Command::without_options
#[test]
fn without_options() {
    // initialize arguments list and add arguments
    let mut arguments = Vec::new();
    arguments.push("command".into());
    arguments.push("--param1=value1".into());
    arguments.push("--param2=value2".into());
    arguments.push("-short-param1=short-value1".into());
    arguments.push("-short-param2=short-value2".into());
    arguments.push("-mso".into());
    arguments.push("--param3".into());
    arguments.push("value3".into());
    arguments.push("--param4".into());
    arguments.push("value4".into());
    arguments.push("some".into());
    arguments.push("more".into());
    arguments.push("arguments".into());

    // parse command
    let command = Command::without_options(&arguments);

    // check values
    assert_eq!(command.get_command(), "command");
    assert_eq!(*command.get_parameter("param1").unwrap(), "value1");
    assert_eq!(*command.get_parameter("param2").unwrap(), "value2");
    assert_eq!(*command.get_parameter("param3").unwrap(), "value3");
    assert_eq!(*command.get_parameter("param4").unwrap(), "value4");
    assert_eq!(
        *command.get_parameter("short-param1").unwrap(),
        "short-value1"
    );
    assert_eq!(
        *command.get_parameter("short-param2").unwrap(),
        "short-value2"
    );
    assert_eq!(command.is_option("option1"), false);
    assert_eq!(command.is_option("m"), true);
    assert_eq!(command.is_option("s"), true);
    assert_eq!(command.is_option("o"), true);
    assert_eq!(*command.get_argument(0).unwrap(), "some");
    assert_eq!(*command.get_argument(1).unwrap(), "more");
    assert_eq!(*command.get_argument(2).unwrap(), "arguments");

    // check lengths
    assert_eq!(command.get_parameters().len(), 6);
    assert_eq!(command.get_options().len(), 3);
    assert_eq!(command.get_arguments().len(), 3);
}
