//! Traits that allow standard interaction with all kinds

pub trait Kind {
    // items: which lists all items of a given kind

    // files: which lists all files of interest

    // indexable: a list of files that should be passed to search indexers
    //            may need to return file type as well as some kinds may result in multiple types of files

    // collections: directories into which new items can be collected

    // storage: directories into which new items can be collected
}
