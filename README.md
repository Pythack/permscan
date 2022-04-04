# permscan

permscan is a linux and macOS binary to filter file system items based on
permission criteria. By default, it will search not-recursively and ignore hidden
files but this behavior can be modified using flags. You can also specify the
path of the directory you want to search into.

## Contents

* [Help](##Help)
* [Wiki](##Wiki)
* [Run in docker](##Run-in-docker)
* [Installation](##Installation)
* [Building](##Building)
* [License](##License)

## Help

```
Permission scanner 2.2.10
Scan a directory for files that match permission criteria.
Visit https://github.com/Pythack/permscan#readme for more information.

USAGE:
    permscan [FLAGS] [OPTIONS] [path]

FLAGS:
    -a                 Parse hidden files as well
    -b, --build        If the update flag is also present and the user decide to update, the update will be built from
                       source
    -u, --update       Check for a newer version of permscan
    -e, --exit-info    Show exit code
    -h, --help         Prints help information
    -i                 Return the list of files that don't match with the criteria
    -m                 Return files that match at least one criteria, instead of those that match all criteria
    -r                 If present, permscan will recursively traverse the folder
    -V, --version      Prints version information

OPTIONS:
        --type <file-type>    Specify the type of the object
        --group <group>       Specify permissions that the group who owns the file or directory needs to have on the
                              item in the format @rwx
        --other <other>       Specify permissions that users who does not own the file or directory needs to have on the
                              item in the format @rwx
        --owner <owner>       Specify the owner of the file in the format user:group
        --user <user>         Specify permissions that the user who owns the file or directory needs to have on the item
                              in the format @rwx

ARGS:
    <path>    The path of the directory your want to look into. [default: ./]
```

## Wiki

For a precise guide on the different arguments, options, flags and errors, see the [wiki](https://github.com/Pythack/permscan/wiki)

## Run in Docker

Run this on a machine with Docker installed and running :

```console
docker run -it --rm --name permscan ghcr.io/pythack/permscan-gnu:latest
```

## Installation

```
wget https://raw.githubusercontent.com/Pythack/permscan/master/permscan-installer.sh
```

```
chmod +x ./permscan-installer.sh
```

```
./permscan-installer.sh
```

This will install the latest version but you can also specify a version using
the -v flag followed of the version number.
Ex :

```
./permscan-installer.sh -v 2.2.10
```

## Building

* As permscan is written in rust, you will need rust to build it. The
  [latest](https://www.rust-lang.org/tools/install) version is recommended.

```
wget https://raw.githubusercontent.com/Pythack/permscan/master/permscan-installer.sh
```

```
chmod +x ./permscan-installer.sh
```

```
./permscan-installer.sh -b
```

## License

This project is licensed under both :

* The Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* The MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)
