# permscan

A linux binary to filter files and directories based on permission criteria

## `permscan --help` output

```
Permission scanner 1.2.2
Scan a directory for files that match permission criteria.
Visit https://github.com/Pythack/permscan for more information.

USAGE:
    permscan [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -i               If present, will return the list of files that don't match with the criteria
    -m               If present, will return the list of files that match at least one criteria, else return the list of
                     files that match all criteria
    -V, --version    Prints version information

OPTIONS:
        --group <group>    Specify permissions that the group who owns the file or directory needs to have on the item
                           in the format ?rwx
        --other <other>    Specify permissions that users who does not own the file or directory needs to have on the
                           item in the format ?rwx
        --owner <owner>    Specify the owner of the file in the format user:group
        --user <user>      Specify permissions that the user who owns the file or directory needs to have on the item in
                           the format ?rwx
```

## Wiki

For a precise guide on what the options do and how to use them, see the [wiki](https://github.com/Pythack/permscan/wiki)

## Try it on Docker

Run `docker run -it --rm --name permscan ghcr.io/pythack/permscan:latest` on a machine with Docker installed and running.

## Installation

### GNU/Linux

* Download
<https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-gnu.zip>
* Decompress the downloaded folder.
* Inside the decompressed folder, there will be a binary named permscan: this is
  the program.
* Move it to /usr/local/bin to be able to run it at any time from the command
  line.

## Building

* As permscan is written in rust, you will need rust to build it. The
  [latest](https://www.rust-lang.org/tools/install) version is recommended.

* To build :

  * `git clone https://github.com/Pythack/permscan`
  * `cd permscan`
  * `cargo build --release`

## License

This project is licensed under both :

* The Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* The MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)
