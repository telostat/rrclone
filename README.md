# rrclone

![GitHub contributors](https://img.shields.io/github/contributors/telostat/rrclone)
![GitHub](https://img.shields.io/github/license/telostat/rrclone)

`rrclone` is an Rclone wrapper to use YAML/JSON configuration for backup tasks.

> **TODO:** Provide full README.

> **NOTE:** This is a work-in-progress application with a very limited
> functionality. Expect breaking changes and improved functionality as we move
> on.

## Installation

1. Install [rclone](https://rclone.org/install/)
1. Put `rrclone` onto `$PATH`

## Usage

1. Configure rclone
1. Prepare configuration file for `rrclone`.
1. Run `rrclone` in dry-run mode:

    ```sh
    rrclone ./config.yaml --dry-run
    ```

1. Run rrclone:

    ```sh
    rrclone ./config.yaml
    ```

## Cross Compilation

Install [cross](https://github.com/cross-rs/cross):

```sh
cargo install -f cross
```

Cross compile, for example for Linux on aarch64:

```sh
cross build --target aarch64-unknown-linux-gnu --release
```

## License

See [LICENSE](./LICENSE) file.
