# permscan
A linux binary to filter files and directories based on permission criterias
# `permscan --help` output
```
Permission scanner 1.0.0
Scan a directory for files that match permission criterias

USAGE:
    permscan [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -m               If present, will return the list of files that match at least one criteria, else return the list of
                     files that match all criterias
    -V, --version    Prints version information

OPTIONS:
        --owner <owner>    Specify the owner of the file in the format user:group
        --user <user>      Specify permissions that the user who owns the file or directory needs to have on the item in
                           the format rwx
```
