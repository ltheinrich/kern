use kern::{CliBuilder, Command};

#[test]
fn command() {
    // define possible options and arguments
    let options = ["option1", "option2", "option3"];
    let paramopts = ["paramopt1", "paramopt2"];
    let arguments = generate_arguments(true);
    let arguments_wo = generate_arguments(false);

    // parse commands
    let builder = CliBuilder::new();
    let command = builder
        .clone()
        .options(&options)
        .paramopt(&paramopts)
        .build(&arguments);
    let command_wo = builder.build(&arguments_wo);

    // check both commands
    check_command(&command);
    check_command(&command_wo);

    // check specific for command
    assert_eq!(command.param("short-param2", "falsch"), "short-value2");
    assert_eq!(command.option("option1"), true);
    assert_eq!(command.option("option2"), true);
    assert_eq!(command.option("m"), true);
    assert_eq!(command.option("s"), true);
    assert_eq!(command.option("o"), true);
    assert_eq!(command.parameters().len(), 9);
    assert_eq!(command.options().len(), 6);
    assert_eq!(command.arguments().len(), 5);

    // check specific for command_wo
    assert_eq!(command_wo.param("short-param2", "falsch"), "falsch");
    assert_eq!(command_wo.option("option1"), false);
    assert_eq!(command_wo.option("option2"), false);
    assert_eq!(command_wo.option("m"), false);
    assert_eq!(command_wo.option("s"), false);
    assert_eq!(command_wo.option("o"), false);
    assert_eq!(command_wo.parameters().len(), 9);
    assert_eq!(command_wo.options().len(), 0);
    assert_eq!(command_wo.arguments().len(), 5);
}

fn generate_arguments(paramopts: bool) -> Vec<String> {
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
    if paramopts {
        arguments.push("--paramopt1".into());
    }
    arguments.push("--param4".into());
    arguments.push("value4".into());
    if paramopts {
        arguments.push("--paramopt2".into());
        arguments.push("value2".into());
    }
    arguments.push("some".into());
    arguments.push("more".into());
    arguments.push("arguments".into());
    arguments.push("--param-int=544".into());
    arguments.push("--param-bool=true".into());
    arguments.push("545".into());
    arguments.push("true".into());
    arguments
}

fn check_command(command: &Command) {
    // check command name
    assert_eq!(command.command(), "command");

    // check param
    assert_eq!(command.param("param1", "falsch"), "value1");
    assert_eq!(command.param("param2", "falsch"), "value2");
    assert_eq!(command.param("param3", "falsch"), "value3");
    assert_eq!(command.param("param4", "falsch"), "value4");
    assert_eq!(command.param("short-param1", "falsch"), "short-value1");

    // check parameter
    assert_eq!(command.parameter("param-int", 0), 544);
    assert_eq!(command.parameter("param-bool", false), true);

    // check option
    assert_eq!(command.option("option3"), false);

    // check arg
    assert_eq!(command.arg(0, "falsch"), "some");
    assert_eq!(command.arg(1, "falsch"), "more");
    assert_eq!(command.arg(2, "falsch"), "arguments");

    // check argument
    assert_eq!(command.argument(3, 0), 545);
    assert_eq!(command.argument(4, false), true);
}
