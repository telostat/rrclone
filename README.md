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
2. Put `rrclone` onto `$PATH`

## Usage

1. Configure rclone
2. Prepare configuration file for `rrclone`.
3. Run `rrclone` in dry-run mode:
  ```sh
  rrclone ./config.yaml --dry-run
  ```
4. Run rrclone:
  ```sh
  rrclone ./config.yaml
  ```

## License

See [LICENSE](./LICENSE) file.
