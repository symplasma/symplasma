# Overview

A Rust-based CLI app and library to handle entities and common functionality for the Symplasma project.

## Structure

### Config

- Config for this project should be stored in [The KDL Document Language](https://kdl.dev/).
- The config files should be stored at the appropriate paths according to the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir/latest/).

### Sources

- Circles
- Projects (might merge with circles)
- Repos
- Markdown (mostly notes)
- Pictures
- Videos
- Music
- Audio

Some notes about the sources:

- For every source listed, there should be a section in the config that allows paths to be listed where all of the children of each path are an instance of the given type.
- `Circles`, `Projects`, and `Repos` refer to collections of files while the rest of the categories refer to individual files.

## CLI App

The following subcommands should be supported:

- `list sources`: List all possible data sources.
- `list <SOURCE>`: List all directories and files from the given source.
- `find [<SOURCE>] <FILE_NAME>`: Find the path to the given file, scoped to the source type if that's provided.
- `find-or-create [<SOURCE>] <FILE_NAME>`: The same as find, however if the file name provided cannot be found, create it instead.

The main functionality should be implemented in the library with the CLI app wrapping the library and calling its functions.
