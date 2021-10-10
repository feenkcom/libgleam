use shared_library_builder::{GitLocation, LibraryLocation, RustLibrary};

pub fn libgleam(version: impl Into<String>) -> RustLibrary {
    RustLibrary::new(
        "Gleam",
        LibraryLocation::Git(GitLocation::github("feenkcom", "libgleam").tag(version)),
    )
    .package("libgleam")
}
