# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).




## [0.5.13](https://github.com/loonghao/shimexe/compare/v0.5.12...v0.5.13) (2025-08-10)


### Features

* add --info flag to display detailed system and version information ([cfd86c1](https://github.com/loonghao/shimexe/commit/cfd86c18370653d984132e7d56cc05d2d4675062))
* add comprehensive archive support and package management ([48ce5c5](https://github.com/loonghao/shimexe/commit/48ce5c54b6f74c1d57fa1960cf7c962ab476dc40))
* add comprehensive vx integration example ([c088d97](https://github.com/loonghao/shimexe/commit/c088d972664de35fcbc8ec8b664f6d1544c6dfcf))
* add high-level ShimManager API and update Chinese README ([e98666d](https://github.com/loonghao/shimexe/commit/e98666dceb763aea2109766f5f5f10fdac00ccf1))
* add HTTP URL support for automatic executable download ([28abbc0](https://github.com/loonghao/shimexe/commit/28abbc0902fe0b68275cf3c5f87ea7288960698a))
* add multi-platform package management and enhanced badges ([d60371a](https://github.com/loonghao/shimexe/commit/d60371ab02985e3e87d2fab565cd708c147fa59b))
* add src directory monitoring to release-please ([5f1b065](https://github.com/loonghao/shimexe/commit/5f1b06519209ee44756ff6835482cf507f49cd90))
* disable clippy uninlined_format_args lint ([e2d69dd](https://github.com/loonghao/shimexe/commit/e2d69ddfa39690858863deac9a3b3570c8ffed66))
* enhance auto-update with turbo-cdn integration ([2a5e660](https://github.com/loonghao/shimexe/commit/2a5e6607a02fda18df9bd120cd1c054af14a61bb))
* enhance CI workflow with cross-compilation testing ([afb7a58](https://github.com/loonghao/shimexe/commit/afb7a583ef923f3211156f1b29377e5af024fa27))
* enhance HTTP URL support with persistent download tracking ([d169cb7](https://github.com/loonghao/shimexe/commit/d169cb752d916dc1824adb6287656d44c8d5c12c))
* enhance package publishing and fix env config ([332cad3](https://github.com/loonghao/shimexe/commit/332cad316beebd4af5dfd4cf6f3362ce5339779c))
* enhance README with logo and improve vx integration ([6085f46](https://github.com/loonghao/shimexe/commit/6085f4674df70635f106677dfa3289f70371f0d7))
* enhance release workflow with advanced cross-compilation ([e802a67](https://github.com/loonghao/shimexe/commit/e802a67dd88a8243fabd8bf51580cdf9e0c9e3f4))
* enhance release-plz validation with version update check ([ab0df95](https://github.com/loonghao/shimexe/commit/ab0df952d5893364597259209fde276b4000f9d6))
* enhance shim configuration with dynamic template system and improved args handling ([00bfd93](https://github.com/loonghao/shimexe/commit/00bfd93df15a3873032ec0b43dd22dea355d3049))
* enhance unit test coverage with comprehensive test suites ([03addc8](https://github.com/loonghao/shimexe/commit/03addc833391ec3f08973763e6dcc0da0ee8f1cc))
* implement standalone shim execution with local config lookup ([7bde37a](https://github.com/loonghao/shimexe/commit/7bde37a24629ac9e80863fdba838a4797e6b0637))
* improve code coverage and fix codecov configuration ([ced5396](https://github.com/loonghao/shimexe/commit/ced53964bd31d6f12dd1bbfa2e4e1f1c31fc311c))
* integrate automated package manager publishing ([a813bc4](https://github.com/loonghao/shimexe/commit/a813bc494a160e33abba5a22f310345af03150d2))
* integrate automated package manager publishing ([30d072b](https://github.com/loonghao/shimexe/commit/30d072bc20d09c013cb70b2e6d806e352f4700df))
* integrate turbo-cdn and restructure tests ([95de3fb](https://github.com/loonghao/shimexe/commit/95de3fbcd1f6fca4d57edcf21f216df7b7e978f2)), closes [#45](https://github.com/loonghao/shimexe/issues/45)
* migrate from release-plz to release-please ([4203fd4](https://github.com/loonghao/shimexe/commit/4203fd4dea50a5b387fa2a4e4f461ed645983a34))
* modernize GitHub Actions to latest versions ([f099150](https://github.com/loonghao/shimexe/commit/f09915002b94b2b3d19035a79bd0e0345a9b8c7f))
* optimize performance and fix workflow ([76a65e4](https://github.com/loonghao/shimexe/commit/76a65e4379f79fc5562b68d805026459f694f85b))
* setup release-please automation and add crate READMEs ([af4bd8a](https://github.com/loonghao/shimexe/commit/af4bd8a9da1b119d9bd016ca41f528871c129a39))
* simplify release workflow with actions-rust-release ([cec6df5](https://github.com/loonghao/shimexe/commit/cec6df5b46479fd858c07d1761daad5b7eb3485e))
* upgrade turbo-cdn to 0.4.3 and improve verbose logging control ([949a4f6](https://github.com/loonghao/shimexe/commit/949a4f661c57c715fab31ca71248f47289d3098f))
* **windows:** hide console window when running shims ([54a6830](https://github.com/loonghao/shimexe/commit/54a68302b39890aff3ce7e96a8a812b90e359200))


### Bug Fixes

* add git-token parameter to release-plz dry-run ([7d99467](https://github.com/loonghao/shimexe/commit/7d994675570235b904fd72fc8a801a3df9d6eba7))
* add retry logic and fallback for GitHub API rate limits ([ecd6958](https://github.com/loonghao/shimexe/commit/ecd6958007b7fe552917a0657bb60781941b7e43))
* add retry logic to update-packages workflow ([8b76325](https://github.com/loonghao/shimexe/commit/8b76325982a798acb4dbfe9c1dd4f43151088b81))
* add shimexe-cli to release-please configuration ([02e445d](https://github.com/loonghao/shimexe/commit/02e445d52c29150219dbd8f716f35dc2226f9131))
* add version specification for shimexe-core dependency to enable crates.io publishing ([f6ff396](https://github.com/loonghao/shimexe/commit/f6ff39687f70a9e7db54b1779010d24ac35ea9cc))
* adjust performance test timeouts for Windows environment ([d0c4bd3](https://github.com/loonghao/shimexe/commit/d0c4bd379cb06d8e494edc7011a46e350a89883d))
* **ci:** cleanup stale 'autorelease: pending' labels before running release-please ([fa5199c](https://github.com/loonghao/shimexe/commit/fa5199c8bf68f631d6c83b96a504e1870119a70a))
* **ci:** remove cargo-workspace plugin; rely on explicit packages mapping for root autobump ([0a61989](https://github.com/loonghao/shimexe/commit/0a619894410d7dacac47081a1a785efea69e9828))
* **ci:** set release-please PR title pattern to include scope/component/version ([8816990](https://github.com/loonghao/shimexe/commit/8816990c98c0411309ddc8bfef6c208c6ff1e6b5))
* clean working directory by removing tracked build files ([c86db29](https://github.com/loonghao/shimexe/commit/c86db296a85af0b144e2e1db08b044b50d5d3702))
* complete unit test coverage enhancement ([42041ae](https://github.com/loonghao/shimexe/commit/42041ae25a30c55c4f551315f5be48d7259e720f))
* completely disable codecov status checks to prevent CI failures ([015597f](https://github.com/loonghao/shimexe/commit/015597f5f476566f0ff2c50f5d1fb3bba781e4df))
* completely rewrite release-please config for Rust workspace ([353eb0c](https://github.com/loonghao/shimexe/commit/353eb0c527363bb21bf39d48760b50897b383d3c))
* comprehensive GitHub Actions permissions and CI configuration ([8aaaa40](https://github.com/loonghao/shimexe/commit/8aaaa40609ab5d7ba92650c52dc706005822c87a))
* configure release-plz to show only current version changelog in releases ([2472b5a](https://github.com/loonghao/shimexe/commit/2472b5a2f3a224d463a4a709e936a6680d4feba9))
* correct install script URLs to use raw.githubusercontent.com ([83275ee](https://github.com/loonghao/shimexe/commit/83275ee03436808250eeeb8afb35dada966088f4))
* correct PATH usage and improve user guidance ([291763e](https://github.com/loonghao/shimexe/commit/291763ee24a8218cf9224d004a1b3e0ab3c57e4c))
* correct release tag format in install scripts ([de4ac68](https://github.com/loonghao/shimexe/commit/de4ac683a490c9260489dbc894b4224570971e34))
* correct test implementations to match actual API ([cc35da6](https://github.com/loonghao/shimexe/commit/cc35da6f63a535c616dcfde54db22d0da5ba6535))
* **deps:** update rust crate tokio to v1.46.1 ([e423b26](https://github.com/loonghao/shimexe/commit/e423b2658a1f36e52459ae267c65091411d14ad5))
* **deps:** update rust crate tokio to v1.47.0 ([8e186b9](https://github.com/loonghao/shimexe/commit/8e186b94affd1dbbe7a5de7fe00785e6d34cffd0))
* **deps:** update rust crate zip to v4 ([f7223d5](https://github.com/loonghao/shimexe/commit/f7223d5216305757045da9fc72d2cd131cb74f07))
* **deps:** update rust crate zip to v4.2.0 ([87bd01f](https://github.com/loonghao/shimexe/commit/87bd01f96e206e4518ff48b4e80214c21f73da94))
* enable release workflow triggering with PAT token ([807e0e9](https://github.com/loonghao/shimexe/commit/807e0e92029f2881d7702d24a5da606de9e52561))
* improve release workflow to handle existing releases ([be6a75e](https://github.com/loonghao/shimexe/commit/be6a75e6815447a86e23cadd6b7c4089d65ec9a1))
* improve version detection and release changelog formatting ([7003e09](https://github.com/loonghao/shimexe/commit/7003e09484d4668b2ec7e45e0b472d1ea66a9e27))
* **info:** avoid compile-time env! for TARGET/RUSTC_VERSION; use runtime fallbacks ([6970011](https://github.com/loonghao/shimexe/commit/69700114defefaf4942502d19afab6634c64100b))
* make codecov informational only, don't fail CI ([cc8dace](https://github.com/loonghao/shimexe/commit/cc8dace27bb764957552a11216c4a80324ee7489))
* optimize release workflow for tag-triggered releases ([78a27ff](https://github.com/loonghao/shimexe/commit/78a27ff5dccdd6be228db68914d3d69cece73bf6))
* optimize release workflow trigger conditions ([749406c](https://github.com/loonghao/shimexe/commit/749406cc7c7a96d3fb2a0fa954a81a108d62f16d))
* optimize release-plz config for mixed workspace ([b466497](https://github.com/loonghao/shimexe/commit/b466497845c83f4075c4282439c527d08dace412))
* remove needless borrow in archive path handling ([874247a](https://github.com/loonghao/shimexe/commit/874247a8c5a9ec79c87bdcb056d3da9dc3ae5571))
* remove root directory from release-please configuration ([75ed065](https://github.com/loonghao/shimexe/commit/75ed06561bb97660e41f52040c885cb99585b6d2))
* remove unsupported update_files from release-plz config ([1d917c4](https://github.com/loonghao/shimexe/commit/1d917c47eb3ce60ec8a0cfcdd5c5c84deeb70228))
* replace winget with chocolatey for ImageMagick installation ([c3fd017](https://github.com/loonghao/shimexe/commit/c3fd017ec05583851baa94ae7a816ebababf1a59))
* resolve all compilation errors and template syntax issues ([82122f5](https://github.com/loonghao/shimexe/commit/82122f5161e6c253497a55604fe49563f57532a5))
* resolve CI configuration and testing issues ([8754f55](https://github.com/loonghao/shimexe/commit/8754f553a9ce9d601be7713ab02f0b9aa06c9628))
* resolve clippy and rustdoc warnings for CI compatibility ([a1acc7b](https://github.com/loonghao/shimexe/commit/a1acc7b51892c356d964ac21fe99bec6ecadc24f))
* resolve clippy warnings and compilation errors ([ee01ee3](https://github.com/loonghao/shimexe/commit/ee01ee3c161c9b41df0764e69f039eceb4f861a9))
* resolve clippy warnings and implement Default trait for ConfigCache ([e6641b3](https://github.com/loonghao/shimexe/commit/e6641b3a1a59e41e0f97454bfd32e2fc53f8bcc0))
* resolve clippy warnings in test files ([1d4196b](https://github.com/loonghao/shimexe/commit/1d4196bca81362410336254c97960b1bfc1d2183))
* resolve compilation errors in test files ([ebf65ec](https://github.com/loonghao/shimexe/commit/ebf65ecb6ab39a90f09364891c7ff5a547b67868))
* resolve compilation errors in tests ([2ea6753](https://github.com/loonghao/shimexe/commit/2ea6753c90dd36622c2b1306f19b5e93c683031c))
* resolve duplicate release issues and improve Chocolatey publishing ([f7286cb](https://github.com/loonghao/shimexe/commit/f7286cb18807fcabe7246494fa7d5fa821bd4cc9))
* resolve git detached HEAD issue in release-plz dry-run ([e09a7df](https://github.com/loonghao/shimexe/commit/e09a7df4fa74964faf9f4ba1bfca1136b2bffcae))
* resolve Git merge conflict markers in release-please manifest ([1a082cd](https://github.com/loonghao/shimexe/commit/1a082cdc8da7f2de63e0a45e6d33173fa25c82d4))
* resolve GitHub Actions permission issue for release-please ([bbcd794](https://github.com/loonghao/shimexe/commit/bbcd794f83cc5029b7c16e3d8539dbb5b38e3c48))
* resolve OpenSSL cross-compilation issues with rustls ([9c11fab](https://github.com/loonghao/shimexe/commit/9c11fabbb25906dddc34d2b71db4ad3c9386b502))
* resolve package manager publishing issues ([1f53f43](https://github.com/loonghao/shimexe/commit/1f53f43681e4ac6489098b3e895911b264d27759))
* resolve release-please configuration issues ([5875b0d](https://github.com/loonghao/shimexe/commit/5875b0d39d90e8c187916bc89eaf9712d8cd0bb8))
* resolve release-plz config and clippy warnings ([409d2e3](https://github.com/loonghao/shimexe/commit/409d2e3576424de45fd9f580d44e8ffcd90bd885))
* resolve release-plz configuration issues ([6955e44](https://github.com/loonghao/shimexe/commit/6955e4487ba81e533bfbd9654086dab215ead0fa))
* resolve remaining clippy uninlined format args warnings ([26266bb](https://github.com/loonghao/shimexe/commit/26266bb516907fab760f793b5103827e83e11f0d))
* resolve remaining redundant_closure warning in runner.rs ([e22b419](https://github.com/loonghao/shimexe/commit/e22b419b0bce0077bcf03a7033b6d61be472edd8))
* resolve TOML serialization test failure ([4a99cf7](https://github.com/loonghao/shimexe/commit/4a99cf7a05a6c1406beaa1a5d6307a3f9bccfd98))
* resolve utils test failures for environment variables ([cea8746](https://github.com/loonghao/shimexe/commit/cea874653aca5868a6cc8de62ee2f0bf65ed2390))
* resolve version detection issues in install scripts and update Scoop manifest ([054774a](https://github.com/loonghao/shimexe/commit/054774ae09a587ba22b1223c0465e0bc56415898))
* resolve workflow conflicts between release.yml and update-packages.yml ([5929ba6](https://github.com/loonghao/shimexe/commit/5929ba63fda95059409e1dd3e37bde94aa6b47c4))
* set explicit version for root package to resolve cargo-workspace issue ([900c6f7](https://github.com/loonghao/shimexe/commit/900c6f7e1df88a7cde90cae40ce10fa9afb526d0))
* set explicit version for shimexe-core to resolve cargo-workspace issue ([304e13b](https://github.com/loonghao/shimexe/commit/304e13b1a6b66153e7b538b3c3c665b1bcaf8c4b))
* support both v* and shimexe-v* tag formats in install scripts ([f8ea896](https://github.com/loonghao/shimexe/commit/f8ea89629000758d22e70b238d46e86f2dc2ce71))
* unify workspace versioning and disable component-in-tag to reduce duplicate PRs ([46d1f90](https://github.com/loonghao/shimexe/commit/46d1f90f9fef2bc1f0fcdcdad5308da2ead79d5f))
* update README_zh.md and fix install script file naming ([dcdc637](https://github.com/loonghao/shimexe/commit/dcdc6370ecbd389c86cecee4fb7b1caa2039b0bd))
* update release-please configuration to resolve workflow issues ([bfcceb3](https://github.com/loonghao/shimexe/commit/bfcceb3d4d4f54a21c1bc0ff68b1824e44b30036))
* update release-plz configuration and add dry-run checks ([07d3a92](https://github.com/loonghao/shimexe/commit/07d3a9244ec3d30f3c62f4cf4a8e18c29892ee2a))
* update release-plz configuration for standard tag format ([479177c](https://github.com/loonghao/shimexe/commit/479177c1484d21a552852fd3f23906fb615cb4cb))
* update workspace license to MIT only ([0558a7b](https://github.com/loonghao/shimexe/commit/0558a7ba54b1c32c6d4b52fb03a242aeba1f0308))
* use official release-plz GitHub Action ([3cd74d0](https://github.com/loonghao/shimexe/commit/3cd74d083cc194da7ddb96c77ed3bad2f53b0519))
* **workflows:** resolve YAML errors and move package managers update under release-assets; add actionlint; make coverage non-blocking ([9f72088](https://github.com/loonghao/shimexe/commit/9f72088fcf154a8a5db2cb48aa4d3d56726159b4))


### Performance Improvements

* **core:** remove unused file_modified from cache entry; trust cached value within TTL to avoid fs IO; fix -D dead-code ([5f5f77d](https://github.com/loonghao/shimexe/commit/5f5f77da7e9bf69d861c38cd1da705586b727c08))
* optimize CI/CD workflows and fix GitHub API auth ([0b2131b](https://github.com/loonghao/shimexe/commit/0b2131b3e37563860153a63fc6e5bbcff3839822))


### Code Refactoring

* move GitHub token validation to CI workflow ([6676aef](https://github.com/loonghao/shimexe/commit/6676aef8c8632af8343cc67db361e761a2650cf2))
* move release-plz dry-run to CI workflow ([bdb71a1](https://github.com/loonghao/shimexe/commit/bdb71a1427225c91104982227a01b3c726e14e0b))
* move shimexe-cli to root src directory ([d533ec6](https://github.com/loonghao/shimexe/commit/d533ec6535e9393cbe4ba949443fa2db54839f65))
* optimize dependency installation with unified approach ([8fde233](https://github.com/loonghao/shimexe/commit/8fde233d8d8dd966949efcc2b94d8cdd63fb0ba0))
* rename [@pkg](https://github.com/pkg) to pkg directory and fix code quality ([e3b0da8](https://github.com/loonghao/shimexe/commit/e3b0da89c2042d1ab60cace62352c96d327e830b))
* reorganize CI workflow responsibilities ([edeab8c](https://github.com/loonghao/shimexe/commit/edeab8c456e122ed71c177049d099c00df687f81))
* reorganize package manager files to [@pkg](https://github.com/pkg) directory ([4baa079](https://github.com/loonghao/shimexe/commit/4baa079fb660c96675d969d8da36404fd27a43b3))
* simplify PR validation to config-only check ([540333b](https://github.com/loonghao/shimexe/commit/540333b801218e2ceb6b049408a55cfc90bedcc3))


### Documentation

* update README and add comprehensive unit tests ([c35b70d](https://github.com/loonghao/shimexe/commit/c35b70df7102a0d9fdae9394e10ec8aa13967109))

## [0.5.12](https://github.com/loonghao/shimexe/compare/v0.5.11...v0.5.12) (2025-08-10)


### Features

* add --info flag to display detailed system and version information ([cfd86c1](https://github.com/loonghao/shimexe/commit/cfd86c18370653d984132e7d56cc05d2d4675062))
* add comprehensive archive support and package management ([48ce5c5](https://github.com/loonghao/shimexe/commit/48ce5c54b6f74c1d57fa1960cf7c962ab476dc40))
* add comprehensive vx integration example ([c088d97](https://github.com/loonghao/shimexe/commit/c088d972664de35fcbc8ec8b664f6d1544c6dfcf))
* add high-level ShimManager API and update Chinese README ([e98666d](https://github.com/loonghao/shimexe/commit/e98666dceb763aea2109766f5f5f10fdac00ccf1))
* add HTTP URL support for automatic executable download ([28abbc0](https://github.com/loonghao/shimexe/commit/28abbc0902fe0b68275cf3c5f87ea7288960698a))
* add multi-platform package management and enhanced badges ([d60371a](https://github.com/loonghao/shimexe/commit/d60371ab02985e3e87d2fab565cd708c147fa59b))
* add src directory monitoring to release-please ([5f1b065](https://github.com/loonghao/shimexe/commit/5f1b06519209ee44756ff6835482cf507f49cd90))
* disable clippy uninlined_format_args lint ([e2d69dd](https://github.com/loonghao/shimexe/commit/e2d69ddfa39690858863deac9a3b3570c8ffed66))
* enhance auto-update with turbo-cdn integration ([2a5e660](https://github.com/loonghao/shimexe/commit/2a5e6607a02fda18df9bd120cd1c054af14a61bb))
* enhance CI workflow with cross-compilation testing ([afb7a58](https://github.com/loonghao/shimexe/commit/afb7a583ef923f3211156f1b29377e5af024fa27))
* enhance HTTP URL support with persistent download tracking ([d169cb7](https://github.com/loonghao/shimexe/commit/d169cb752d916dc1824adb6287656d44c8d5c12c))
* enhance package publishing and fix env config ([332cad3](https://github.com/loonghao/shimexe/commit/332cad316beebd4af5dfd4cf6f3362ce5339779c))
* enhance README with logo and improve vx integration ([6085f46](https://github.com/loonghao/shimexe/commit/6085f4674df70635f106677dfa3289f70371f0d7))
* enhance release workflow with advanced cross-compilation ([e802a67](https://github.com/loonghao/shimexe/commit/e802a67dd88a8243fabd8bf51580cdf9e0c9e3f4))
* enhance release-plz validation with version update check ([ab0df95](https://github.com/loonghao/shimexe/commit/ab0df952d5893364597259209fde276b4000f9d6))
* enhance shim configuration with dynamic template system and improved args handling ([00bfd93](https://github.com/loonghao/shimexe/commit/00bfd93df15a3873032ec0b43dd22dea355d3049))
* enhance unit test coverage with comprehensive test suites ([03addc8](https://github.com/loonghao/shimexe/commit/03addc833391ec3f08973763e6dcc0da0ee8f1cc))
* implement standalone shim execution with local config lookup ([7bde37a](https://github.com/loonghao/shimexe/commit/7bde37a24629ac9e80863fdba838a4797e6b0637))
* improve code coverage and fix codecov configuration ([ced5396](https://github.com/loonghao/shimexe/commit/ced53964bd31d6f12dd1bbfa2e4e1f1c31fc311c))
* integrate automated package manager publishing ([a813bc4](https://github.com/loonghao/shimexe/commit/a813bc494a160e33abba5a22f310345af03150d2))
* integrate automated package manager publishing ([30d072b](https://github.com/loonghao/shimexe/commit/30d072bc20d09c013cb70b2e6d806e352f4700df))
* integrate turbo-cdn and restructure tests ([95de3fb](https://github.com/loonghao/shimexe/commit/95de3fbcd1f6fca4d57edcf21f216df7b7e978f2)), closes [#45](https://github.com/loonghao/shimexe/issues/45)
* migrate from release-plz to release-please ([4203fd4](https://github.com/loonghao/shimexe/commit/4203fd4dea50a5b387fa2a4e4f461ed645983a34))
* modernize GitHub Actions to latest versions ([f099150](https://github.com/loonghao/shimexe/commit/f09915002b94b2b3d19035a79bd0e0345a9b8c7f))
* optimize performance and fix workflow ([76a65e4](https://github.com/loonghao/shimexe/commit/76a65e4379f79fc5562b68d805026459f694f85b))
* setup release-please automation and add crate READMEs ([af4bd8a](https://github.com/loonghao/shimexe/commit/af4bd8a9da1b119d9bd016ca41f528871c129a39))
* simplify release workflow with actions-rust-release ([cec6df5](https://github.com/loonghao/shimexe/commit/cec6df5b46479fd858c07d1761daad5b7eb3485e))
* upgrade turbo-cdn to 0.4.3 and improve verbose logging control ([949a4f6](https://github.com/loonghao/shimexe/commit/949a4f661c57c715fab31ca71248f47289d3098f))
* **windows:** hide console window when running shims ([54a6830](https://github.com/loonghao/shimexe/commit/54a68302b39890aff3ce7e96a8a812b90e359200))


### Bug Fixes

* add git-token parameter to release-plz dry-run ([7d99467](https://github.com/loonghao/shimexe/commit/7d994675570235b904fd72fc8a801a3df9d6eba7))
* add retry logic and fallback for GitHub API rate limits ([ecd6958](https://github.com/loonghao/shimexe/commit/ecd6958007b7fe552917a0657bb60781941b7e43))
* add retry logic to update-packages workflow ([8b76325](https://github.com/loonghao/shimexe/commit/8b76325982a798acb4dbfe9c1dd4f43151088b81))
* add shimexe-cli to release-please configuration ([02e445d](https://github.com/loonghao/shimexe/commit/02e445d52c29150219dbd8f716f35dc2226f9131))
* add version specification for shimexe-core dependency to enable crates.io publishing ([f6ff396](https://github.com/loonghao/shimexe/commit/f6ff39687f70a9e7db54b1779010d24ac35ea9cc))
* adjust performance test timeouts for Windows environment ([d0c4bd3](https://github.com/loonghao/shimexe/commit/d0c4bd379cb06d8e494edc7011a46e350a89883d))
* **ci:** cleanup stale 'autorelease: pending' labels before running release-please ([fa5199c](https://github.com/loonghao/shimexe/commit/fa5199c8bf68f631d6c83b96a504e1870119a70a))
* **ci:** remove cargo-workspace plugin; rely on explicit packages mapping for root autobump ([0a61989](https://github.com/loonghao/shimexe/commit/0a619894410d7dacac47081a1a785efea69e9828))
* **ci:** set release-please PR title pattern to include scope/component/version ([8816990](https://github.com/loonghao/shimexe/commit/8816990c98c0411309ddc8bfef6c208c6ff1e6b5))
* clean working directory by removing tracked build files ([c86db29](https://github.com/loonghao/shimexe/commit/c86db296a85af0b144e2e1db08b044b50d5d3702))
* complete unit test coverage enhancement ([42041ae](https://github.com/loonghao/shimexe/commit/42041ae25a30c55c4f551315f5be48d7259e720f))
* completely disable codecov status checks to prevent CI failures ([015597f](https://github.com/loonghao/shimexe/commit/015597f5f476566f0ff2c50f5d1fb3bba781e4df))
* completely rewrite release-please config for Rust workspace ([353eb0c](https://github.com/loonghao/shimexe/commit/353eb0c527363bb21bf39d48760b50897b383d3c))
* comprehensive GitHub Actions permissions and CI configuration ([8aaaa40](https://github.com/loonghao/shimexe/commit/8aaaa40609ab5d7ba92650c52dc706005822c87a))
* configure release-plz to show only current version changelog in releases ([2472b5a](https://github.com/loonghao/shimexe/commit/2472b5a2f3a224d463a4a709e936a6680d4feba9))
* correct install script URLs to use raw.githubusercontent.com ([83275ee](https://github.com/loonghao/shimexe/commit/83275ee03436808250eeeb8afb35dada966088f4))
* correct PATH usage and improve user guidance ([291763e](https://github.com/loonghao/shimexe/commit/291763ee24a8218cf9224d004a1b3e0ab3c57e4c))
* correct release tag format in install scripts ([de4ac68](https://github.com/loonghao/shimexe/commit/de4ac683a490c9260489dbc894b4224570971e34))
* correct test implementations to match actual API ([cc35da6](https://github.com/loonghao/shimexe/commit/cc35da6f63a535c616dcfde54db22d0da5ba6535))
* **deps:** update rust crate tokio to v1.46.1 ([e423b26](https://github.com/loonghao/shimexe/commit/e423b2658a1f36e52459ae267c65091411d14ad5))
* **deps:** update rust crate tokio to v1.47.0 ([8e186b9](https://github.com/loonghao/shimexe/commit/8e186b94affd1dbbe7a5de7fe00785e6d34cffd0))
* **deps:** update rust crate zip to v4 ([f7223d5](https://github.com/loonghao/shimexe/commit/f7223d5216305757045da9fc72d2cd131cb74f07))
* **deps:** update rust crate zip to v4.2.0 ([87bd01f](https://github.com/loonghao/shimexe/commit/87bd01f96e206e4518ff48b4e80214c21f73da94))
* enable release workflow triggering with PAT token ([807e0e9](https://github.com/loonghao/shimexe/commit/807e0e92029f2881d7702d24a5da606de9e52561))
* improve release workflow to handle existing releases ([be6a75e](https://github.com/loonghao/shimexe/commit/be6a75e6815447a86e23cadd6b7c4089d65ec9a1))
* improve version detection and release changelog formatting ([7003e09](https://github.com/loonghao/shimexe/commit/7003e09484d4668b2ec7e45e0b472d1ea66a9e27))
* **info:** avoid compile-time env! for TARGET/RUSTC_VERSION; use runtime fallbacks ([6970011](https://github.com/loonghao/shimexe/commit/69700114defefaf4942502d19afab6634c64100b))
* make codecov informational only, don't fail CI ([cc8dace](https://github.com/loonghao/shimexe/commit/cc8dace27bb764957552a11216c4a80324ee7489))
* optimize release workflow for tag-triggered releases ([78a27ff](https://github.com/loonghao/shimexe/commit/78a27ff5dccdd6be228db68914d3d69cece73bf6))
* optimize release workflow trigger conditions ([749406c](https://github.com/loonghao/shimexe/commit/749406cc7c7a96d3fb2a0fa954a81a108d62f16d))
* optimize release-plz config for mixed workspace ([b466497](https://github.com/loonghao/shimexe/commit/b466497845c83f4075c4282439c527d08dace412))
* remove needless borrow in archive path handling ([874247a](https://github.com/loonghao/shimexe/commit/874247a8c5a9ec79c87bdcb056d3da9dc3ae5571))
* remove root directory from release-please configuration ([75ed065](https://github.com/loonghao/shimexe/commit/75ed06561bb97660e41f52040c885cb99585b6d2))
* remove unsupported update_files from release-plz config ([1d917c4](https://github.com/loonghao/shimexe/commit/1d917c47eb3ce60ec8a0cfcdd5c5c84deeb70228))
* replace winget with chocolatey for ImageMagick installation ([c3fd017](https://github.com/loonghao/shimexe/commit/c3fd017ec05583851baa94ae7a816ebababf1a59))
* resolve all compilation errors and template syntax issues ([82122f5](https://github.com/loonghao/shimexe/commit/82122f5161e6c253497a55604fe49563f57532a5))
* resolve CI configuration and testing issues ([8754f55](https://github.com/loonghao/shimexe/commit/8754f553a9ce9d601be7713ab02f0b9aa06c9628))
* resolve clippy and rustdoc warnings for CI compatibility ([a1acc7b](https://github.com/loonghao/shimexe/commit/a1acc7b51892c356d964ac21fe99bec6ecadc24f))
* resolve clippy warnings and compilation errors ([ee01ee3](https://github.com/loonghao/shimexe/commit/ee01ee3c161c9b41df0764e69f039eceb4f861a9))
* resolve clippy warnings and implement Default trait for ConfigCache ([e6641b3](https://github.com/loonghao/shimexe/commit/e6641b3a1a59e41e0f97454bfd32e2fc53f8bcc0))
* resolve clippy warnings in test files ([1d4196b](https://github.com/loonghao/shimexe/commit/1d4196bca81362410336254c97960b1bfc1d2183))
* resolve compilation errors in test files ([ebf65ec](https://github.com/loonghao/shimexe/commit/ebf65ecb6ab39a90f09364891c7ff5a547b67868))
* resolve compilation errors in tests ([2ea6753](https://github.com/loonghao/shimexe/commit/2ea6753c90dd36622c2b1306f19b5e93c683031c))
* resolve duplicate release issues and improve Chocolatey publishing ([f7286cb](https://github.com/loonghao/shimexe/commit/f7286cb18807fcabe7246494fa7d5fa821bd4cc9))
* resolve git detached HEAD issue in release-plz dry-run ([e09a7df](https://github.com/loonghao/shimexe/commit/e09a7df4fa74964faf9f4ba1bfca1136b2bffcae))
* resolve Git merge conflict markers in release-please manifest ([1a082cd](https://github.com/loonghao/shimexe/commit/1a082cdc8da7f2de63e0a45e6d33173fa25c82d4))
* resolve GitHub Actions permission issue for release-please ([bbcd794](https://github.com/loonghao/shimexe/commit/bbcd794f83cc5029b7c16e3d8539dbb5b38e3c48))
* resolve OpenSSL cross-compilation issues with rustls ([9c11fab](https://github.com/loonghao/shimexe/commit/9c11fabbb25906dddc34d2b71db4ad3c9386b502))
* resolve package manager publishing issues ([1f53f43](https://github.com/loonghao/shimexe/commit/1f53f43681e4ac6489098b3e895911b264d27759))
* resolve release-please configuration issues ([5875b0d](https://github.com/loonghao/shimexe/commit/5875b0d39d90e8c187916bc89eaf9712d8cd0bb8))
* resolve release-plz config and clippy warnings ([409d2e3](https://github.com/loonghao/shimexe/commit/409d2e3576424de45fd9f580d44e8ffcd90bd885))
* resolve release-plz configuration issues ([6955e44](https://github.com/loonghao/shimexe/commit/6955e4487ba81e533bfbd9654086dab215ead0fa))
* resolve remaining clippy uninlined format args warnings ([26266bb](https://github.com/loonghao/shimexe/commit/26266bb516907fab760f793b5103827e83e11f0d))
* resolve remaining redundant_closure warning in runner.rs ([e22b419](https://github.com/loonghao/shimexe/commit/e22b419b0bce0077bcf03a7033b6d61be472edd8))
* resolve TOML serialization test failure ([4a99cf7](https://github.com/loonghao/shimexe/commit/4a99cf7a05a6c1406beaa1a5d6307a3f9bccfd98))
* resolve utils test failures for environment variables ([cea8746](https://github.com/loonghao/shimexe/commit/cea874653aca5868a6cc8de62ee2f0bf65ed2390))
* resolve version detection issues in install scripts and update Scoop manifest ([054774a](https://github.com/loonghao/shimexe/commit/054774ae09a587ba22b1223c0465e0bc56415898))
* resolve workflow conflicts between release.yml and update-packages.yml ([5929ba6](https://github.com/loonghao/shimexe/commit/5929ba63fda95059409e1dd3e37bde94aa6b47c4))
* set explicit version for root package to resolve cargo-workspace issue ([900c6f7](https://github.com/loonghao/shimexe/commit/900c6f7e1df88a7cde90cae40ce10fa9afb526d0))
* set explicit version for shimexe-core to resolve cargo-workspace issue ([304e13b](https://github.com/loonghao/shimexe/commit/304e13b1a6b66153e7b538b3c3c665b1bcaf8c4b))
* support both v* and shimexe-v* tag formats in install scripts ([f8ea896](https://github.com/loonghao/shimexe/commit/f8ea89629000758d22e70b238d46e86f2dc2ce71))
* unify workspace versioning and disable component-in-tag to reduce duplicate PRs ([46d1f90](https://github.com/loonghao/shimexe/commit/46d1f90f9fef2bc1f0fcdcdad5308da2ead79d5f))
* update README_zh.md and fix install script file naming ([dcdc637](https://github.com/loonghao/shimexe/commit/dcdc6370ecbd389c86cecee4fb7b1caa2039b0bd))
* update release-please configuration to resolve workflow issues ([bfcceb3](https://github.com/loonghao/shimexe/commit/bfcceb3d4d4f54a21c1bc0ff68b1824e44b30036))
* update release-plz configuration and add dry-run checks ([07d3a92](https://github.com/loonghao/shimexe/commit/07d3a9244ec3d30f3c62f4cf4a8e18c29892ee2a))
* update release-plz configuration for standard tag format ([479177c](https://github.com/loonghao/shimexe/commit/479177c1484d21a552852fd3f23906fb615cb4cb))
* update workspace license to MIT only ([0558a7b](https://github.com/loonghao/shimexe/commit/0558a7ba54b1c32c6d4b52fb03a242aeba1f0308))
* use official release-plz GitHub Action ([3cd74d0](https://github.com/loonghao/shimexe/commit/3cd74d083cc194da7ddb96c77ed3bad2f53b0519))
* **workflows:** resolve YAML errors and move package managers update under release-assets; add actionlint; make coverage non-blocking ([9f72088](https://github.com/loonghao/shimexe/commit/9f72088fcf154a8a5db2cb48aa4d3d56726159b4))


### Performance Improvements

* **core:** remove unused file_modified from cache entry; trust cached value within TTL to avoid fs IO; fix -D dead-code ([5f5f77d](https://github.com/loonghao/shimexe/commit/5f5f77da7e9bf69d861c38cd1da705586b727c08))
* optimize CI/CD workflows and fix GitHub API auth ([0b2131b](https://github.com/loonghao/shimexe/commit/0b2131b3e37563860153a63fc6e5bbcff3839822))


### Code Refactoring

* move GitHub token validation to CI workflow ([6676aef](https://github.com/loonghao/shimexe/commit/6676aef8c8632af8343cc67db361e761a2650cf2))
* move release-plz dry-run to CI workflow ([bdb71a1](https://github.com/loonghao/shimexe/commit/bdb71a1427225c91104982227a01b3c726e14e0b))
* move shimexe-cli to root src directory ([d533ec6](https://github.com/loonghao/shimexe/commit/d533ec6535e9393cbe4ba949443fa2db54839f65))
* optimize dependency installation with unified approach ([8fde233](https://github.com/loonghao/shimexe/commit/8fde233d8d8dd966949efcc2b94d8cdd63fb0ba0))
* rename [@pkg](https://github.com/pkg) to pkg directory and fix code quality ([e3b0da8](https://github.com/loonghao/shimexe/commit/e3b0da89c2042d1ab60cace62352c96d327e830b))
* reorganize CI workflow responsibilities ([edeab8c](https://github.com/loonghao/shimexe/commit/edeab8c456e122ed71c177049d099c00df687f81))
* reorganize package manager files to [@pkg](https://github.com/pkg) directory ([4baa079](https://github.com/loonghao/shimexe/commit/4baa079fb660c96675d969d8da36404fd27a43b3))
* simplify PR validation to config-only check ([540333b](https://github.com/loonghao/shimexe/commit/540333b801218e2ceb6b049408a55cfc90bedcc3))


### Documentation

* update README and add comprehensive unit tests ([c35b70d](https://github.com/loonghao/shimexe/commit/c35b70df7102a0d9fdae9394e10ec8aa13967109))

## [0.5.11](https://github.com/loonghao/shimexe/compare/v0.5.10...v0.5.11) (2025-08-10)


### Features

* add --info flag to display detailed system and version information ([cfd86c1](https://github.com/loonghao/shimexe/commit/cfd86c18370653d984132e7d56cc05d2d4675062))
* add comprehensive archive support and package management ([48ce5c5](https://github.com/loonghao/shimexe/commit/48ce5c54b6f74c1d57fa1960cf7c962ab476dc40))
* add comprehensive vx integration example ([c088d97](https://github.com/loonghao/shimexe/commit/c088d972664de35fcbc8ec8b664f6d1544c6dfcf))
* add high-level ShimManager API and update Chinese README ([e98666d](https://github.com/loonghao/shimexe/commit/e98666dceb763aea2109766f5f5f10fdac00ccf1))
* add HTTP URL support for automatic executable download ([28abbc0](https://github.com/loonghao/shimexe/commit/28abbc0902fe0b68275cf3c5f87ea7288960698a))
* add multi-platform package management and enhanced badges ([d60371a](https://github.com/loonghao/shimexe/commit/d60371ab02985e3e87d2fab565cd708c147fa59b))
* add src directory monitoring to release-please ([5f1b065](https://github.com/loonghao/shimexe/commit/5f1b06519209ee44756ff6835482cf507f49cd90))
* disable clippy uninlined_format_args lint ([e2d69dd](https://github.com/loonghao/shimexe/commit/e2d69ddfa39690858863deac9a3b3570c8ffed66))
* enhance auto-update with turbo-cdn integration ([2a5e660](https://github.com/loonghao/shimexe/commit/2a5e6607a02fda18df9bd120cd1c054af14a61bb))
* enhance CI workflow with cross-compilation testing ([afb7a58](https://github.com/loonghao/shimexe/commit/afb7a583ef923f3211156f1b29377e5af024fa27))
* enhance HTTP URL support with persistent download tracking ([d169cb7](https://github.com/loonghao/shimexe/commit/d169cb752d916dc1824adb6287656d44c8d5c12c))
* enhance package publishing and fix env config ([332cad3](https://github.com/loonghao/shimexe/commit/332cad316beebd4af5dfd4cf6f3362ce5339779c))
* enhance README with logo and improve vx integration ([6085f46](https://github.com/loonghao/shimexe/commit/6085f4674df70635f106677dfa3289f70371f0d7))
* enhance release workflow with advanced cross-compilation ([e802a67](https://github.com/loonghao/shimexe/commit/e802a67dd88a8243fabd8bf51580cdf9e0c9e3f4))
* enhance release-plz validation with version update check ([ab0df95](https://github.com/loonghao/shimexe/commit/ab0df952d5893364597259209fde276b4000f9d6))
* enhance shim configuration with dynamic template system and improved args handling ([00bfd93](https://github.com/loonghao/shimexe/commit/00bfd93df15a3873032ec0b43dd22dea355d3049))
* enhance unit test coverage with comprehensive test suites ([03addc8](https://github.com/loonghao/shimexe/commit/03addc833391ec3f08973763e6dcc0da0ee8f1cc))
* implement standalone shim execution with local config lookup ([7bde37a](https://github.com/loonghao/shimexe/commit/7bde37a24629ac9e80863fdba838a4797e6b0637))
* improve code coverage and fix codecov configuration ([ced5396](https://github.com/loonghao/shimexe/commit/ced53964bd31d6f12dd1bbfa2e4e1f1c31fc311c))
* integrate automated package manager publishing ([a813bc4](https://github.com/loonghao/shimexe/commit/a813bc494a160e33abba5a22f310345af03150d2))
* integrate automated package manager publishing ([30d072b](https://github.com/loonghao/shimexe/commit/30d072bc20d09c013cb70b2e6d806e352f4700df))
* integrate turbo-cdn and restructure tests ([95de3fb](https://github.com/loonghao/shimexe/commit/95de3fbcd1f6fca4d57edcf21f216df7b7e978f2)), closes [#45](https://github.com/loonghao/shimexe/issues/45)
* migrate from release-plz to release-please ([4203fd4](https://github.com/loonghao/shimexe/commit/4203fd4dea50a5b387fa2a4e4f461ed645983a34))
* modernize GitHub Actions to latest versions ([f099150](https://github.com/loonghao/shimexe/commit/f09915002b94b2b3d19035a79bd0e0345a9b8c7f))
* optimize performance and fix workflow ([76a65e4](https://github.com/loonghao/shimexe/commit/76a65e4379f79fc5562b68d805026459f694f85b))
* setup release-please automation and add crate READMEs ([af4bd8a](https://github.com/loonghao/shimexe/commit/af4bd8a9da1b119d9bd016ca41f528871c129a39))
* simplify release workflow with actions-rust-release ([cec6df5](https://github.com/loonghao/shimexe/commit/cec6df5b46479fd858c07d1761daad5b7eb3485e))
* upgrade turbo-cdn to 0.4.3 and improve verbose logging control ([949a4f6](https://github.com/loonghao/shimexe/commit/949a4f661c57c715fab31ca71248f47289d3098f))
* **windows:** hide console window when running shims ([54a6830](https://github.com/loonghao/shimexe/commit/54a68302b39890aff3ce7e96a8a812b90e359200))


### Bug Fixes

* add git-token parameter to release-plz dry-run ([7d99467](https://github.com/loonghao/shimexe/commit/7d994675570235b904fd72fc8a801a3df9d6eba7))
* add retry logic and fallback for GitHub API rate limits ([ecd6958](https://github.com/loonghao/shimexe/commit/ecd6958007b7fe552917a0657bb60781941b7e43))
* add retry logic to update-packages workflow ([8b76325](https://github.com/loonghao/shimexe/commit/8b76325982a798acb4dbfe9c1dd4f43151088b81))
* add shimexe-cli to release-please configuration ([02e445d](https://github.com/loonghao/shimexe/commit/02e445d52c29150219dbd8f716f35dc2226f9131))
* add version specification for shimexe-core dependency to enable crates.io publishing ([f6ff396](https://github.com/loonghao/shimexe/commit/f6ff39687f70a9e7db54b1779010d24ac35ea9cc))
* adjust performance test timeouts for Windows environment ([d0c4bd3](https://github.com/loonghao/shimexe/commit/d0c4bd379cb06d8e494edc7011a46e350a89883d))
* **ci:** cleanup stale 'autorelease: pending' labels before running release-please ([fa5199c](https://github.com/loonghao/shimexe/commit/fa5199c8bf68f631d6c83b96a504e1870119a70a))
* **ci:** remove cargo-workspace plugin; rely on explicit packages mapping for root autobump ([0a61989](https://github.com/loonghao/shimexe/commit/0a619894410d7dacac47081a1a785efea69e9828))
* **ci:** set release-please PR title pattern to include scope/component/version ([8816990](https://github.com/loonghao/shimexe/commit/8816990c98c0411309ddc8bfef6c208c6ff1e6b5))
* clean working directory by removing tracked build files ([c86db29](https://github.com/loonghao/shimexe/commit/c86db296a85af0b144e2e1db08b044b50d5d3702))
* complete unit test coverage enhancement ([42041ae](https://github.com/loonghao/shimexe/commit/42041ae25a30c55c4f551315f5be48d7259e720f))
* completely disable codecov status checks to prevent CI failures ([015597f](https://github.com/loonghao/shimexe/commit/015597f5f476566f0ff2c50f5d1fb3bba781e4df))
* completely rewrite release-please config for Rust workspace ([353eb0c](https://github.com/loonghao/shimexe/commit/353eb0c527363bb21bf39d48760b50897b383d3c))
* comprehensive GitHub Actions permissions and CI configuration ([8aaaa40](https://github.com/loonghao/shimexe/commit/8aaaa40609ab5d7ba92650c52dc706005822c87a))
* configure release-plz to show only current version changelog in releases ([2472b5a](https://github.com/loonghao/shimexe/commit/2472b5a2f3a224d463a4a709e936a6680d4feba9))
* correct install script URLs to use raw.githubusercontent.com ([83275ee](https://github.com/loonghao/shimexe/commit/83275ee03436808250eeeb8afb35dada966088f4))
* correct PATH usage and improve user guidance ([291763e](https://github.com/loonghao/shimexe/commit/291763ee24a8218cf9224d004a1b3e0ab3c57e4c))
* correct release tag format in install scripts ([de4ac68](https://github.com/loonghao/shimexe/commit/de4ac683a490c9260489dbc894b4224570971e34))
* correct test implementations to match actual API ([cc35da6](https://github.com/loonghao/shimexe/commit/cc35da6f63a535c616dcfde54db22d0da5ba6535))
* **deps:** update rust crate tokio to v1.46.1 ([e423b26](https://github.com/loonghao/shimexe/commit/e423b2658a1f36e52459ae267c65091411d14ad5))
* **deps:** update rust crate tokio to v1.47.0 ([8e186b9](https://github.com/loonghao/shimexe/commit/8e186b94affd1dbbe7a5de7fe00785e6d34cffd0))
* **deps:** update rust crate zip to v4 ([f7223d5](https://github.com/loonghao/shimexe/commit/f7223d5216305757045da9fc72d2cd131cb74f07))
* **deps:** update rust crate zip to v4.2.0 ([87bd01f](https://github.com/loonghao/shimexe/commit/87bd01f96e206e4518ff48b4e80214c21f73da94))
* enable release workflow triggering with PAT token ([807e0e9](https://github.com/loonghao/shimexe/commit/807e0e92029f2881d7702d24a5da606de9e52561))
* improve release workflow to handle existing releases ([be6a75e](https://github.com/loonghao/shimexe/commit/be6a75e6815447a86e23cadd6b7c4089d65ec9a1))
* improve version detection and release changelog formatting ([7003e09](https://github.com/loonghao/shimexe/commit/7003e09484d4668b2ec7e45e0b472d1ea66a9e27))
* **info:** avoid compile-time env! for TARGET/RUSTC_VERSION; use runtime fallbacks ([6970011](https://github.com/loonghao/shimexe/commit/69700114defefaf4942502d19afab6634c64100b))
* make codecov informational only, don't fail CI ([cc8dace](https://github.com/loonghao/shimexe/commit/cc8dace27bb764957552a11216c4a80324ee7489))
* optimize release workflow for tag-triggered releases ([78a27ff](https://github.com/loonghao/shimexe/commit/78a27ff5dccdd6be228db68914d3d69cece73bf6))
* optimize release workflow trigger conditions ([749406c](https://github.com/loonghao/shimexe/commit/749406cc7c7a96d3fb2a0fa954a81a108d62f16d))
* optimize release-plz config for mixed workspace ([b466497](https://github.com/loonghao/shimexe/commit/b466497845c83f4075c4282439c527d08dace412))
* remove needless borrow in archive path handling ([874247a](https://github.com/loonghao/shimexe/commit/874247a8c5a9ec79c87bdcb056d3da9dc3ae5571))
* remove root directory from release-please configuration ([75ed065](https://github.com/loonghao/shimexe/commit/75ed06561bb97660e41f52040c885cb99585b6d2))
* remove unsupported update_files from release-plz config ([1d917c4](https://github.com/loonghao/shimexe/commit/1d917c47eb3ce60ec8a0cfcdd5c5c84deeb70228))
* replace winget with chocolatey for ImageMagick installation ([c3fd017](https://github.com/loonghao/shimexe/commit/c3fd017ec05583851baa94ae7a816ebababf1a59))
* resolve all compilation errors and template syntax issues ([82122f5](https://github.com/loonghao/shimexe/commit/82122f5161e6c253497a55604fe49563f57532a5))
* resolve CI configuration and testing issues ([8754f55](https://github.com/loonghao/shimexe/commit/8754f553a9ce9d601be7713ab02f0b9aa06c9628))
* resolve clippy and rustdoc warnings for CI compatibility ([a1acc7b](https://github.com/loonghao/shimexe/commit/a1acc7b51892c356d964ac21fe99bec6ecadc24f))
* resolve clippy warnings and compilation errors ([ee01ee3](https://github.com/loonghao/shimexe/commit/ee01ee3c161c9b41df0764e69f039eceb4f861a9))
* resolve clippy warnings and implement Default trait for ConfigCache ([e6641b3](https://github.com/loonghao/shimexe/commit/e6641b3a1a59e41e0f97454bfd32e2fc53f8bcc0))
* resolve clippy warnings in test files ([1d4196b](https://github.com/loonghao/shimexe/commit/1d4196bca81362410336254c97960b1bfc1d2183))
* resolve compilation errors in test files ([ebf65ec](https://github.com/loonghao/shimexe/commit/ebf65ecb6ab39a90f09364891c7ff5a547b67868))
* resolve compilation errors in tests ([2ea6753](https://github.com/loonghao/shimexe/commit/2ea6753c90dd36622c2b1306f19b5e93c683031c))
* resolve duplicate release issues and improve Chocolatey publishing ([f7286cb](https://github.com/loonghao/shimexe/commit/f7286cb18807fcabe7246494fa7d5fa821bd4cc9))
* resolve git detached HEAD issue in release-plz dry-run ([e09a7df](https://github.com/loonghao/shimexe/commit/e09a7df4fa74964faf9f4ba1bfca1136b2bffcae))
* resolve Git merge conflict markers in release-please manifest ([1a082cd](https://github.com/loonghao/shimexe/commit/1a082cdc8da7f2de63e0a45e6d33173fa25c82d4))
* resolve GitHub Actions permission issue for release-please ([bbcd794](https://github.com/loonghao/shimexe/commit/bbcd794f83cc5029b7c16e3d8539dbb5b38e3c48))
* resolve OpenSSL cross-compilation issues with rustls ([9c11fab](https://github.com/loonghao/shimexe/commit/9c11fabbb25906dddc34d2b71db4ad3c9386b502))
* resolve package manager publishing issues ([1f53f43](https://github.com/loonghao/shimexe/commit/1f53f43681e4ac6489098b3e895911b264d27759))
* resolve release-please configuration issues ([5875b0d](https://github.com/loonghao/shimexe/commit/5875b0d39d90e8c187916bc89eaf9712d8cd0bb8))
* resolve release-plz config and clippy warnings ([409d2e3](https://github.com/loonghao/shimexe/commit/409d2e3576424de45fd9f580d44e8ffcd90bd885))
* resolve release-plz configuration issues ([6955e44](https://github.com/loonghao/shimexe/commit/6955e4487ba81e533bfbd9654086dab215ead0fa))
* resolve remaining clippy uninlined format args warnings ([26266bb](https://github.com/loonghao/shimexe/commit/26266bb516907fab760f793b5103827e83e11f0d))
* resolve remaining redundant_closure warning in runner.rs ([e22b419](https://github.com/loonghao/shimexe/commit/e22b419b0bce0077bcf03a7033b6d61be472edd8))
* resolve TOML serialization test failure ([4a99cf7](https://github.com/loonghao/shimexe/commit/4a99cf7a05a6c1406beaa1a5d6307a3f9bccfd98))
* resolve utils test failures for environment variables ([cea8746](https://github.com/loonghao/shimexe/commit/cea874653aca5868a6cc8de62ee2f0bf65ed2390))
* resolve version detection issues in install scripts and update Scoop manifest ([054774a](https://github.com/loonghao/shimexe/commit/054774ae09a587ba22b1223c0465e0bc56415898))
* resolve workflow conflicts between release.yml and update-packages.yml ([5929ba6](https://github.com/loonghao/shimexe/commit/5929ba63fda95059409e1dd3e37bde94aa6b47c4))
* set explicit version for root package to resolve cargo-workspace issue ([900c6f7](https://github.com/loonghao/shimexe/commit/900c6f7e1df88a7cde90cae40ce10fa9afb526d0))
* set explicit version for shimexe-core to resolve cargo-workspace issue ([304e13b](https://github.com/loonghao/shimexe/commit/304e13b1a6b66153e7b538b3c3c665b1bcaf8c4b))
* support both v* and shimexe-v* tag formats in install scripts ([f8ea896](https://github.com/loonghao/shimexe/commit/f8ea89629000758d22e70b238d46e86f2dc2ce71))
* unify workspace versioning and disable component-in-tag to reduce duplicate PRs ([46d1f90](https://github.com/loonghao/shimexe/commit/46d1f90f9fef2bc1f0fcdcdad5308da2ead79d5f))
* update README_zh.md and fix install script file naming ([dcdc637](https://github.com/loonghao/shimexe/commit/dcdc6370ecbd389c86cecee4fb7b1caa2039b0bd))
* update release-please configuration to resolve workflow issues ([bfcceb3](https://github.com/loonghao/shimexe/commit/bfcceb3d4d4f54a21c1bc0ff68b1824e44b30036))
* update release-plz configuration and add dry-run checks ([07d3a92](https://github.com/loonghao/shimexe/commit/07d3a9244ec3d30f3c62f4cf4a8e18c29892ee2a))
* update release-plz configuration for standard tag format ([479177c](https://github.com/loonghao/shimexe/commit/479177c1484d21a552852fd3f23906fb615cb4cb))
* update workspace license to MIT only ([0558a7b](https://github.com/loonghao/shimexe/commit/0558a7ba54b1c32c6d4b52fb03a242aeba1f0308))
* use official release-plz GitHub Action ([3cd74d0](https://github.com/loonghao/shimexe/commit/3cd74d083cc194da7ddb96c77ed3bad2f53b0519))
* **workflows:** resolve YAML errors and move package managers update under release-assets; add actionlint; make coverage non-blocking ([9f72088](https://github.com/loonghao/shimexe/commit/9f72088fcf154a8a5db2cb48aa4d3d56726159b4))


### Performance Improvements

* **core:** remove unused file_modified from cache entry; trust cached value within TTL to avoid fs IO; fix -D dead-code ([5f5f77d](https://github.com/loonghao/shimexe/commit/5f5f77da7e9bf69d861c38cd1da705586b727c08))
* optimize CI/CD workflows and fix GitHub API auth ([0b2131b](https://github.com/loonghao/shimexe/commit/0b2131b3e37563860153a63fc6e5bbcff3839822))


### Code Refactoring

* move GitHub token validation to CI workflow ([6676aef](https://github.com/loonghao/shimexe/commit/6676aef8c8632af8343cc67db361e761a2650cf2))
* move release-plz dry-run to CI workflow ([bdb71a1](https://github.com/loonghao/shimexe/commit/bdb71a1427225c91104982227a01b3c726e14e0b))
* move shimexe-cli to root src directory ([d533ec6](https://github.com/loonghao/shimexe/commit/d533ec6535e9393cbe4ba949443fa2db54839f65))
* optimize dependency installation with unified approach ([8fde233](https://github.com/loonghao/shimexe/commit/8fde233d8d8dd966949efcc2b94d8cdd63fb0ba0))
* rename [@pkg](https://github.com/pkg) to pkg directory and fix code quality ([e3b0da8](https://github.com/loonghao/shimexe/commit/e3b0da89c2042d1ab60cace62352c96d327e830b))
* reorganize CI workflow responsibilities ([edeab8c](https://github.com/loonghao/shimexe/commit/edeab8c456e122ed71c177049d099c00df687f81))
* reorganize package manager files to [@pkg](https://github.com/pkg) directory ([4baa079](https://github.com/loonghao/shimexe/commit/4baa079fb660c96675d969d8da36404fd27a43b3))
* simplify PR validation to config-only check ([540333b](https://github.com/loonghao/shimexe/commit/540333b801218e2ceb6b049408a55cfc90bedcc3))


### Documentation

* update README and add comprehensive unit tests ([c35b70d](https://github.com/loonghao/shimexe/commit/c35b70df7102a0d9fdae9394e10ec8aa13967109))

## [0.5.10](https://github.com/loonghao/shimexe/compare/shimexe-v0.5.9...shimexe-v0.5.10) (2025-08-10)


### Features

* add --info flag to display detailed system and version information ([cfd86c1](https://github.com/loonghao/shimexe/commit/cfd86c18370653d984132e7d56cc05d2d4675062))
* add comprehensive archive support and package management ([48ce5c5](https://github.com/loonghao/shimexe/commit/48ce5c54b6f74c1d57fa1960cf7c962ab476dc40))
* add comprehensive vx integration example ([c088d97](https://github.com/loonghao/shimexe/commit/c088d972664de35fcbc8ec8b664f6d1544c6dfcf))
* add high-level ShimManager API and update Chinese README ([e98666d](https://github.com/loonghao/shimexe/commit/e98666dceb763aea2109766f5f5f10fdac00ccf1))
* add HTTP URL support for automatic executable download ([28abbc0](https://github.com/loonghao/shimexe/commit/28abbc0902fe0b68275cf3c5f87ea7288960698a))
* add multi-platform package management and enhanced badges ([d60371a](https://github.com/loonghao/shimexe/commit/d60371ab02985e3e87d2fab565cd708c147fa59b))
* add src directory monitoring to release-please ([5f1b065](https://github.com/loonghao/shimexe/commit/5f1b06519209ee44756ff6835482cf507f49cd90))
* disable clippy uninlined_format_args lint ([e2d69dd](https://github.com/loonghao/shimexe/commit/e2d69ddfa39690858863deac9a3b3570c8ffed66))
* enhance auto-update with turbo-cdn integration ([2a5e660](https://github.com/loonghao/shimexe/commit/2a5e6607a02fda18df9bd120cd1c054af14a61bb))
* enhance CI workflow with cross-compilation testing ([afb7a58](https://github.com/loonghao/shimexe/commit/afb7a583ef923f3211156f1b29377e5af024fa27))
* enhance HTTP URL support with persistent download tracking ([d169cb7](https://github.com/loonghao/shimexe/commit/d169cb752d916dc1824adb6287656d44c8d5c12c))
* enhance package publishing and fix env config ([332cad3](https://github.com/loonghao/shimexe/commit/332cad316beebd4af5dfd4cf6f3362ce5339779c))
* enhance README with logo and improve vx integration ([6085f46](https://github.com/loonghao/shimexe/commit/6085f4674df70635f106677dfa3289f70371f0d7))
* enhance release workflow with advanced cross-compilation ([e802a67](https://github.com/loonghao/shimexe/commit/e802a67dd88a8243fabd8bf51580cdf9e0c9e3f4))
* enhance release-plz validation with version update check ([ab0df95](https://github.com/loonghao/shimexe/commit/ab0df952d5893364597259209fde276b4000f9d6))
* enhance shim configuration with dynamic template system and improved args handling ([00bfd93](https://github.com/loonghao/shimexe/commit/00bfd93df15a3873032ec0b43dd22dea355d3049))
* enhance unit test coverage with comprehensive test suites ([03addc8](https://github.com/loonghao/shimexe/commit/03addc833391ec3f08973763e6dcc0da0ee8f1cc))
* implement standalone shim execution with local config lookup ([7bde37a](https://github.com/loonghao/shimexe/commit/7bde37a24629ac9e80863fdba838a4797e6b0637))
* improve code coverage and fix codecov configuration ([ced5396](https://github.com/loonghao/shimexe/commit/ced53964bd31d6f12dd1bbfa2e4e1f1c31fc311c))
* integrate automated package manager publishing ([a813bc4](https://github.com/loonghao/shimexe/commit/a813bc494a160e33abba5a22f310345af03150d2))
* integrate automated package manager publishing ([30d072b](https://github.com/loonghao/shimexe/commit/30d072bc20d09c013cb70b2e6d806e352f4700df))
* integrate turbo-cdn and restructure tests ([95de3fb](https://github.com/loonghao/shimexe/commit/95de3fbcd1f6fca4d57edcf21f216df7b7e978f2)), closes [#45](https://github.com/loonghao/shimexe/issues/45)
* migrate from release-plz to release-please ([4203fd4](https://github.com/loonghao/shimexe/commit/4203fd4dea50a5b387fa2a4e4f461ed645983a34))
* modernize GitHub Actions to latest versions ([f099150](https://github.com/loonghao/shimexe/commit/f09915002b94b2b3d19035a79bd0e0345a9b8c7f))
* optimize performance and fix workflow ([76a65e4](https://github.com/loonghao/shimexe/commit/76a65e4379f79fc5562b68d805026459f694f85b))
* setup release-please automation and add crate READMEs ([af4bd8a](https://github.com/loonghao/shimexe/commit/af4bd8a9da1b119d9bd016ca41f528871c129a39))
* simplify release workflow with actions-rust-release ([cec6df5](https://github.com/loonghao/shimexe/commit/cec6df5b46479fd858c07d1761daad5b7eb3485e))
* upgrade turbo-cdn to 0.4.3 and improve verbose logging control ([949a4f6](https://github.com/loonghao/shimexe/commit/949a4f661c57c715fab31ca71248f47289d3098f))
* **windows:** hide console window when running shims ([54a6830](https://github.com/loonghao/shimexe/commit/54a68302b39890aff3ce7e96a8a812b90e359200))


### Bug Fixes

* add git-token parameter to release-plz dry-run ([7d99467](https://github.com/loonghao/shimexe/commit/7d994675570235b904fd72fc8a801a3df9d6eba7))
* add retry logic and fallback for GitHub API rate limits ([ecd6958](https://github.com/loonghao/shimexe/commit/ecd6958007b7fe552917a0657bb60781941b7e43))
* add retry logic to update-packages workflow ([8b76325](https://github.com/loonghao/shimexe/commit/8b76325982a798acb4dbfe9c1dd4f43151088b81))
* add shimexe-cli to release-please configuration ([02e445d](https://github.com/loonghao/shimexe/commit/02e445d52c29150219dbd8f716f35dc2226f9131))
* add version specification for shimexe-core dependency to enable crates.io publishing ([f6ff396](https://github.com/loonghao/shimexe/commit/f6ff39687f70a9e7db54b1779010d24ac35ea9cc))
* adjust performance test timeouts for Windows environment ([d0c4bd3](https://github.com/loonghao/shimexe/commit/d0c4bd379cb06d8e494edc7011a46e350a89883d))
* **ci:** cleanup stale 'autorelease: pending' labels before running release-please ([fa5199c](https://github.com/loonghao/shimexe/commit/fa5199c8bf68f631d6c83b96a504e1870119a70a))
* **ci:** remove cargo-workspace plugin; rely on explicit packages mapping for root autobump ([0a61989](https://github.com/loonghao/shimexe/commit/0a619894410d7dacac47081a1a785efea69e9828))
* **ci:** set release-please PR title pattern to include scope/component/version ([8816990](https://github.com/loonghao/shimexe/commit/8816990c98c0411309ddc8bfef6c208c6ff1e6b5))
* clean working directory by removing tracked build files ([c86db29](https://github.com/loonghao/shimexe/commit/c86db296a85af0b144e2e1db08b044b50d5d3702))
* complete unit test coverage enhancement ([42041ae](https://github.com/loonghao/shimexe/commit/42041ae25a30c55c4f551315f5be48d7259e720f))
* completely disable codecov status checks to prevent CI failures ([015597f](https://github.com/loonghao/shimexe/commit/015597f5f476566f0ff2c50f5d1fb3bba781e4df))
* completely rewrite release-please config for Rust workspace ([353eb0c](https://github.com/loonghao/shimexe/commit/353eb0c527363bb21bf39d48760b50897b383d3c))
* comprehensive GitHub Actions permissions and CI configuration ([8aaaa40](https://github.com/loonghao/shimexe/commit/8aaaa40609ab5d7ba92650c52dc706005822c87a))
* configure release-plz to show only current version changelog in releases ([2472b5a](https://github.com/loonghao/shimexe/commit/2472b5a2f3a224d463a4a709e936a6680d4feba9))
* correct install script URLs to use raw.githubusercontent.com ([83275ee](https://github.com/loonghao/shimexe/commit/83275ee03436808250eeeb8afb35dada966088f4))
* correct PATH usage and improve user guidance ([291763e](https://github.com/loonghao/shimexe/commit/291763ee24a8218cf9224d004a1b3e0ab3c57e4c))
* correct release tag format in install scripts ([de4ac68](https://github.com/loonghao/shimexe/commit/de4ac683a490c9260489dbc894b4224570971e34))
* correct test implementations to match actual API ([cc35da6](https://github.com/loonghao/shimexe/commit/cc35da6f63a535c616dcfde54db22d0da5ba6535))
* **deps:** update rust crate tokio to v1.46.1 ([e423b26](https://github.com/loonghao/shimexe/commit/e423b2658a1f36e52459ae267c65091411d14ad5))
* **deps:** update rust crate tokio to v1.47.0 ([8e186b9](https://github.com/loonghao/shimexe/commit/8e186b94affd1dbbe7a5de7fe00785e6d34cffd0))
* **deps:** update rust crate zip to v4 ([f7223d5](https://github.com/loonghao/shimexe/commit/f7223d5216305757045da9fc72d2cd131cb74f07))
* **deps:** update rust crate zip to v4.2.0 ([87bd01f](https://github.com/loonghao/shimexe/commit/87bd01f96e206e4518ff48b4e80214c21f73da94))
* enable release workflow triggering with PAT token ([807e0e9](https://github.com/loonghao/shimexe/commit/807e0e92029f2881d7702d24a5da606de9e52561))
* improve release workflow to handle existing releases ([be6a75e](https://github.com/loonghao/shimexe/commit/be6a75e6815447a86e23cadd6b7c4089d65ec9a1))
* improve version detection and release changelog formatting ([7003e09](https://github.com/loonghao/shimexe/commit/7003e09484d4668b2ec7e45e0b472d1ea66a9e27))
* **info:** avoid compile-time env! for TARGET/RUSTC_VERSION; use runtime fallbacks ([6970011](https://github.com/loonghao/shimexe/commit/69700114defefaf4942502d19afab6634c64100b))
* make codecov informational only, don't fail CI ([cc8dace](https://github.com/loonghao/shimexe/commit/cc8dace27bb764957552a11216c4a80324ee7489))
* optimize release workflow for tag-triggered releases ([78a27ff](https://github.com/loonghao/shimexe/commit/78a27ff5dccdd6be228db68914d3d69cece73bf6))
* optimize release workflow trigger conditions ([749406c](https://github.com/loonghao/shimexe/commit/749406cc7c7a96d3fb2a0fa954a81a108d62f16d))
* optimize release-plz config for mixed workspace ([b466497](https://github.com/loonghao/shimexe/commit/b466497845c83f4075c4282439c527d08dace412))
* remove needless borrow in archive path handling ([874247a](https://github.com/loonghao/shimexe/commit/874247a8c5a9ec79c87bdcb056d3da9dc3ae5571))
* remove root directory from release-please configuration ([75ed065](https://github.com/loonghao/shimexe/commit/75ed06561bb97660e41f52040c885cb99585b6d2))
* remove unsupported update_files from release-plz config ([1d917c4](https://github.com/loonghao/shimexe/commit/1d917c47eb3ce60ec8a0cfcdd5c5c84deeb70228))
* replace winget with chocolatey for ImageMagick installation ([c3fd017](https://github.com/loonghao/shimexe/commit/c3fd017ec05583851baa94ae7a816ebababf1a59))
* resolve all compilation errors and template syntax issues ([82122f5](https://github.com/loonghao/shimexe/commit/82122f5161e6c253497a55604fe49563f57532a5))
* resolve CI configuration and testing issues ([8754f55](https://github.com/loonghao/shimexe/commit/8754f553a9ce9d601be7713ab02f0b9aa06c9628))
* resolve clippy and rustdoc warnings for CI compatibility ([a1acc7b](https://github.com/loonghao/shimexe/commit/a1acc7b51892c356d964ac21fe99bec6ecadc24f))
* resolve clippy warnings and compilation errors ([ee01ee3](https://github.com/loonghao/shimexe/commit/ee01ee3c161c9b41df0764e69f039eceb4f861a9))
* resolve clippy warnings and implement Default trait for ConfigCache ([e6641b3](https://github.com/loonghao/shimexe/commit/e6641b3a1a59e41e0f97454bfd32e2fc53f8bcc0))
* resolve clippy warnings in test files ([1d4196b](https://github.com/loonghao/shimexe/commit/1d4196bca81362410336254c97960b1bfc1d2183))
* resolve compilation errors in test files ([ebf65ec](https://github.com/loonghao/shimexe/commit/ebf65ecb6ab39a90f09364891c7ff5a547b67868))
* resolve compilation errors in tests ([2ea6753](https://github.com/loonghao/shimexe/commit/2ea6753c90dd36622c2b1306f19b5e93c683031c))
* resolve duplicate release issues and improve Chocolatey publishing ([f7286cb](https://github.com/loonghao/shimexe/commit/f7286cb18807fcabe7246494fa7d5fa821bd4cc9))
* resolve git detached HEAD issue in release-plz dry-run ([e09a7df](https://github.com/loonghao/shimexe/commit/e09a7df4fa74964faf9f4ba1bfca1136b2bffcae))
* resolve Git merge conflict markers in release-please manifest ([1a082cd](https://github.com/loonghao/shimexe/commit/1a082cdc8da7f2de63e0a45e6d33173fa25c82d4))
* resolve GitHub Actions permission issue for release-please ([bbcd794](https://github.com/loonghao/shimexe/commit/bbcd794f83cc5029b7c16e3d8539dbb5b38e3c48))
* resolve OpenSSL cross-compilation issues with rustls ([9c11fab](https://github.com/loonghao/shimexe/commit/9c11fabbb25906dddc34d2b71db4ad3c9386b502))
* resolve package manager publishing issues ([1f53f43](https://github.com/loonghao/shimexe/commit/1f53f43681e4ac6489098b3e895911b264d27759))
* resolve release-please configuration issues ([5875b0d](https://github.com/loonghao/shimexe/commit/5875b0d39d90e8c187916bc89eaf9712d8cd0bb8))
* resolve release-plz config and clippy warnings ([409d2e3](https://github.com/loonghao/shimexe/commit/409d2e3576424de45fd9f580d44e8ffcd90bd885))
* resolve release-plz configuration issues ([6955e44](https://github.com/loonghao/shimexe/commit/6955e4487ba81e533bfbd9654086dab215ead0fa))
* resolve remaining clippy uninlined format args warnings ([26266bb](https://github.com/loonghao/shimexe/commit/26266bb516907fab760f793b5103827e83e11f0d))
* resolve remaining redundant_closure warning in runner.rs ([e22b419](https://github.com/loonghao/shimexe/commit/e22b419b0bce0077bcf03a7033b6d61be472edd8))
* resolve TOML serialization test failure ([4a99cf7](https://github.com/loonghao/shimexe/commit/4a99cf7a05a6c1406beaa1a5d6307a3f9bccfd98))
* resolve utils test failures for environment variables ([cea8746](https://github.com/loonghao/shimexe/commit/cea874653aca5868a6cc8de62ee2f0bf65ed2390))
* resolve version detection issues in install scripts and update Scoop manifest ([054774a](https://github.com/loonghao/shimexe/commit/054774ae09a587ba22b1223c0465e0bc56415898))
* resolve workflow conflicts between release.yml and update-packages.yml ([5929ba6](https://github.com/loonghao/shimexe/commit/5929ba63fda95059409e1dd3e37bde94aa6b47c4))
* set explicit version for root package to resolve cargo-workspace issue ([900c6f7](https://github.com/loonghao/shimexe/commit/900c6f7e1df88a7cde90cae40ce10fa9afb526d0))
* set explicit version for shimexe-core to resolve cargo-workspace issue ([304e13b](https://github.com/loonghao/shimexe/commit/304e13b1a6b66153e7b538b3c3c665b1bcaf8c4b))
* support both v* and shimexe-v* tag formats in install scripts ([f8ea896](https://github.com/loonghao/shimexe/commit/f8ea89629000758d22e70b238d46e86f2dc2ce71))
* update README_zh.md and fix install script file naming ([dcdc637](https://github.com/loonghao/shimexe/commit/dcdc6370ecbd389c86cecee4fb7b1caa2039b0bd))
* update release-please configuration to resolve workflow issues ([bfcceb3](https://github.com/loonghao/shimexe/commit/bfcceb3d4d4f54a21c1bc0ff68b1824e44b30036))
* update release-plz configuration and add dry-run checks ([07d3a92](https://github.com/loonghao/shimexe/commit/07d3a9244ec3d30f3c62f4cf4a8e18c29892ee2a))
* update release-plz configuration for standard tag format ([479177c](https://github.com/loonghao/shimexe/commit/479177c1484d21a552852fd3f23906fb615cb4cb))
* update workspace license to MIT only ([0558a7b](https://github.com/loonghao/shimexe/commit/0558a7ba54b1c32c6d4b52fb03a242aeba1f0308))
* use official release-plz GitHub Action ([3cd74d0](https://github.com/loonghao/shimexe/commit/3cd74d083cc194da7ddb96c77ed3bad2f53b0519))
* **workflows:** resolve YAML errors and move package managers update under release-assets; add actionlint; make coverage non-blocking ([9f72088](https://github.com/loonghao/shimexe/commit/9f72088fcf154a8a5db2cb48aa4d3d56726159b4))


### Performance Improvements

* **core:** remove unused file_modified from cache entry; trust cached value within TTL to avoid fs IO; fix -D dead-code ([5f5f77d](https://github.com/loonghao/shimexe/commit/5f5f77da7e9bf69d861c38cd1da705586b727c08))
* optimize CI/CD workflows and fix GitHub API auth ([0b2131b](https://github.com/loonghao/shimexe/commit/0b2131b3e37563860153a63fc6e5bbcff3839822))


### Code Refactoring

* move GitHub token validation to CI workflow ([6676aef](https://github.com/loonghao/shimexe/commit/6676aef8c8632af8343cc67db361e761a2650cf2))
* move release-plz dry-run to CI workflow ([bdb71a1](https://github.com/loonghao/shimexe/commit/bdb71a1427225c91104982227a01b3c726e14e0b))
* move shimexe-cli to root src directory ([d533ec6](https://github.com/loonghao/shimexe/commit/d533ec6535e9393cbe4ba949443fa2db54839f65))
* optimize dependency installation with unified approach ([8fde233](https://github.com/loonghao/shimexe/commit/8fde233d8d8dd966949efcc2b94d8cdd63fb0ba0))
* rename [@pkg](https://github.com/pkg) to pkg directory and fix code quality ([e3b0da8](https://github.com/loonghao/shimexe/commit/e3b0da89c2042d1ab60cace62352c96d327e830b))
* reorganize CI workflow responsibilities ([edeab8c](https://github.com/loonghao/shimexe/commit/edeab8c456e122ed71c177049d099c00df687f81))
* reorganize package manager files to [@pkg](https://github.com/pkg) directory ([4baa079](https://github.com/loonghao/shimexe/commit/4baa079fb660c96675d969d8da36404fd27a43b3))
* simplify PR validation to config-only check ([540333b](https://github.com/loonghao/shimexe/commit/540333b801218e2ceb6b049408a55cfc90bedcc3))


### Documentation

* update README and add comprehensive unit tests ([c35b70d](https://github.com/loonghao/shimexe/commit/c35b70df7102a0d9fdae9394e10ec8aa13967109))

## [0.5.9](https://github.com/loonghao/shimexe/compare/v0.5.8...v0.5.9) (2025-08-10)


### Bug Fixes

* **ci:** cleanup stale 'autorelease: pending' labels before running release-please ([fa5199c](https://github.com/loonghao/shimexe/commit/fa5199c8bf68f631d6c83b96a504e1870119a70a))
* **ci:** remove cargo-workspace plugin; rely on explicit packages mapping for root autobump ([0a61989](https://github.com/loonghao/shimexe/commit/0a619894410d7dacac47081a1a785efea69e9828))
* **ci:** set release-please PR title pattern to include scope/component/version ([8816990](https://github.com/loonghao/shimexe/commit/8816990c98c0411309ddc8bfef6c208c6ff1e6b5))
* **workflows:** resolve YAML errors and move package managers update under release-assets; add actionlint; make coverage non-blocking ([9f72088](https://github.com/loonghao/shimexe/commit/9f72088fcf154a8a5db2cb48aa4d3d56726159b4))


### Performance Improvements

* **core:** remove unused file_modified from cache entry; trust cached value within TTL to avoid fs IO; fix -D dead-code ([5f5f77d](https://github.com/loonghao/shimexe/commit/5f5f77da7e9bf69d861c38cd1da705586b727c08))

## [0.5.8](https://github.com/loonghao/shimexe/compare/v0.5.7...v0.5.8) (2025-08-08)


### Features

* add --info flag to display detailed system and version information ([cfd86c1](https://github.com/loonghao/shimexe/commit/cfd86c18370653d984132e7d56cc05d2d4675062))
* add comprehensive archive support and package management ([48ce5c5](https://github.com/loonghao/shimexe/commit/48ce5c54b6f74c1d57fa1960cf7c962ab476dc40))
* add comprehensive vx integration example ([c088d97](https://github.com/loonghao/shimexe/commit/c088d972664de35fcbc8ec8b664f6d1544c6dfcf))
* add high-level ShimManager API and update Chinese README ([e98666d](https://github.com/loonghao/shimexe/commit/e98666dceb763aea2109766f5f5f10fdac00ccf1))
* add HTTP URL support for automatic executable download ([28abbc0](https://github.com/loonghao/shimexe/commit/28abbc0902fe0b68275cf3c5f87ea7288960698a))
* add multi-platform package management and enhanced badges ([d60371a](https://github.com/loonghao/shimexe/commit/d60371ab02985e3e87d2fab565cd708c147fa59b))
* add src directory monitoring to release-please ([5f1b065](https://github.com/loonghao/shimexe/commit/5f1b06519209ee44756ff6835482cf507f49cd90))
* disable clippy uninlined_format_args lint ([e2d69dd](https://github.com/loonghao/shimexe/commit/e2d69ddfa39690858863deac9a3b3570c8ffed66))
* enhance auto-update with turbo-cdn integration ([2a5e660](https://github.com/loonghao/shimexe/commit/2a5e6607a02fda18df9bd120cd1c054af14a61bb))
* enhance CI workflow with cross-compilation testing ([afb7a58](https://github.com/loonghao/shimexe/commit/afb7a583ef923f3211156f1b29377e5af024fa27))
* enhance HTTP URL support with persistent download tracking ([d169cb7](https://github.com/loonghao/shimexe/commit/d169cb752d916dc1824adb6287656d44c8d5c12c))
* enhance package publishing and fix env config ([332cad3](https://github.com/loonghao/shimexe/commit/332cad316beebd4af5dfd4cf6f3362ce5339779c))
* enhance README with logo and improve vx integration ([6085f46](https://github.com/loonghao/shimexe/commit/6085f4674df70635f106677dfa3289f70371f0d7))
* enhance release workflow with advanced cross-compilation ([e802a67](https://github.com/loonghao/shimexe/commit/e802a67dd88a8243fabd8bf51580cdf9e0c9e3f4))
* enhance release-plz validation with version update check ([ab0df95](https://github.com/loonghao/shimexe/commit/ab0df952d5893364597259209fde276b4000f9d6))
* enhance shim configuration with dynamic template system and improved args handling ([00bfd93](https://github.com/loonghao/shimexe/commit/00bfd93df15a3873032ec0b43dd22dea355d3049))
* enhance unit test coverage with comprehensive test suites ([03addc8](https://github.com/loonghao/shimexe/commit/03addc833391ec3f08973763e6dcc0da0ee8f1cc))
* implement standalone shim execution with local config lookup ([7bde37a](https://github.com/loonghao/shimexe/commit/7bde37a24629ac9e80863fdba838a4797e6b0637))
* improve code coverage and fix codecov configuration ([ced5396](https://github.com/loonghao/shimexe/commit/ced53964bd31d6f12dd1bbfa2e4e1f1c31fc311c))
* integrate automated package manager publishing ([a813bc4](https://github.com/loonghao/shimexe/commit/a813bc494a160e33abba5a22f310345af03150d2))
* integrate automated package manager publishing ([30d072b](https://github.com/loonghao/shimexe/commit/30d072bc20d09c013cb70b2e6d806e352f4700df))
* integrate turbo-cdn and restructure tests ([95de3fb](https://github.com/loonghao/shimexe/commit/95de3fbcd1f6fca4d57edcf21f216df7b7e978f2)), closes [#45](https://github.com/loonghao/shimexe/issues/45)
* migrate from release-plz to release-please ([4203fd4](https://github.com/loonghao/shimexe/commit/4203fd4dea50a5b387fa2a4e4f461ed645983a34))
* modernize GitHub Actions to latest versions ([f099150](https://github.com/loonghao/shimexe/commit/f09915002b94b2b3d19035a79bd0e0345a9b8c7f))
* optimize performance and fix workflow ([76a65e4](https://github.com/loonghao/shimexe/commit/76a65e4379f79fc5562b68d805026459f694f85b))
* setup release-please automation and add crate READMEs ([af4bd8a](https://github.com/loonghao/shimexe/commit/af4bd8a9da1b119d9bd016ca41f528871c129a39))
* simplify release workflow with actions-rust-release ([cec6df5](https://github.com/loonghao/shimexe/commit/cec6df5b46479fd858c07d1761daad5b7eb3485e))
* upgrade turbo-cdn to 0.4.3 and improve verbose logging control ([949a4f6](https://github.com/loonghao/shimexe/commit/949a4f661c57c715fab31ca71248f47289d3098f))
* **windows:** hide console window when running shims ([54a6830](https://github.com/loonghao/shimexe/commit/54a68302b39890aff3ce7e96a8a812b90e359200))


### Bug Fixes

* add git-token parameter to release-plz dry-run ([7d99467](https://github.com/loonghao/shimexe/commit/7d994675570235b904fd72fc8a801a3df9d6eba7))
* add retry logic and fallback for GitHub API rate limits ([ecd6958](https://github.com/loonghao/shimexe/commit/ecd6958007b7fe552917a0657bb60781941b7e43))
* add retry logic to update-packages workflow ([8b76325](https://github.com/loonghao/shimexe/commit/8b76325982a798acb4dbfe9c1dd4f43151088b81))
* add shimexe-cli to release-please configuration ([02e445d](https://github.com/loonghao/shimexe/commit/02e445d52c29150219dbd8f716f35dc2226f9131))
* add version specification for shimexe-core dependency to enable crates.io publishing ([f6ff396](https://github.com/loonghao/shimexe/commit/f6ff39687f70a9e7db54b1779010d24ac35ea9cc))
* adjust performance test timeouts for Windows environment ([d0c4bd3](https://github.com/loonghao/shimexe/commit/d0c4bd379cb06d8e494edc7011a46e350a89883d))
* clean working directory by removing tracked build files ([c86db29](https://github.com/loonghao/shimexe/commit/c86db296a85af0b144e2e1db08b044b50d5d3702))
* complete unit test coverage enhancement ([42041ae](https://github.com/loonghao/shimexe/commit/42041ae25a30c55c4f551315f5be48d7259e720f))
* completely disable codecov status checks to prevent CI failures ([015597f](https://github.com/loonghao/shimexe/commit/015597f5f476566f0ff2c50f5d1fb3bba781e4df))
* completely rewrite release-please config for Rust workspace ([353eb0c](https://github.com/loonghao/shimexe/commit/353eb0c527363bb21bf39d48760b50897b383d3c))
* comprehensive GitHub Actions permissions and CI configuration ([8aaaa40](https://github.com/loonghao/shimexe/commit/8aaaa40609ab5d7ba92650c52dc706005822c87a))
* configure release-plz to show only current version changelog in releases ([2472b5a](https://github.com/loonghao/shimexe/commit/2472b5a2f3a224d463a4a709e936a6680d4feba9))
* correct install script URLs to use raw.githubusercontent.com ([83275ee](https://github.com/loonghao/shimexe/commit/83275ee03436808250eeeb8afb35dada966088f4))
* correct PATH usage and improve user guidance ([291763e](https://github.com/loonghao/shimexe/commit/291763ee24a8218cf9224d004a1b3e0ab3c57e4c))
* correct release tag format in install scripts ([de4ac68](https://github.com/loonghao/shimexe/commit/de4ac683a490c9260489dbc894b4224570971e34))
* correct test implementations to match actual API ([cc35da6](https://github.com/loonghao/shimexe/commit/cc35da6f63a535c616dcfde54db22d0da5ba6535))
* **deps:** update rust crate tokio to v1.46.1 ([e423b26](https://github.com/loonghao/shimexe/commit/e423b2658a1f36e52459ae267c65091411d14ad5))
* **deps:** update rust crate tokio to v1.47.0 ([8e186b9](https://github.com/loonghao/shimexe/commit/8e186b94affd1dbbe7a5de7fe00785e6d34cffd0))
* **deps:** update rust crate zip to v4 ([f7223d5](https://github.com/loonghao/shimexe/commit/f7223d5216305757045da9fc72d2cd131cb74f07))
* **deps:** update rust crate zip to v4.2.0 ([87bd01f](https://github.com/loonghao/shimexe/commit/87bd01f96e206e4518ff48b4e80214c21f73da94))
* enable release workflow triggering with PAT token ([807e0e9](https://github.com/loonghao/shimexe/commit/807e0e92029f2881d7702d24a5da606de9e52561))
* improve release workflow to handle existing releases ([be6a75e](https://github.com/loonghao/shimexe/commit/be6a75e6815447a86e23cadd6b7c4089d65ec9a1))
* improve version detection and release changelog formatting ([7003e09](https://github.com/loonghao/shimexe/commit/7003e09484d4668b2ec7e45e0b472d1ea66a9e27))
* **info:** avoid compile-time env! for TARGET/RUSTC_VERSION; use runtime fallbacks ([6970011](https://github.com/loonghao/shimexe/commit/69700114defefaf4942502d19afab6634c64100b))
* make codecov informational only, don't fail CI ([cc8dace](https://github.com/loonghao/shimexe/commit/cc8dace27bb764957552a11216c4a80324ee7489))
* optimize release workflow for tag-triggered releases ([78a27ff](https://github.com/loonghao/shimexe/commit/78a27ff5dccdd6be228db68914d3d69cece73bf6))
* optimize release workflow trigger conditions ([749406c](https://github.com/loonghao/shimexe/commit/749406cc7c7a96d3fb2a0fa954a81a108d62f16d))
* optimize release-plz config for mixed workspace ([b466497](https://github.com/loonghao/shimexe/commit/b466497845c83f4075c4282439c527d08dace412))
* remove needless borrow in archive path handling ([874247a](https://github.com/loonghao/shimexe/commit/874247a8c5a9ec79c87bdcb056d3da9dc3ae5571))
* remove root directory from release-please configuration ([75ed065](https://github.com/loonghao/shimexe/commit/75ed06561bb97660e41f52040c885cb99585b6d2))
* remove unsupported update_files from release-plz config ([1d917c4](https://github.com/loonghao/shimexe/commit/1d917c47eb3ce60ec8a0cfcdd5c5c84deeb70228))
* replace winget with chocolatey for ImageMagick installation ([c3fd017](https://github.com/loonghao/shimexe/commit/c3fd017ec05583851baa94ae7a816ebababf1a59))
* resolve all compilation errors and template syntax issues ([82122f5](https://github.com/loonghao/shimexe/commit/82122f5161e6c253497a55604fe49563f57532a5))
* resolve CI configuration and testing issues ([8754f55](https://github.com/loonghao/shimexe/commit/8754f553a9ce9d601be7713ab02f0b9aa06c9628))
* resolve clippy and rustdoc warnings for CI compatibility ([a1acc7b](https://github.com/loonghao/shimexe/commit/a1acc7b51892c356d964ac21fe99bec6ecadc24f))
* resolve clippy warnings and compilation errors ([ee01ee3](https://github.com/loonghao/shimexe/commit/ee01ee3c161c9b41df0764e69f039eceb4f861a9))
* resolve clippy warnings and implement Default trait for ConfigCache ([e6641b3](https://github.com/loonghao/shimexe/commit/e6641b3a1a59e41e0f97454bfd32e2fc53f8bcc0))
* resolve clippy warnings in test files ([1d4196b](https://github.com/loonghao/shimexe/commit/1d4196bca81362410336254c97960b1bfc1d2183))
* resolve compilation errors in test files ([ebf65ec](https://github.com/loonghao/shimexe/commit/ebf65ecb6ab39a90f09364891c7ff5a547b67868))
* resolve compilation errors in tests ([2ea6753](https://github.com/loonghao/shimexe/commit/2ea6753c90dd36622c2b1306f19b5e93c683031c))
* resolve duplicate release issues and improve Chocolatey publishing ([f7286cb](https://github.com/loonghao/shimexe/commit/f7286cb18807fcabe7246494fa7d5fa821bd4cc9))
* resolve git detached HEAD issue in release-plz dry-run ([e09a7df](https://github.com/loonghao/shimexe/commit/e09a7df4fa74964faf9f4ba1bfca1136b2bffcae))
* resolve Git merge conflict markers in release-please manifest ([1a082cd](https://github.com/loonghao/shimexe/commit/1a082cdc8da7f2de63e0a45e6d33173fa25c82d4))
* resolve GitHub Actions permission issue for release-please ([bbcd794](https://github.com/loonghao/shimexe/commit/bbcd794f83cc5029b7c16e3d8539dbb5b38e3c48))
* resolve OpenSSL cross-compilation issues with rustls ([9c11fab](https://github.com/loonghao/shimexe/commit/9c11fabbb25906dddc34d2b71db4ad3c9386b502))
* resolve package manager publishing issues ([1f53f43](https://github.com/loonghao/shimexe/commit/1f53f43681e4ac6489098b3e895911b264d27759))
* resolve release-please configuration issues ([5875b0d](https://github.com/loonghao/shimexe/commit/5875b0d39d90e8c187916bc89eaf9712d8cd0bb8))
* resolve release-plz config and clippy warnings ([409d2e3](https://github.com/loonghao/shimexe/commit/409d2e3576424de45fd9f580d44e8ffcd90bd885))
* resolve release-plz configuration issues ([6955e44](https://github.com/loonghao/shimexe/commit/6955e4487ba81e533bfbd9654086dab215ead0fa))
* resolve remaining clippy uninlined format args warnings ([26266bb](https://github.com/loonghao/shimexe/commit/26266bb516907fab760f793b5103827e83e11f0d))
* resolve remaining redundant_closure warning in runner.rs ([e22b419](https://github.com/loonghao/shimexe/commit/e22b419b0bce0077bcf03a7033b6d61be472edd8))
* resolve TOML serialization test failure ([4a99cf7](https://github.com/loonghao/shimexe/commit/4a99cf7a05a6c1406beaa1a5d6307a3f9bccfd98))
* resolve utils test failures for environment variables ([cea8746](https://github.com/loonghao/shimexe/commit/cea874653aca5868a6cc8de62ee2f0bf65ed2390))
* resolve version detection issues in install scripts and update Scoop manifest ([054774a](https://github.com/loonghao/shimexe/commit/054774ae09a587ba22b1223c0465e0bc56415898))
* resolve workflow conflicts between release.yml and update-packages.yml ([5929ba6](https://github.com/loonghao/shimexe/commit/5929ba63fda95059409e1dd3e37bde94aa6b47c4))
* set explicit version for root package to resolve cargo-workspace issue ([900c6f7](https://github.com/loonghao/shimexe/commit/900c6f7e1df88a7cde90cae40ce10fa9afb526d0))
* set explicit version for shimexe-core to resolve cargo-workspace issue ([304e13b](https://github.com/loonghao/shimexe/commit/304e13b1a6b66153e7b538b3c3c665b1bcaf8c4b))
* support both v* and shimexe-v* tag formats in install scripts ([f8ea896](https://github.com/loonghao/shimexe/commit/f8ea89629000758d22e70b238d46e86f2dc2ce71))
* update README_zh.md and fix install script file naming ([dcdc637](https://github.com/loonghao/shimexe/commit/dcdc6370ecbd389c86cecee4fb7b1caa2039b0bd))
* update release-please configuration to resolve workflow issues ([bfcceb3](https://github.com/loonghao/shimexe/commit/bfcceb3d4d4f54a21c1bc0ff68b1824e44b30036))
* update release-plz configuration and add dry-run checks ([07d3a92](https://github.com/loonghao/shimexe/commit/07d3a9244ec3d30f3c62f4cf4a8e18c29892ee2a))
* update release-plz configuration for standard tag format ([479177c](https://github.com/loonghao/shimexe/commit/479177c1484d21a552852fd3f23906fb615cb4cb))
* update workspace license to MIT only ([0558a7b](https://github.com/loonghao/shimexe/commit/0558a7ba54b1c32c6d4b52fb03a242aeba1f0308))
* use official release-plz GitHub Action ([3cd74d0](https://github.com/loonghao/shimexe/commit/3cd74d083cc194da7ddb96c77ed3bad2f53b0519))


### Performance Improvements

* optimize CI/CD workflows and fix GitHub API auth ([0b2131b](https://github.com/loonghao/shimexe/commit/0b2131b3e37563860153a63fc6e5bbcff3839822))


### Code Refactoring

* move GitHub token validation to CI workflow ([6676aef](https://github.com/loonghao/shimexe/commit/6676aef8c8632af8343cc67db361e761a2650cf2))
* move release-plz dry-run to CI workflow ([bdb71a1](https://github.com/loonghao/shimexe/commit/bdb71a1427225c91104982227a01b3c726e14e0b))
* move shimexe-cli to root src directory ([d533ec6](https://github.com/loonghao/shimexe/commit/d533ec6535e9393cbe4ba949443fa2db54839f65))
* optimize dependency installation with unified approach ([8fde233](https://github.com/loonghao/shimexe/commit/8fde233d8d8dd966949efcc2b94d8cdd63fb0ba0))
* rename [@pkg](https://github.com/pkg) to pkg directory and fix code quality ([e3b0da8](https://github.com/loonghao/shimexe/commit/e3b0da89c2042d1ab60cace62352c96d327e830b))
* reorganize CI workflow responsibilities ([edeab8c](https://github.com/loonghao/shimexe/commit/edeab8c456e122ed71c177049d099c00df687f81))
* reorganize package manager files to [@pkg](https://github.com/pkg) directory ([4baa079](https://github.com/loonghao/shimexe/commit/4baa079fb660c96675d969d8da36404fd27a43b3))
* simplify PR validation to config-only check ([540333b](https://github.com/loonghao/shimexe/commit/540333b801218e2ceb6b049408a55cfc90bedcc3))


### Documentation

* update README and add comprehensive unit tests ([c35b70d](https://github.com/loonghao/shimexe/commit/c35b70df7102a0d9fdae9394e10ec8aa13967109))

## [0.5.7](https://github.com/loonghao/shimexe/compare/v0.5.6...v0.5.7) (2025-08-08)


### Features

* add --info flag to display detailed system and version information ([cfd86c1](https://github.com/loonghao/shimexe/commit/cfd86c18370653d984132e7d56cc05d2d4675062))
* add comprehensive archive support and package management ([48ce5c5](https://github.com/loonghao/shimexe/commit/48ce5c54b6f74c1d57fa1960cf7c962ab476dc40))
* add comprehensive vx integration example ([c088d97](https://github.com/loonghao/shimexe/commit/c088d972664de35fcbc8ec8b664f6d1544c6dfcf))
* add high-level ShimManager API and update Chinese README ([e98666d](https://github.com/loonghao/shimexe/commit/e98666dceb763aea2109766f5f5f10fdac00ccf1))
* add HTTP URL support for automatic executable download ([28abbc0](https://github.com/loonghao/shimexe/commit/28abbc0902fe0b68275cf3c5f87ea7288960698a))
* add multi-platform package management and enhanced badges ([d60371a](https://github.com/loonghao/shimexe/commit/d60371ab02985e3e87d2fab565cd708c147fa59b))
* add src directory monitoring to release-please ([5f1b065](https://github.com/loonghao/shimexe/commit/5f1b06519209ee44756ff6835482cf507f49cd90))
* disable clippy uninlined_format_args lint ([e2d69dd](https://github.com/loonghao/shimexe/commit/e2d69ddfa39690858863deac9a3b3570c8ffed66))
* enhance auto-update with turbo-cdn integration ([2a5e660](https://github.com/loonghao/shimexe/commit/2a5e6607a02fda18df9bd120cd1c054af14a61bb))
* enhance CI workflow with cross-compilation testing ([afb7a58](https://github.com/loonghao/shimexe/commit/afb7a583ef923f3211156f1b29377e5af024fa27))
* enhance HTTP URL support with persistent download tracking ([d169cb7](https://github.com/loonghao/shimexe/commit/d169cb752d916dc1824adb6287656d44c8d5c12c))
* enhance package publishing and fix env config ([332cad3](https://github.com/loonghao/shimexe/commit/332cad316beebd4af5dfd4cf6f3362ce5339779c))
* enhance README with logo and improve vx integration ([6085f46](https://github.com/loonghao/shimexe/commit/6085f4674df70635f106677dfa3289f70371f0d7))
* enhance release workflow with advanced cross-compilation ([e802a67](https://github.com/loonghao/shimexe/commit/e802a67dd88a8243fabd8bf51580cdf9e0c9e3f4))
* enhance release-plz validation with version update check ([ab0df95](https://github.com/loonghao/shimexe/commit/ab0df952d5893364597259209fde276b4000f9d6))
* enhance shim configuration with dynamic template system and improved args handling ([00bfd93](https://github.com/loonghao/shimexe/commit/00bfd93df15a3873032ec0b43dd22dea355d3049))
* enhance unit test coverage with comprehensive test suites ([03addc8](https://github.com/loonghao/shimexe/commit/03addc833391ec3f08973763e6dcc0da0ee8f1cc))
* implement standalone shim execution with local config lookup ([7bde37a](https://github.com/loonghao/shimexe/commit/7bde37a24629ac9e80863fdba838a4797e6b0637))
* improve code coverage and fix codecov configuration ([ced5396](https://github.com/loonghao/shimexe/commit/ced53964bd31d6f12dd1bbfa2e4e1f1c31fc311c))
* integrate automated package manager publishing ([a813bc4](https://github.com/loonghao/shimexe/commit/a813bc494a160e33abba5a22f310345af03150d2))
* integrate automated package manager publishing ([30d072b](https://github.com/loonghao/shimexe/commit/30d072bc20d09c013cb70b2e6d806e352f4700df))
* integrate turbo-cdn and restructure tests ([95de3fb](https://github.com/loonghao/shimexe/commit/95de3fbcd1f6fca4d57edcf21f216df7b7e978f2)), closes [#45](https://github.com/loonghao/shimexe/issues/45)
* migrate from release-plz to release-please ([4203fd4](https://github.com/loonghao/shimexe/commit/4203fd4dea50a5b387fa2a4e4f461ed645983a34))
* modernize GitHub Actions to latest versions ([f099150](https://github.com/loonghao/shimexe/commit/f09915002b94b2b3d19035a79bd0e0345a9b8c7f))
* optimize performance and fix workflow ([76a65e4](https://github.com/loonghao/shimexe/commit/76a65e4379f79fc5562b68d805026459f694f85b))
* setup release-please automation and add crate READMEs ([af4bd8a](https://github.com/loonghao/shimexe/commit/af4bd8a9da1b119d9bd016ca41f528871c129a39))
* simplify release workflow with actions-rust-release ([cec6df5](https://github.com/loonghao/shimexe/commit/cec6df5b46479fd858c07d1761daad5b7eb3485e))
* upgrade turbo-cdn to 0.4.3 and improve verbose logging control ([949a4f6](https://github.com/loonghao/shimexe/commit/949a4f661c57c715fab31ca71248f47289d3098f))
* **windows:** hide console window when running shims ([54a6830](https://github.com/loonghao/shimexe/commit/54a68302b39890aff3ce7e96a8a812b90e359200))


### Bug Fixes

* add git-token parameter to release-plz dry-run ([7d99467](https://github.com/loonghao/shimexe/commit/7d994675570235b904fd72fc8a801a3df9d6eba7))
* add retry logic and fallback for GitHub API rate limits ([ecd6958](https://github.com/loonghao/shimexe/commit/ecd6958007b7fe552917a0657bb60781941b7e43))
* add retry logic to update-packages workflow ([8b76325](https://github.com/loonghao/shimexe/commit/8b76325982a798acb4dbfe9c1dd4f43151088b81))
* add shimexe-cli to release-please configuration ([02e445d](https://github.com/loonghao/shimexe/commit/02e445d52c29150219dbd8f716f35dc2226f9131))
* add version specification for shimexe-core dependency to enable crates.io publishing ([f6ff396](https://github.com/loonghao/shimexe/commit/f6ff39687f70a9e7db54b1779010d24ac35ea9cc))
* adjust performance test timeouts for Windows environment ([d0c4bd3](https://github.com/loonghao/shimexe/commit/d0c4bd379cb06d8e494edc7011a46e350a89883d))
* clean working directory by removing tracked build files ([c86db29](https://github.com/loonghao/shimexe/commit/c86db296a85af0b144e2e1db08b044b50d5d3702))
* complete unit test coverage enhancement ([42041ae](https://github.com/loonghao/shimexe/commit/42041ae25a30c55c4f551315f5be48d7259e720f))
* completely disable codecov status checks to prevent CI failures ([015597f](https://github.com/loonghao/shimexe/commit/015597f5f476566f0ff2c50f5d1fb3bba781e4df))
* completely rewrite release-please config for Rust workspace ([353eb0c](https://github.com/loonghao/shimexe/commit/353eb0c527363bb21bf39d48760b50897b383d3c))
* comprehensive GitHub Actions permissions and CI configuration ([8aaaa40](https://github.com/loonghao/shimexe/commit/8aaaa40609ab5d7ba92650c52dc706005822c87a))
* configure release-plz to show only current version changelog in releases ([2472b5a](https://github.com/loonghao/shimexe/commit/2472b5a2f3a224d463a4a709e936a6680d4feba9))
* correct install script URLs to use raw.githubusercontent.com ([83275ee](https://github.com/loonghao/shimexe/commit/83275ee03436808250eeeb8afb35dada966088f4))
* correct PATH usage and improve user guidance ([291763e](https://github.com/loonghao/shimexe/commit/291763ee24a8218cf9224d004a1b3e0ab3c57e4c))
* correct release tag format in install scripts ([de4ac68](https://github.com/loonghao/shimexe/commit/de4ac683a490c9260489dbc894b4224570971e34))
* correct test implementations to match actual API ([cc35da6](https://github.com/loonghao/shimexe/commit/cc35da6f63a535c616dcfde54db22d0da5ba6535))
* **deps:** update rust crate tokio to v1.46.1 ([e423b26](https://github.com/loonghao/shimexe/commit/e423b2658a1f36e52459ae267c65091411d14ad5))
* **deps:** update rust crate tokio to v1.47.0 ([8e186b9](https://github.com/loonghao/shimexe/commit/8e186b94affd1dbbe7a5de7fe00785e6d34cffd0))
* **deps:** update rust crate zip to v4 ([f7223d5](https://github.com/loonghao/shimexe/commit/f7223d5216305757045da9fc72d2cd131cb74f07))
* **deps:** update rust crate zip to v4.2.0 ([87bd01f](https://github.com/loonghao/shimexe/commit/87bd01f96e206e4518ff48b4e80214c21f73da94))
* enable release workflow triggering with PAT token ([807e0e9](https://github.com/loonghao/shimexe/commit/807e0e92029f2881d7702d24a5da606de9e52561))
* improve release workflow to handle existing releases ([be6a75e](https://github.com/loonghao/shimexe/commit/be6a75e6815447a86e23cadd6b7c4089d65ec9a1))
* improve version detection and release changelog formatting ([7003e09](https://github.com/loonghao/shimexe/commit/7003e09484d4668b2ec7e45e0b472d1ea66a9e27))
* **info:** avoid compile-time env! for TARGET/RUSTC_VERSION; use runtime fallbacks ([6970011](https://github.com/loonghao/shimexe/commit/69700114defefaf4942502d19afab6634c64100b))
* make codecov informational only, don't fail CI ([cc8dace](https://github.com/loonghao/shimexe/commit/cc8dace27bb764957552a11216c4a80324ee7489))
* optimize release workflow for tag-triggered releases ([78a27ff](https://github.com/loonghao/shimexe/commit/78a27ff5dccdd6be228db68914d3d69cece73bf6))
* optimize release workflow trigger conditions ([749406c](https://github.com/loonghao/shimexe/commit/749406cc7c7a96d3fb2a0fa954a81a108d62f16d))
* optimize release-plz config for mixed workspace ([b466497](https://github.com/loonghao/shimexe/commit/b466497845c83f4075c4282439c527d08dace412))
* remove needless borrow in archive path handling ([874247a](https://github.com/loonghao/shimexe/commit/874247a8c5a9ec79c87bdcb056d3da9dc3ae5571))
* remove root directory from release-please configuration ([75ed065](https://github.com/loonghao/shimexe/commit/75ed06561bb97660e41f52040c885cb99585b6d2))
* remove unsupported update_files from release-plz config ([1d917c4](https://github.com/loonghao/shimexe/commit/1d917c47eb3ce60ec8a0cfcdd5c5c84deeb70228))
* replace winget with chocolatey for ImageMagick installation ([c3fd017](https://github.com/loonghao/shimexe/commit/c3fd017ec05583851baa94ae7a816ebababf1a59))
* resolve all compilation errors and template syntax issues ([82122f5](https://github.com/loonghao/shimexe/commit/82122f5161e6c253497a55604fe49563f57532a5))
* resolve CI configuration and testing issues ([8754f55](https://github.com/loonghao/shimexe/commit/8754f553a9ce9d601be7713ab02f0b9aa06c9628))
* resolve clippy and rustdoc warnings for CI compatibility ([a1acc7b](https://github.com/loonghao/shimexe/commit/a1acc7b51892c356d964ac21fe99bec6ecadc24f))
* resolve clippy warnings and compilation errors ([ee01ee3](https://github.com/loonghao/shimexe/commit/ee01ee3c161c9b41df0764e69f039eceb4f861a9))
* resolve clippy warnings and implement Default trait for ConfigCache ([e6641b3](https://github.com/loonghao/shimexe/commit/e6641b3a1a59e41e0f97454bfd32e2fc53f8bcc0))
* resolve clippy warnings in test files ([1d4196b](https://github.com/loonghao/shimexe/commit/1d4196bca81362410336254c97960b1bfc1d2183))
* resolve compilation errors in test files ([ebf65ec](https://github.com/loonghao/shimexe/commit/ebf65ecb6ab39a90f09364891c7ff5a547b67868))
* resolve compilation errors in tests ([2ea6753](https://github.com/loonghao/shimexe/commit/2ea6753c90dd36622c2b1306f19b5e93c683031c))
* resolve duplicate release issues and improve Chocolatey publishing ([f7286cb](https://github.com/loonghao/shimexe/commit/f7286cb18807fcabe7246494fa7d5fa821bd4cc9))
* resolve git detached HEAD issue in release-plz dry-run ([e09a7df](https://github.com/loonghao/shimexe/commit/e09a7df4fa74964faf9f4ba1bfca1136b2bffcae))
* resolve Git merge conflict markers in release-please manifest ([1a082cd](https://github.com/loonghao/shimexe/commit/1a082cdc8da7f2de63e0a45e6d33173fa25c82d4))
* resolve GitHub Actions permission issue for release-please ([bbcd794](https://github.com/loonghao/shimexe/commit/bbcd794f83cc5029b7c16e3d8539dbb5b38e3c48))
* resolve OpenSSL cross-compilation issues with rustls ([9c11fab](https://github.com/loonghao/shimexe/commit/9c11fabbb25906dddc34d2b71db4ad3c9386b502))
* resolve package manager publishing issues ([1f53f43](https://github.com/loonghao/shimexe/commit/1f53f43681e4ac6489098b3e895911b264d27759))
* resolve release-please configuration issues ([5875b0d](https://github.com/loonghao/shimexe/commit/5875b0d39d90e8c187916bc89eaf9712d8cd0bb8))
* resolve release-plz config and clippy warnings ([409d2e3](https://github.com/loonghao/shimexe/commit/409d2e3576424de45fd9f580d44e8ffcd90bd885))
* resolve release-plz configuration issues ([6955e44](https://github.com/loonghao/shimexe/commit/6955e4487ba81e533bfbd9654086dab215ead0fa))
* resolve remaining clippy uninlined format args warnings ([26266bb](https://github.com/loonghao/shimexe/commit/26266bb516907fab760f793b5103827e83e11f0d))
* resolve remaining redundant_closure warning in runner.rs ([e22b419](https://github.com/loonghao/shimexe/commit/e22b419b0bce0077bcf03a7033b6d61be472edd8))
* resolve TOML serialization test failure ([4a99cf7](https://github.com/loonghao/shimexe/commit/4a99cf7a05a6c1406beaa1a5d6307a3f9bccfd98))
* resolve utils test failures for environment variables ([cea8746](https://github.com/loonghao/shimexe/commit/cea874653aca5868a6cc8de62ee2f0bf65ed2390))
* resolve version detection issues in install scripts and update Scoop manifest ([054774a](https://github.com/loonghao/shimexe/commit/054774ae09a587ba22b1223c0465e0bc56415898))
* resolve workflow conflicts between release.yml and update-packages.yml ([5929ba6](https://github.com/loonghao/shimexe/commit/5929ba63fda95059409e1dd3e37bde94aa6b47c4))
* set explicit version for root package to resolve cargo-workspace issue ([900c6f7](https://github.com/loonghao/shimexe/commit/900c6f7e1df88a7cde90cae40ce10fa9afb526d0))
* set explicit version for shimexe-core to resolve cargo-workspace issue ([304e13b](https://github.com/loonghao/shimexe/commit/304e13b1a6b66153e7b538b3c3c665b1bcaf8c4b))
* support both v* and shimexe-v* tag formats in install scripts ([f8ea896](https://github.com/loonghao/shimexe/commit/f8ea89629000758d22e70b238d46e86f2dc2ce71))
* update README_zh.md and fix install script file naming ([dcdc637](https://github.com/loonghao/shimexe/commit/dcdc6370ecbd389c86cecee4fb7b1caa2039b0bd))
* update release-please configuration to resolve workflow issues ([bfcceb3](https://github.com/loonghao/shimexe/commit/bfcceb3d4d4f54a21c1bc0ff68b1824e44b30036))
* update release-plz configuration and add dry-run checks ([07d3a92](https://github.com/loonghao/shimexe/commit/07d3a9244ec3d30f3c62f4cf4a8e18c29892ee2a))
* update release-plz configuration for standard tag format ([479177c](https://github.com/loonghao/shimexe/commit/479177c1484d21a552852fd3f23906fb615cb4cb))
* update workspace license to MIT only ([0558a7b](https://github.com/loonghao/shimexe/commit/0558a7ba54b1c32c6d4b52fb03a242aeba1f0308))
* use official release-plz GitHub Action ([3cd74d0](https://github.com/loonghao/shimexe/commit/3cd74d083cc194da7ddb96c77ed3bad2f53b0519))


### Performance Improvements

* optimize CI/CD workflows and fix GitHub API auth ([0b2131b](https://github.com/loonghao/shimexe/commit/0b2131b3e37563860153a63fc6e5bbcff3839822))


### Code Refactoring

* move GitHub token validation to CI workflow ([6676aef](https://github.com/loonghao/shimexe/commit/6676aef8c8632af8343cc67db361e761a2650cf2))
* move release-plz dry-run to CI workflow ([bdb71a1](https://github.com/loonghao/shimexe/commit/bdb71a1427225c91104982227a01b3c726e14e0b))
* move shimexe-cli to root src directory ([d533ec6](https://github.com/loonghao/shimexe/commit/d533ec6535e9393cbe4ba949443fa2db54839f65))
* optimize dependency installation with unified approach ([8fde233](https://github.com/loonghao/shimexe/commit/8fde233d8d8dd966949efcc2b94d8cdd63fb0ba0))
* rename [@pkg](https://github.com/pkg) to pkg directory and fix code quality ([e3b0da8](https://github.com/loonghao/shimexe/commit/e3b0da89c2042d1ab60cace62352c96d327e830b))
* reorganize CI workflow responsibilities ([edeab8c](https://github.com/loonghao/shimexe/commit/edeab8c456e122ed71c177049d099c00df687f81))
* reorganize package manager files to [@pkg](https://github.com/pkg) directory ([4baa079](https://github.com/loonghao/shimexe/commit/4baa079fb660c96675d969d8da36404fd27a43b3))
* simplify PR validation to config-only check ([540333b](https://github.com/loonghao/shimexe/commit/540333b801218e2ceb6b049408a55cfc90bedcc3))


### Documentation

* update README and add comprehensive unit tests ([c35b70d](https://github.com/loonghao/shimexe/commit/c35b70df7102a0d9fdae9394e10ec8aa13967109))

## [0.5.6](https://github.com/loonghao/shimexe/compare/v0.5.5...v0.5.6) - 2025-08-04

### Added

- integrate automated package manager publishing
- integrate automated package manager publishing
- *(windows)* hide console window when running shims

### Fixed

- remove unsupported update_files from release-plz config
- resolve release-plz configuration issues
- *(deps)* update rust crate tokio to v1.47.0

## [0.5.5](https://github.com/loonghao/shimexe/compare/shimexe-v0.5.4...shimexe-v0.5.5) - 2025-07-06

### Fixed

- completely disable codecov status checks to prevent CI failures

## [0.5.4](https://github.com/loonghao/shimexe/compare/shimexe-v0.5.3...shimexe-v0.5.4) - 2025-07-06

### Fixed

- make codecov informational only, don't fail CI
- resolve package manager publishing issues

### Other

- *(deps)* update peter-evans/create-pull-request action to v7

## [0.5.3](https://github.com/loonghao/shimexe/compare/shimexe-v0.5.2...shimexe-v0.5.3) - 2025-07-06

### Fixed

- optimize release-plz config for mixed workspace

## [0.5.2](https://github.com/loonghao/shimexe/compare/shimexe-v0.5.1...shimexe-v0.5.2) - 2025-07-06

### Added

- enhance package publishing and fix env config

## [0.5.1](https://github.com/loonghao/shimexe/compare/shimexe-v0.5.0...shimexe-v0.5.1) - 2025-07-05

### Fixed

- resolve workflow conflicts between release.yml and update-packages.yml

## [0.5.0](https://github.com/loonghao/shimexe/compare/shimexe-v0.4.0...shimexe-v0.5.0) - 2025-07-05

### Added

- disable clippy uninlined_format_args lint
- upgrade turbo-cdn to 0.4.3 and improve verbose logging control

### Fixed

- resolve clippy and rustdoc warnings for CI compatibility

### Other

- *(deps)* update crazy-max/ghaction-chocolatey action to v3.4.0

## [0.4.0](https://github.com/loonghao/shimexe/compare/v0.3.5...v0.4.0) - 2025-06-23

### Added

- enhance auto-update with turbo-cdn integration
- add multi-platform package management and enhanced badges
- integrate turbo-cdn and restructure tests

### Fixed

- resolve CI configuration and testing issues

### Other

- rename @pkg to pkg directory and fix code quality
- reorganize package manager files to @pkg directory

## [0.3.5](https://github.com/loonghao/shimexe/compare/v0.3.4...v0.3.5) - 2025-06-19

### Other

- *(deps)* update rust crate thiserror to v2

## [0.3.4](https://github.com/loonghao/shimexe/compare/v0.3.3...v0.3.4) - 2025-06-18

### Fixed

- support both v* and shimexe-v* tag formats in install scripts
- improve version detection and release changelog formatting
- configure release-plz to show only current version changelog in releases
- resolve version detection issues in install scripts and update Scoop manifest

## [0.3.3](https://github.com/loonghao/shimexe/compare/v0.3.2...v0.3.3) - 2025-06-18

### Added

- add comprehensive vx integration example
- add high-level ShimManager API and update Chinese README
- enhance README with logo and improve vx integration

### Fixed

- resolve clippy warnings and compilation errors
- correct PATH usage and improve user guidance
- add retry logic and fallback for GitHub API rate limits

## [0.3.2](https://github.com/loonghao/shimexe/compare/v0.3.1...v0.3.2) - 2025-06-17

### Fixed

- add retry logic to update-packages workflow

### Other

- reorganize CI workflow responsibilities

## [0.3.1](https://github.com/loonghao/shimexe/compare/v0.3.0...v0.3.1) - 2025-06-17

### Fixed

- correct release tag format in install scripts
- update README_zh.md and fix install script file naming
- correct install script URLs to use raw.githubusercontent.com
- resolve duplicate release issues and improve Chocolatey publishing

### Other

- update chocolatey-action to latest version v3.3.0

## [0.3.0](https://github.com/loonghao/shimexe/compare/v0.2.1...v0.3.0) - 2025-06-17

### Added

- add comprehensive archive support and package management

### Fixed

- *(deps)* update rust crate zip to v4
- remove needless borrow in archive path handling

### Other

- *(deps)* update softprops/action-gh-release action to v2

## [0.2.1](https://github.com/loonghao/shimexe/compare/v0.2.0...v0.2.1) - 2025-06-17

### Fixed

- resolve clippy warnings and implement Default trait for ConfigCache

## [0.2.0](https://github.com/loonghao/shimexe/compare/v0.1.4...v0.2.0) - 2025-06-17

### Added

- enhance HTTP URL support with persistent download tracking
- add HTTP URL support for automatic executable download

### Fixed

- optimize release workflow trigger conditions

### Other

- move GitHub token validation to CI workflow
- optimize CI/CD workflows and fix GitHub API auth
- update README and add comprehensive unit tests

## [0.1.4](https://github.com/loonghao/shimexe/compare/v0.1.3...v0.1.4) - 2025-06-17

### Added

- enhance CI workflow with cross-compilation testing
- enhance release workflow with advanced cross-compilation
- simplify release workflow with actions-rust-release

### Fixed

- resolve OpenSSL cross-compilation issues with rustls
- replace winget with chocolatey for ImageMagick installation
- enable release workflow triggering with PAT token

### Other

- optimize dependency installation with unified approach

## [0.1.3](https://github.com/loonghao/shimexe/compare/v0.1.2...v0.1.3) - 2025-06-17

### Added

- enhance release-plz validation with version update check
- modernize GitHub Actions to latest versions

### Fixed

- use official release-plz GitHub Action
- add git-token parameter to release-plz dry-run
- resolve git detached HEAD issue in release-plz dry-run
- update release-plz configuration and add dry-run checks
- improve release workflow to handle existing releases
- update release-plz configuration for standard tag format

### Other

- simplify PR validation to config-only check
- move release-plz dry-run to CI workflow
- move shimexe-cli to root src directory

## [0.1.2](https://github.com/loonghao/shimexe/compare/shimexe-v0.1.1...shimexe-v0.1.2) - 2025-06-16

### Added

- implement standalone shim execution with local config lookup

## [0.1.1](https://github.com/loonghao/shimexe/compare/shimexe-v0.1.0...shimexe-v0.1.1) - 2025-06-16

### Added

- setup release-please automation and add crate READMEs

## [0.1.0](https://github.com/loonghao/shimexe/releases/tag/shimexe-v0.1.0) - 2025-06-16

### Added

- enhance shim configuration with dynamic template system and improved args handling
