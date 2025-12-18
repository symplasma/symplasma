# Symplasma

The main repo for the [Symplasma Protocol](https://www.symplasma.com/).

This repo will hold the Symplasma common library as well as a CLI for testing and troubleshooting.

## Development History

### Early History

This project has been in the research and planning stages for almost 2 decades. An initial prototype was written in Ruby over a decade ago however it focused on data transport...as most next gen internet protocols have.

More recently it has become increasingly obvious that, while data transport is important, data management, search and organization, and most critically, self-assigned identity with a Web of Trust are far more critical and generally neglected in the design of such protocols.

Recent work was initially focused in the [educe](https://github.com/symplasma/educe) repository. However, that repository was created when I was new to Rust. Thus, it's design and architecture leave much to be desired.

### Current Work

The current consensus of those working on Symplasma is that the code and functionality should be as granular and atomic as possible, allowing others to use the pieces they want without having to pull in any heavyweight libraries or binaries.

### Components

Currently, this library will serve as a common core for a number of programs:

#### Porcelain

- [free-launch](https://github.com/symplasma/free-launch): A fuzzy launcher written in Rust to replace Ulauncher.
- [retsyn](https://github.com/symplasma/retsyn): The indexing, search, and ranking library for Symplasma.
- [eunicode](https://github.com/egrieco/eunicode): A text processing CLI and library that helps sanitize text by removing the naughty bits to make strings good and safe.
- [statical](https://github.com/egrieco/statical): A calendar aggregator and generator to make maintaining calendars on static websites easier.

#### Plumbing

- [picleo](https://github.com/symplasma/picleo): A fuzzy picker/matcher CLI using nucleo. Similar to skim, FZF, et. al.
- [uurl](https://github.com/symplasma/uurl): A transformer and manipulator for Urls. Can be used via CLI or as a library.
- [giti](https://github.com/symplasma/giti): A program to retrieve information about git repos.
