use std::error::Error;

pub fn show_help(_dirs: Vec<&str>) -> Result<String, Box<dyn Error>> {
    Ok(
"ezcd: A shell utility to enhance directory navigation.

Usage:
  ezcd [options] [arguments]

Options:
  --set ALIAS [DIR...]        Set a new directory alias. If no directory segments are provided
                              after the alias, the current directory is used as the alias target.
                              For example, 'ezcd --set myDocs' sets the current directory as 'myDocs',
                              and 'ezcd --set myDocs usr local bin' sets '/usr/local/bin' as 'myDocs'.

  --list                      List all configured directory aliases.

  --update ALIAS [NEW_PATH]   Update an existing alias to a new path composed of space-separated
                              directory segments.

  --remove ALIAS              Remove an existing directory alias.

  --help                      Display this help and exit.

Arguments:
  ALIAS                       If the argument is a known directory alias, ezcd will change
                              to the directory associated with that alias.

  DIR                         Navigate to a directory by path. Multiple directory segments can be
                              concatenated using spaces, e.g., 'ezcd dir1 dir2' is equivalent to
                              navigating to '/dir1/dir2'.

Details:
  - The 'ezcd --set' command allows setting a new directory alias which can then be used to
    quickly navigate to frequently accessed directories.

  - Use 'ezcd --list' to display all aliases currently configured.

  - The 'ezcd --update' command can be used to change the path associated with an existing alias.

  - To remove an alias, use 'ezcd --remove'.

  - Without any options, ezcd treats arguments as parts of a directory path or an alias.

Examples:
  ezcd --set myDocs
  - Sets the current directory as an alias named 'myDocs'.

  ezcd --set projectDir usr local share doc
  - Sets an alias 'projectDir' for '/usr/local/share/doc'.

  ezcd --list
  - Lists all directory aliases.

  ezcd projectDir
  - Changes directory to the path associated with 'projectDir'.

  ezcd dir1 dir2
  - Changes directory to '/dir1/dir2'.

Note:
  - Directories in CDPATH are not used when a path starts with a slash ('/') or when
    using an alias defined by ezcd.
"
    .to_string()
    )
}