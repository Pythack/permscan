# permscan

A linux and macOS binary to filter file system items based on permission criteria

## Help

```
Permission scanner 2.2.6
Scan a directory for files that match permission criteria.
Visit https://github.com/Pythack/permscan#readme for more information.

USAGE:
    permscan [FLAGS] [OPTIONS] [path]

FLAGS:
    -a                 If present, permscan will parse hidden files as well
    -u, --update       Check for a newer version of permscan
    -e, --exit-info    If present, print exit code
    -h, --help         Prints help information
    -i                 If present, will return the list of files that don't match with the criteria
    -m                 If present, will return the list of files that match at least one criteria, else return the list
                       of files that match all criteria
    -r                 If present, will recursively traverse the folder
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

For a precise guide on  the different arguments, options, flags and errors, see the [wiki](https://github.com/Pythack/permscan/wiki)

## Run in Docker

Run this on a machine with Docker installed and running :

```console
docker run -it --rm --name permscan ghcr.io/pythack/permscan-gnu:latest
```

## Installation

### Using our installer

```
wget https://raw.githubusercontent.com/Pythack/permscan/master/permscan-installer.sh
```

```
chmod +x ./permscan-installer.sh
```

```
./permscan-installer.sh
```

### Manually

#### GNU/Linux

```
wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-gnu.tar.gz
```

```
tar -xzvf permscan-x86_64-unknown-linux-gnu.tar.gz
```

```
sudo mv permscan-x86_64-unknown-linux-gnu/permscan /bin
```

#### Linux-musl

```
wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-musl.tar.gz
```

```
tar -xzvf permscan-x86_64-unknown-linux-musl.tar.gz
```

```
sudo mv permscan-x86_64-unknown-linux-musl/permscan /bin
```

#### MacOS

* If you have an Apple chip, your architecture is ARM.
* If you have an Intel chip, your architecture is x86_64.

##### ARM

```
wget https://github.com/Pythack/permscan/releases/latest/download/permscan-aarch64-apple-darwin.zip
```

```
unzip permscan-aarch64-apple-darwin.zip
```

```
sudo mv permscan-aarch64-apple-darwin/permscan /usr/local/bin
```

##### x86_64

```
wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-apple-darwin.zip
```

```
unzip permscan-x86_64-apple-darwin.zip
```

```
sudo mv permscan-x86_64-apple-darwin/permscan /usr/local/bin
```

## Building

* As permscan is written in rust, you will need rust to build it. The
  [latest](https://www.rust-lang.org/tools/install) version is recommended.

* To build :

  ```
  git clone https://github.com/Pythack/permscan
  ```

  ```
  cd permscan
  ```

  ```
  cargo build --release
  ```

## License

This project is licensed under both :

* The Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* The MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)
