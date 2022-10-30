use shared_library_builder::{GitLocation, LibraryLocation, RustLibrary};

pub fn libgleam(version: Option<impl Into<String>>) -> RustLibrary {
    RustLibrary::new(
        "Gleam",
        LibraryLocation::Git(GitLocation::github("feenkcom", "libgleam").tag_or_latest(version)),
    )
    .package("libgleam")
}

pub fn latest_libgleam() -> RustLibrary {
    let version: Option<String> = None;
    libgleam(version)
}
