## Summary

<!-- Describe what this PR changes and which issue it addresses (e.g. Fixes #123). -->

## Scope & Layer

Check the layer(s) this PR modifies:
- [ ] Rust application (`src/`)
- [ ] Cargo dependencies (`Cargo.toml` + `Cargo.lock`)
- [ ] Flatpak / Flathub metadata (`tomodorange.yaml`, `tomodorange.metainfo.xml`, `tomodorange.desktop`)
- [ ] Arch Linux / AUR (`PKGBUILD`)
- [ ] Nix (`flake.nix`)
- [ ] Documentation / CI

## Verification

List the exact commands you ran locally and their results (do not check boxes you did not run):
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy -- -D warnings`
- [ ] `cargo build --release --locked`
- [ ] Packaging validation (if applicable: `appstreamcli validate`, `flatpak-builder`, `makepkg`, `nix build`)

## Contributor & AI Policy Checklist

- [ ] I read [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md).
- [ ] If `Cargo.toml` changed, `Cargo.lock` is updated and `--locked` builds pass.
- [ ] All URLs, file paths, and identifiers in this diff were verified to exist.
- [ ] AI-assisted PR? (Yes / No): <!-- If Yes, confirm a human reviewed and tested the final diff. -->
