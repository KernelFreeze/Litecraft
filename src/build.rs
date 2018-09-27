#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    use winres::WindowsResource;

    let mut res = WindowsResource::new();

    res.set_icon("resources/litecraft/textures/logo.ico");
    res.compile().expect("Failed to embed resource icon");
}

#[cfg(unix)]
fn main() {}
