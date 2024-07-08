Annotations about Getting Started section:

#### Raw

**rustc main.rs**
<p>Builds and generate executable in the same folder using file name. Example: *main.exe*</p>

#### Cargo

<p>Cargo is the project manager for Rust. Using Cargo.toml file inside project root folder, the properties of the project could be defined along with the project required dependencies.</p>

**cargo new {projectName}**
<p>Will create a new folder labeled with projectName and initial directory structure. A Git repository will be created with the correspondent .gitignore file.</p>
<p>Cargo expects all the source files to be inside *src* folder.</p>

###### Build and Run
**cargo build**
<p>Builds the project and an executable should appear inside */target/debug/* using *{projectName}.exe* as name.</p>
- Using --release compile with optimizations for **release**.</p>

**cargo run**
<p>Builds and run the project. </p>

**cargo check**
<p>Build but do not generate the executable. Faster way to check if code is compilable</p>