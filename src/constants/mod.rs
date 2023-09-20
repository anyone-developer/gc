pub static APP_SECTION: &str = "App";
pub static APP_VERSION: &str = "Version";
pub static APP_GIT_SOURCE: &str = "https://github.com/Anyone-Developers/xxx";

pub static COMMANDS: &str = "Commands";

pub static FILE_NAME: &str = ".gc_config";

pub static GC_APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static GC_APP_AUTHOR: &str = "Edward Zhang (zhang_nan_163@163.com)";
pub static GC_APP_ABOUT: &str = "Orchestrate good commands with your desired way.";

pub static GC_ADD_ABOUT: &str = "Add a command";
pub static GC_ADD_LONG_ABOUT: &str = "This is for adding command in gc.";
pub static GC_DELETE_ABOUT: &str = "Delete a command";
pub static GC_DELETE_LONG_ABOUT: &str = "This is for deleting command in gc.";
pub static GC_RUN_ABOUT: &str = "Run a command";
pub static GC_RUN_LONG_ABOUT: &str = "This is for executing command in gc.";

pub static GC_COMMAND_HELP: &str = "user defined command name. eg. gc add git-diff develop main. The [command] indicate to [git-diff].";
pub static GC_DETAIL_HELP: &str =
    "The shell command that needs to be executed. eg: \"git diff --name-only $0 $1\"";
pub static GC_PREFIX_HELP: &str = "Optional: for adding prefix to each line of output";
pub static GC_SUFFIX_HELP: &str = "Optional: for adding suffix to each line of output";
