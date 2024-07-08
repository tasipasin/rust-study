Annotations about Guessing Game section:

The website to get crates is *crates.io*

To use *rand* create, a dependency was defined in *Cargo.toml*. After first run, Cargo will create *Cargo.lock* with the dependencies version established and their respective dependencies.

The *Cargo.toml* acts like a base, but Cargo will always prefer *Cargo.lock* crates definitions to decide which version to download, assuring the code keeps working.

To really update creates versions, run **cargo update**. It will ignore *Cargo.lock* looking for newer versions and update *Cargo.lock* definitions.

Documentation are generally available online on *crates.io*, but can be locally generated using **cargo doc --open**