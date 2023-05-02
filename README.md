<<<<<<< HEAD
<p align="center">
  <img src="https://img.shields.io/badge/version-0.0.1-green"> <img src="https://img.shields.io/github/license/wakflo/wakflo-cli?color=pink"> <img src="https://img.shields.io/tokei/lines/github/wakflo/fleet?color=white&label=lines%20of%20code"> <img src="https://img.shields.io/github/languages/top/wakflo/fleet?color=%230xfffff">
=======
![image](https://user-images.githubusercontent.com/63039748/164709140-8bb96d45-972e-4ac5-8e0e-ae566e673761.png)

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.0--beta-green"> <img src="https://img.shields.io/github/license/wakflo/fleet?color=pink"> <img src="https://img.shields.io/tokei/lines/github/wakflo/fleet?color=white&label=lines%20of%20code"> <img src="https://img.shields.io/github/languages/top/wakflo/fleet?color=%230xfffff">
>>>>>>> 85173fa (feat: first commit)
</p>

<br>


<<<<<<< HEAD
[Wakflo](https://wakflo.ai) is a blazing fast build tool for Rust.
=======
[Fleet](https://fleet.rs) is a blazing fast build tool for Rust. Compiling with Fleet is up-to 5x faster than with `cargo`.

**Note**: Since  Fleet is still under development, it might not be completely stable yet. Feel free to open any issues or bug reports at [issues](https://github.com/wakflo/fleet/issues/).

>>>>>>> 85173fa (feat: first commit)
<br>

# :zap: Installation

On MacOS & Linux:
```bash
curl -L get.wakflo.ai | sh
```
<br>

On Windows:
```powershell
iwr -useb windows.wakflo.ai | iex
```

## Building from source
Prerequisites: **Rust**
```powershell
<<<<<<< HEAD
cargo install --git https://github.com/wakflo/wakflo-cli wakflo
=======
cargo install --git https://github.com/wakflo/wakflo-rs wakflo-rs
>>>>>>> 85173fa (feat: first commit)
```


## How does fleet work?

Fleet works by optimizing your builds using existing tooling available in the Rust ecosystem, including seamlessly integrating sccache, lld, zld, ramdisks (for those using WSL or HDD's) et al.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/wakflo/fleet/tags).

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE.md](LICENSE) file for details.