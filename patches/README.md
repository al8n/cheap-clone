# hostaddr 1.1 migration

`hostaddr-1.1.0.patch` removes hostaddr's old cheap-clone dependency, feature
forwarding and trait implementations, bumps its version to 1.1.0, and documents
the manifest migration. It was prepared against the clean local hostaddr commit
`75d0231` and tested in a separate checkout. The user has applied it to the
sibling hostaddr checkout; the resulting diff matches this patch exactly, and
the downstream integration tests also pass against that checkout.

For another clean checkout of the same hostaddr revision, apply it from the
cheap-clone repository as follows. Do not apply it again to the updated checkout:

```sh
git -C ../hostaddr apply --check ../cheap-clone/patches/hostaddr-1.1.0.patch
git -C ../hostaddr apply ../cheap-clone/patches/hostaddr-1.1.0.patch
```

Before hostaddr 1.1 is published, test this workspace against the updated local
checkout using a command-line override (do not commit the override):

```sh
cargo test --workspace --all-features --config 'patch.crates-io.hostaddr.path="../hostaddr"'
cargo test --manifest-path tests/fixtures/hostaddr/Cargo.toml --config 'patch.crates-io.hostaddr.path="../hostaddr"'
```

Publish hostaddr 1.1.0 first. Once it is available in the registry, the normal
cheap-clone build needs no override. Then release the cheap-clone workspace;
the release workflow publishes cheap-clone-derive before cheap-clone 1.0.0.
Both packages need the appropriate crates.io trusted-publishing configuration.

The old hostaddr feature and 0.3 trait implementations are intentionally removed.
Consumers must move to cheap-clone 1 with its hostaddr integration enabled.
