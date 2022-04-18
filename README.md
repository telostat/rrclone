# rrclone

![GitHub contributors](https://img.shields.io/github/contributors/telostat/rrclone)
![GitHub](https://img.shields.io/github/license/telostat/rrclone)

`rrclone` is an Rclone wrapper to use YAML/JSON configuration for backup tasks.

> **TODO:** Provide full README.
>
> **NOTE:** This is a work-in-progress application with a very limited
> functionality. Expect breaking changes and improved functionality as we move
> on.

## Installation

First, make sure that [rclone](https://rclone.org/install/) is
installed.

Then, download the latest version from releases for your architecture
and put it in your path:

```sh
curl -sLo /tmp/rrclone "https://github.com/telostat/rrclone/releases/latest/download/rrclone-$(uname -s)-$(uname -m)"
sudo install /tmp/rrclone /usr/local/bin
```

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

## Releasing

Install [cross](https://github.com/cross-rs/cross):

```sh
cargo install -f cross
```

First, merge `develop` to `main`:

```sh
git checkout main
git merge --no-ff develop
```

Update the version information in `Cargo.toml` if required. Update the
`CHANGELOG.md` as well (currently using `standard-version`).

Set the release tag (example is `0.0.2`):

```sh
export RELTAG="0.0.2"
```

Commit changes:

```sh
git commit -am "chore(release): ${RELTAG}"
```

Tag:

```sh
git tag -a -m "Release ${RELTAG}" "${RELTAG}"
```

Push everything:

```sh
git push --follow-tags origin main
```

Create a new GitHub release:

```sh
gh release create "${RELTAG}" --generate-notes --title "v${RELTAG}" --draft
```

Build on your own architecture and upload to the release(hoping that it is Linux
x86_64, otherwise must add this to cross build instructions, too):

```sh
cargo build --release
mv target/release/rrclone "target/release/rrclone-$(uname -s)-$(uname -m)"
gh release upload "${RELTAG}" --clobber "target/release/rrclone-$(uname -s)-$(uname -m)"
```

Build and upload other architectures:

```sh
cross build --target aarch64-unknown-linux-gnu --release
mv target/aarch64-unknown-linux-gnu/release/rrclone "target/aarch64-unknown-linux-gnu/release/rrclone-Linux-aarch64"
gh release upload "${RELTAG}" --clobber "target/aarch64-unknown-linux-gnu/release/rrclone-Linux-aarch64"

cross build --target arm-unknown-linux-gnueabihf --release
mv target/arm-unknown-linux-gnueabihf/release/rrclone "target/arm-unknown-linux-gnueabihf/release/rrclone-Linux-arm"
gh release upload "${RELTAG}" --clobber "target/arm-unknown-linux-gnueabihf/release/rrclone-Linux-arm"

cross build --target armv7-unknown-linux-gnueabihf --release
mv target/armv7-unknown-linux-gnueabihf/release/rrclone "target/armv7-unknown-linux-gnueabihf/release/rrclone-Linux-armv7"
gh release upload "${RELTAG}" --clobber "target/armv7-unknown-linux-gnueabihf/release/rrclone-Linux-armv7"
```

Switch to develop and rebase:

```sh
git checkout develop
git rebase main
```

Bump development version, commit and push:

```sh
git push
```

## License

See [LICENSE](./LICENSE) file.
