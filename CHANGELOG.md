# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0](https://github.com/bornacvitanic/file_aggregator/compare/v0.1.0..v0.2.0) - 2024-07-24

### Documentation

- Update README.md to add another planned feature
 - ([8864bd5](https://github.com/bornacvitanic/file_aggregator/commit/8864bd561d12063c75c45a56bbb77dccce8581fc))
- Update Cargo.toml and README.md to rename package
 - ([689013d](https://github.com/bornacvitanic/file_aggregator/commit/689013d00c2b01b541da7a3e4fdae191846099f2))
- Update README.md to add crates.io badge
 - ([43c7ba4](https://github.com/bornacvitanic/file_aggregator/commit/43c7ba456ec3129b5985edefef6751ac7ea10ddc))
- Update README.md to add feature roadmap
 - ([f50144b](https://github.com/bornacvitanic/file_aggregator/commit/f50144b95b736bd9da025219f6c552904374e70c))
- Update README.md to add dynamic badges
 - ([1dff83e](https://github.com/bornacvitanic/file_aggregator/commit/1dff83e3d87d90c8c43aaf2003cf350aa0d9bc7a))


### Features

- Update file_operations.rs to add support for deleting of files when distributing
 - ([57c06d7](https://github.com/bornacvitanic/file_aggregator/commit/57c06d71ca8e7f21320a906c1f8b01a98da2fae0))
- Update Cargo.toml to add license field
 - ([3140be2](https://github.com/bornacvitanic/file_aggregator/commit/3140be2687ac42880fac4603b13e33e83a646926))
- Update Cargo.toml to add keywords and categories
 - ([1d7b040](https://github.com/bornacvitanic/file_aggregator/commit/1d7b0400cef9e08bcd33af5aa34feda79f3d1f60))
- Add rust.yml github action
 - ([b354ef2](https://github.com/bornacvitanic/file_aggregator/commit/b354ef2ee6f0428824606ea4f4cde122499fa2f1))


### Revert

- Revert "Update Cargo.toml and README.md to rename package"

This reverts commit 689013d00c2b01b541da7a3e4fdae191846099f2.
 - ([0123189](https://github.com/bornacvitanic/file_aggregator/commit/0123189976cf87bfc3fda3812a6119cc462eebd3))


### Styling

- Update all files to format them using fmt
 - ([fe808ea](https://github.com/bornacvitanic/file_aggregator/commit/fe808eaf7a4d0c8c42ca5da2125da03f1c42f1b1))
- Update file_operations.rs to clean up methods
 - ([c3f2053](https://github.com/bornacvitanic/file_aggregator/commit/c3f20538808f2f2a7ed26560d12ac1b2b2fa51ea))


### Testing

- Udate file_operations.rs to add more edge case unit tests
 - ([c0cb4f8](https://github.com/bornacvitanic/file_aggregator/commit/c0cb4f8520eeb134fbf1b1e79de932718028d5a1))
- Update file_operations to add basic unit tests
 - ([2192132](https://github.com/bornacvitanic/file_aggregator/commit/2192132959793cab63a9ca2f00f2830742efa1a1))


### Updates

- Update file_operations.rs to optimize method arguments
 - ([58e6d72](https://github.com/bornacvitanic/file_aggregator/commit/58e6d7275ffb12e85f501eb4a35572229627e4aa))
- Update Cargo.toml to specify compatible version requirement
 - ([ade824c](https://github.com/bornacvitanic/file_aggregator/commit/ade824c1711883f5217a3f684a5546e0429937f8))
- Update cli.rs and main.rs to make Aggregate the default command
 - ([0fd6968](https://github.com/bornacvitanic/file_aggregator/commit/0fd6968a7e35e018453f661f8c71819599e12d99))


## [0.1.0] - 2024-07-22

### Bug Fixes

- Fix main.rs to handle no extensions provided
 - ([25e1ce9](https://github.com/bornacvitanic/file_aggregator/commit/25e1ce941f75e232431ff4057270d44878763fef))


### Documentation

- Add README.md and LICENSE.md
 - ([ff04250](https://github.com/bornacvitanic/file_aggregator/commit/ff04250075e604fdc195e0df09f1fb6128b20bcf))


### Features

- Add file_operations.rs to move file operation related methods out of main.rs
 - ([932915b](https://github.com/bornacvitanic/file_aggregator/commit/932915b1b108a46ff68a03c803cc444ba5faf127))
- Update cli.rs to add better help strings
 - ([dc088b1](https://github.com/bornacvitanic/file_aggregator/commit/dc088b1e3157f8018f4acfed1a07a69b5f220bc9))
- Update Cargo.toml to add custom build file name
 - ([7de96d2](https://github.com/bornacvitanic/file_aggregator/commit/7de96d2756c1f1faf43ef572f1dfbf25b8e7e23e))
- Update mainl.rs to add extension filtering support
 - ([5d78107](https://github.com/bornacvitanic/file_aggregator/commit/5d781070e3c6c4eb642a1899ebaa4383ac5112bc))
- Update main.rs to implement distribution logic
 - ([84461ba](https://github.com/bornacvitanic/file_aggregator/commit/84461baf277711ac108747e19376bb0b2e824001))
- Add copypasta and update main.rs to post the contents of files to the clipboard
 - ([6c609be](https://github.com/bornacvitanic/file_aggregator/commit/6c609be51d5a90a0c2d1bfe45373a89f924dbeab))
- Update main.rs to add file content reading
 - ([6d35d7c](https://github.com/bornacvitanic/file_aggregator/commit/6d35d7c33c89a0ee9fb188f4975f3fba2fface2e))
- Add walkdir and upate main.rs to print out all files from all sub-directories from the root directory
 - ([ad9a1ec](https://github.com/bornacvitanic/file_aggregator/commit/ad9a1ec375f3739f40fc08db19a03cca599cbae0))
- Add clap library and cli.rs to specify basic command line arguments
 - ([e49939f](https://github.com/bornacvitanic/file_aggregator/commit/e49939f4be33b5bf1aade0808f7986ae274669e9))


### Updates

- Update file_aggregator.iml
 - ([d6778d2](https://github.com/bornacvitanic/file_aggregator/commit/d6778d23af84b2349269ee80de10e90ce33c4bc0))


