# permscan

A linux binary to filter files and directories based on permission criteria

## `permscan --help` output

```bash
Permission scanner 1.0.0
Scan a directory for files that match permission criteria

USAGE:
    permscan [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -m               If present, will return the list of files that match at least one criteria, else return the list of
                     files that match all criteria
    -V, --version    Prints version information

OPTIONS:
        --owner <owner>    Specify the owner of the file in the format user:group
        --user <user>      Specify permissions that the user who owns the file or directory needs to have on the item in
                           the format rwx
```

## License

This project is licensed under both :

* The Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* The MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)
