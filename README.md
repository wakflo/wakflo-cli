<p align="center">
  <img src="https://img.shields.io/badge/version-0.0.1-green"> <img src="https://img.shields.io/github/license/wakflo/wakflo-cli?color=pink"> <img src="https://img.shields.io/tokei/lines/github/wakflo/fleet?color=white&label=lines%20of%20code"> <img src="https://img.shields.io/github/languages/top/wakflo/fleet?color=%230xfffff">
</p>

<br>


[Wakflo](https://wakflo.ai) is a blazing fast build tool for Rust.
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
cargo install --git https://github.com/wakflo/wakflo-rs wakflo-rs
```


## How does fleet work?

Fleet works by optimizing your builds using existing tooling available in the Rust ecosystem, including seamlessly integrating sccache, lld, zld, ramdisks (for those using WSL or HDD's) et al.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/wakflo/fleet/tags).

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE.md](LICENSE) file for details.