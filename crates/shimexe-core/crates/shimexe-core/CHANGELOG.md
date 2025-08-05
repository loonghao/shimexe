# Changelog

## [0.5.6](https://github.com/loonghao/shimexe/compare/shimexe-core-v0.5.5...shimexe-core-v0.5.6) (2025-08-05)


### Features

* add comprehensive archive support and package management ([48ce5c5](https://github.com/loonghao/shimexe/commit/48ce5c54b6f74c1d57fa1960cf7c962ab476dc40))
* add high-level ShimManager API and update Chinese README ([e98666d](https://github.com/loonghao/shimexe/commit/e98666dceb763aea2109766f5f5f10fdac00ccf1))
* add HTTP URL support for automatic executable download ([28abbc0](https://github.com/loonghao/shimexe/commit/28abbc0902fe0b68275cf3c5f87ea7288960698a))
* disable clippy uninlined_format_args lint ([e2d69dd](https://github.com/loonghao/shimexe/commit/e2d69ddfa39690858863deac9a3b3570c8ffed66))
* enhance auto-update with turbo-cdn integration ([2a5e660](https://github.com/loonghao/shimexe/commit/2a5e6607a02fda18df9bd120cd1c054af14a61bb))
* enhance HTTP URL support with persistent download tracking ([d169cb7](https://github.com/loonghao/shimexe/commit/d169cb752d916dc1824adb6287656d44c8d5c12c))
* enhance package publishing and fix env config ([332cad3](https://github.com/loonghao/shimexe/commit/332cad316beebd4af5dfd4cf6f3362ce5339779c))
* enhance shim configuration with dynamic template system and improved args handling ([00bfd93](https://github.com/loonghao/shimexe/commit/00bfd93df15a3873032ec0b43dd22dea355d3049))
* integrate turbo-cdn and restructure tests ([95de3fb](https://github.com/loonghao/shimexe/commit/95de3fbcd1f6fca4d57edcf21f216df7b7e978f2)), closes [#45](https://github.com/loonghao/shimexe/issues/45)
* optimize performance and fix workflow ([76a65e4](https://github.com/loonghao/shimexe/commit/76a65e4379f79fc5562b68d805026459f694f85b))
* setup release-please automation and add crate READMEs ([af4bd8a](https://github.com/loonghao/shimexe/commit/af4bd8a9da1b119d9bd016ca41f528871c129a39))
* upgrade turbo-cdn to 0.4.3 and improve verbose logging control ([949a4f6](https://github.com/loonghao/shimexe/commit/949a4f661c57c715fab31ca71248f47289d3098f))


### Bug Fixes

* adjust performance test timeouts for Windows environment ([d0c4bd3](https://github.com/loonghao/shimexe/commit/d0c4bd379cb06d8e494edc7011a46e350a89883d))
* **deps:** update rust crate zip to v4 ([f7223d5](https://github.com/loonghao/shimexe/commit/f7223d5216305757045da9fc72d2cd131cb74f07))
* resolve CI configuration and testing issues ([8754f55](https://github.com/loonghao/shimexe/commit/8754f553a9ce9d601be7713ab02f0b9aa06c9628))
* resolve clippy and rustdoc warnings for CI compatibility ([a1acc7b](https://github.com/loonghao/shimexe/commit/a1acc7b51892c356d964ac21fe99bec6ecadc24f))
* resolve clippy warnings and compilation errors ([ee01ee3](https://github.com/loonghao/shimexe/commit/ee01ee3c161c9b41df0764e69f039eceb4f861a9))
* resolve clippy warnings and implement Default trait for ConfigCache ([e6641b3](https://github.com/loonghao/shimexe/commit/e6641b3a1a59e41e0f97454bfd32e2fc53f8bcc0))
* resolve OpenSSL cross-compilation issues with rustls ([9c11fab](https://github.com/loonghao/shimexe/commit/9c11fabbb25906dddc34d2b71db4ad3c9386b502))
* resolve release-plz config and clippy warnings ([409d2e3](https://github.com/loonghao/shimexe/commit/409d2e3576424de45fd9f580d44e8ffcd90bd885))
* resolve remaining clippy uninlined format args warnings ([26266bb](https://github.com/loonghao/shimexe/commit/26266bb516907fab760f793b5103827e83e11f0d))
* resolve remaining redundant_closure warning in runner.rs ([e22b419](https://github.com/loonghao/shimexe/commit/e22b419b0bce0077bcf03a7033b6d61be472edd8))
* set explicit version for shimexe-core to resolve cargo-workspace issue ([304e13b](https://github.com/loonghao/shimexe/commit/304e13b1a6b66153e7b538b3c3c665b1bcaf8c4b))


### Code Refactoring

* rename [@pkg](https://github.com/pkg) to pkg directory and fix code quality ([e3b0da8](https://github.com/loonghao/shimexe/commit/e3b0da89c2042d1ab60cace62352c96d327e830b))


### Documentation

* update README and add comprehensive unit tests ([c35b70d](https://github.com/loonghao/shimexe/commit/c35b70df7102a0d9fdae9394e10ec8aa13967109))
