# Changelog

All notable changes to this project will be documented in this file.


## [Unreleased]



### Chore

- update CHANGELOG [skip ci] (c0bb6e7)


## [v0.1.112]



### Chore

- update CHANGELOG [skip ci] (58b3bd8)

- bump Cargo toml version (3c9ec9c)

- update CHANGELOG [skip ci] (c1dd6d8)

- update documentation for v0.1.111 [skip ci] (f44f599)


## [v0.1.111]



### Chore

- update CHANGELOG [skip ci] (40fbae0)

- update documentation for v0.1.43 [skip ci] (7b4063a)

- update CHANGELOG [skip ci] (d19305a)

- added Clone and and Debug Derivatives to arg parsers (8c780e0)

- updated version to 0.1.5 (70d996f)

- bumped version to 0.1.10 (8d95552)


### Feat

- made add, list, preview submodules public to all resource modules (dd12371)


### Refactor

- added routing  files. To enable <command> <resource>  <arg> , type of CLI (e9fa82f)

- Migrated from gh-templates => gitforge (177997e)

- updated tests to match the new gitforge  syntax and pattern (83382a0)

- migrate Python wheel builds to maturin (0c81959)

- rename project from 'gitforge' to 'gitcraft' (001288e)

- broke down build.yml to different yml files. (68c01c3)


## [v0.1.43]



### Chore

- update CHANGELOG [skip ci] (deb09bb)

- update documentation for v0.1.42 [skip ci] (3c38d14)

- update CHANGELOG [skip ci] (8b3c562)


### Fix

- standardize the format template (adbff58)


### Refactor

- made contribution guidelines more clearer in CONTRIBUTING.md (3ff9ce7)


## [v0.1.42]



### Chore

- update CHANGELOG [skip ci] (9283431)

- update documentation for v0.1.41 [skip ci] (e1e3f6e)


### Feat

- (version) bumped version to 0.1.42 (3a07769)


## [v0.1.41]



### Chore

- update CHANGELOG [skip ci] (2f590a2)

- update documentation for v0.1.40 [skip ci] (2b9e4e0)


### Finx

- npm configuration for build workflow (2fa77aa)


## [v0.1.40]



### Chore

- update CHANGELOG [skip ci] (caf9c53)

- update documentation for v0.1.39 [skip ci] (608f2ad)

- update CHANGELOG [skip ci] (4faee01)

- update documentation for v0.1.40 [skip ci] (e2892c7)

- remove emoji from exception raised in npm (483bbba)

- update CHANGELOG [skip ci] (6c4f365)

- update documentation for v0.1.40 [skip ci] (2c1e855)


### Feat

- (version) Updated Cargo.toml to version 0.1.40 (33a5384)


### Fix

- gitignore template names with the '.gitignore' to be parsed correctly.\n\nInitially we used to allow only the template name without the the ext though some users may pass the '.gitignore' ext. It shouldn't fail but be parsed correctly\n (26d5b34)


### Refactor

- used a more idiomatic rust pattern. related with normalization of the gitignore template name with ext (6031f50)


### Test

- added tests to verify fetching, previewing and adding of templates with .gitignore is succesful. (05a4439)


## [v0.1.39]



### Chore

- update CHANGELOG [skip ci] (4ef13a7)

- update documentation for v0.1.38 [skip ci] (16f7649)

- bump version to 0.1.39 (a30421d)


## [v0.1.38]



### Chore

- update CHANGELOG [skip ci] (b50b7f4)

- update documentation for v0.1.37 [skip ci] (2911756)

- update CHANGELOG [skip ci] (9d208df)


### Feat

- replaced use of emojis with react svg icons && bump package version in toml to 0.1.38 (7cfe98d)


## [v0.1.37]



### Chore

- update CHANGELOG [skip ci] (988f20c)

- update documentation for v0.1.36 [skip ci] (8c8bd0a)


### Fix

- problem with uploading new bin && bump version to 0.1.37 (3c5d76c)


## [v0.1.36]



### Chore

- update CHANGELOG [skip ci] (e32fef1)

- update documentation for v0.1.35 [skip ci] (5fde181)


### Fix

- broken links && bump tool version in cargotoml 0.1.36 (aab9021)


## [v0.1.35]



### Chore

- update CHANGELOG [skip ci] (83b2dda)

- update documentation for v0.1.34 [skip ci] (9a1171d)


## [v0.1.34]



### Chore

- update CHANGELOG [skip ci] (ee99247)

- update documentation for v0.1.33 [skip ci] (9f24047)

- Cargo toml app version to 0.1.34 (71e9587)


## [v0.1.33]



### Chore

- update CHANGELOG [skip ci] (51bfd59)

- update documentation for v0.1.32 [skip ci] (4de0f81)

- update Cargo.lock (3cc5780)


### Docs

- removed toggling between light and dark theme && bump version of Cargo toml (4d7a8d4)


## [v0.1.32]



### Chore

- update CHANGELOG [skip ci] (72f6356)

- update documentation for v0.1.31 [skip ci] (6e4d6da)

- updated docs readme && bump cargo toml version (ecf9f64)


## [v0.1.31]



### Chore

- update CHANGELOG [skip ci] (8c9ebaf)

- update documentation for v0.1.30 [skip ci] (0e4147c)


### Fix

- add manual trigger and changes to docs/ on docs deployment and bump version to 0.1.31 (ea5da1f)


## [v0.1.30]



### Chore

- update CHANGELOG [skip ci] (3005962)

- update documentation for v0.1.29 [skip ci] (95995f6)

- bump version in Cargo.toml (b95f9e7)


## [v0.1.29]



### Chore

- update CHANGELOG [skip ci] (10e96b6)

- update documentation for v0.1.28 [skip ci] (2f153c1)


### Fix

- deploy docs (removed deprecated version) && bumped version in toml (f86d604)


## [v0.1.28]



### Chore

- update CHANGELOG [skip ci] (0208876)

- update documentation for v0.1.27 [skip ci] (0e8eba4)

- added starter comments to pr templates (a90b5bf)


### Docs

- added docusaurus documentation (010ed25)

- added more supported platforms in installation md file && added package-lock json (fa23713)


### Feat

- added more pr templates (bb00fb8)

- added separator during preview (64f36d7)

- issue:add allow passing of template with ext (0f46367)

- issue:list:  pass file without ext (0063564)

- updated pr manifest.yml (154cfa4)

- license:add:all:: when downloading all licenses save with remote file name (fee6175)

- deploy docs workflow (5cea7d3)


### Fix

- removed misplaced LICENSE (9749bbe)


## [v0.1.27]



### Chore

- update CHANGELOG [skip ci] (6f8b40f)

- update documentation for v0.1.26 [skip ci] (34b7526)


### Fix

- removed MinGW build (a386535)


## [v0.1.26]



### Chore

- update CHANGELOG [skip ci] (8bd9fcf)

- update documentation for v0.1.25 [skip ci] (bebd445)


### Feat

- added 6 more platforms (bdc6a22)


## [v0.1.25]



### Chore

- update CHANGELOG [skip ci] (5d9333b)

- update documentation for v0.1.24 [skip ci] (8ff7414)

- bump cargo toml version (0d229f5)


## [v0.1.24]



### Chore

- update CHANGELOG [skip ci] (8811d4f)

- update documentation for v0.1.23 [skip ci] (1390df3)


### Fix

- homebrew cloning (93c827d)


## [v0.1.23]



### Chore

- update CHANGELOG [skip ci] (dc92a6a)

- update documentation for v0.1.22 [skip ci] (94f810a)


### Fix

- pypi npm homebrew package deployment (40421c4)


## [v0.1.22]



### Chore

- update CHANGELOG [skip ci] (6da4262)

- update documentation for v0.1.21 [skip ci] (bb98229)


### Fix

- pypi and npm publish (31bbdd5)


## [v0.1.21]



### Chore

- update CHANGELOG [skip ci] (85966c5)

- update documentation for v0.1.20 [skip ci] (68ce744)


### Fix

- publish to have a unified namespace and fixed npm wrapper (0113323)


## [v0.1.20]



### Chore

- update CHANGELOG [skip ci] (8004893)

- update documentation for v0.1.19 [skip ci] (6e60afb)


### Fix

- publish homebrew automated versioning and npm to handle global install correctly (6d497a7)


## [v0.1.19]



### Chore

- update CHANGELOG [skip ci] (4ae0c08)

- update documentation for v0.1.18 [skip ci] (1bb656a)


### Fix

- publish with ref to git cliff (0a642cc)


## [v0.1.18]



### Chore

- update CHANGELOG [skip ci] (f31ee48)

- update documentation for v0.1.17 [skip ci] (bfd94cc)


### Fix

- publish workflow (61ab339)


## [v0.1.17]



### Chore

- update CHANGELOG [skip ci] (b352965)

- update documentation for v0.1.16 [skip ci] (aeb40d7)

- bump version 0.1.17 (02b9aa9)


## [v0.1.16]



### Chore

- update CHANGELOG [skip ci] (5275894)

- update documentation for v0.1.15 [skip ci] (32d0ec6)

- bump version to 0.1.16 (be6b4d5)


## [v0.1.15]



### Chore

- update CHANGELOG [skip ci] (5c8b2c2)

- update documentation for v0.1.14 [skip ci] (16106a2)


### Fix

- shell errors in workflow (e207413)


## [v0.1.14]



### Chore

- update CHANGELOG [skip ci] (2b9729d)

- update documentation for v0.1.13 [skip ci] (4f8b638)


### Feat

- updated license name with the correct spdx id (e54a7e5)


## [v0.1.13]



### Chore

- update CHANGELOG [skip ci] (5cb58c3)

- update documentation for v0.1.12 [skip ci] (ad1f640)

- update cargo.toml  with meta description (29defb4)


### Feat

- added manual trigger for build (ea49c2c)

- added cargo version (da3b0bb)


## [v0.1.12]



### Chore

- update CHANGELOG [skip ci] (240d83b)

- update documentation for v0.1.11 [skip ci] (1b5025d)

- update CHANGELOG [skip ci] (ac2cdba)

- update CHANGELOG [skip ci] (0949f5e)


### Feat

- updated build yml to support publishing to different platforms (749598f)


## [v0.1.11]



### Chore

- update CHANGELOG [skip ci] (10ff7f1)

- update documentation for v0.1.10 [skip ci] (f48203c)


### Docs

- updated license test doc (2868a36)


### Feat

- support using filename and custom name (b1710dc)

- add output param (e79b7d4)

- added output arg in license subcommand (044319e)

- added output arg in pr subcommand (df22fe7)


### Fix

- added requires templates inside output arg (81dc582)

- centralized saved file logging inside file::save_file (0031a56)


### Test

- test_gitignore_add_valid_and_invalid_template (639fcc2)

- added tests to the output param (af9e48f)

- added output argument tests (a405394)

- added output arg license tests (a243e96)

- added output arg pr tests (6464a74)


## [v0.1.10]



### Chore

- update CHANGELOG [skip ci] (e80224f)

- update documentation for v0.1.9 [skip ci] (54cb113)


## [v0.1.9]



### Chore

- update CHANGELOG [skip ci] (dd9b592)

- update documentation for v0.1.8 [skip ci] (2cdb8dd)


### Fix

- delete LICENSE.Apache-2.0 (4b69eaf)


## [v0.1.8]



### Chore

- update CHANGELOG [skip ci] (c48d4ff)

- update documentation for v0.1.7 [skip ci] (4c6f08f)


### Rename

- LICENSE.Apache-2.0 => LICENSE (8def485)


## [v0.1.7]



### Chore

- update CHANGELOG [skip ci] (0e5b27a)

- update documentation for v0.1.6 [skip ci] (b9c1bc6)


### Refactor

- Apache license instead (cd293e4)


## [v0.1.6]



### Chore

- update CHANGELOG [skip ci] (3a9165b)


## [v0.1.5]



### Chore

- update CHANGELOG [skip ci] (432707e)

- update documentation for v0.1.2 [skip ci] (954f0d2)


## [v0.1.1]



### Chore

- update CHANGELOG [skip ci] (7257aa7)

- update documentation for v0.0.58 [skip ci] (2417dec)


## [v0.0.58]



### Chore

- update CHANGELOG [skip ci] (bcca655)


## [v0.0.57]



### Chore

- update CHANGELOG [skip ci] (5afc51d)


## [v0.1.0]



### Chore

- update CHANGELOG [skip ci] (ed83d80)


## [v0.0.55]



### Chore

- update CHANGELOG [skip ci] (04089ca)


## [v0.0.54]



### Chore

- update CHANGELOG [skip ci] (74cbd49)


## [v0.0.53]



### Chore

- update CHANGELOG [skip ci] (d75fe63)

- update README with the refactored  CLI design (a3b7bee)

- addded starter comment at manifest.yml (3436bd3)

- Update dependencies (a7050e2)

- prettify display output dir (189c74d)

- update dependency (fdedbac)

- update dependency (cabe808)

- updated readme with new commands (c31d307)

- replaced pull request template with the new one (e1abdfa)

- updated README with installation using installation scripts. (3203082)


### Docs

- added inline docs for `help` cmd (23377d0)


### Feat

- added cache gen utility (1c6b2b0)

- added gitignore add subcmd (eaf1d37)

- added gitignore (eda53fe)

- Cargo.lock (33f8c90)

- added list and preview subcommands in gitignore (aa7431f)

- add refactor issue template for improved code requests (76c01d0)

- add technical debt issue template for reporting and tracking (9cf2322)

- add security issue template for reporting vulnerabilities and data leaks (a69f6c9)

- add chore issue template for reporting tasks that need attention (7443507)

- remove security issue template from issue templates (59e39c5)

- added manifest.yml files to templates dir to correctly map the templates (a9764b4)

- license:list added --non-software (75ce2ce)

- added pattern matching util file (5197ac8)

- added manifest_naviagator in utility mod.rs (1a046d0)

- created append_file function (8157968)

- added --append flag (673bea9)

- automatic versioning of the tool (bb365f0)

- added checksum (--build-info) (bd39dc6)

- added chore, refactor, technical debt isssue templates (e6c7153)

- added installation scripts (5f72244)


### Feature

- added manifest navigator util file (5f49ee9)


### Fix

- correct type for 'steps' field in testing issue template (6111dc5)

- correct type for 'steps' field in testing issue template (9461ffe)

- enhance bug report template descriptions to include "_No Response_" option (a57e1f4)

- update community issue template to include "_No Response_" option in descriptions (f8edafa)

- update documentation issue template to clarify "_No Response_" option in descriptions (d95fb79)

- add "_No Response_" option to subcategory and description fields in DX feedback template (7501d50)

- update feature request template to clarify "_No Response_" option in subcategory and description fields (10f27fc)

- update support template to clarify "_No Response_" option in subcategory and question fields (e72155a)

- update testing & QA template to clarify "_No Response_" option in subcategory and description fields (ec41568)

- Pull request template: removed the non performative part of the syntax (c139f68)

- fully update CHANGELOG instead of prepending (3713193)

- auto version workflow - to correctly take the correct tag (c2c6a4f)

- remove color encodings inside string to use 'colored' lib (b276bd8)

- show all categories by default (6783745)

- license:mod.rs : a logical error on updating cache (fc9da40)

- pr:add:  correctly handled dir output on default template (88ddf51)

- updated default pr templates to be more generic (fc5e5a0)


### Refactor

- deprecated description field in bug issue template (5510bea)

- maintainable cli design (62cb312)

- removed the  custom parser (ebb0627)

- removed commonAddArgs in base (c86eada)

- gitignore::list removed complexities (serialization/deserialization) (82a160d)

- standardized python ci template structure to be replicable with other languages and frameworks (2201cfa)

- license add subcommand (66fce40)

- license list rs (f348f57)

- list.rs  and cache logic (0d6e34d)

- deprecated versioning in cache and increased verbosity in error handling of load_cache (b31e713)

- removed to be supported soon in utils/file.rs (667c439)

- improved licenses: used an alternative to Github API #107 (c420c7a)

- removed version in cache (b99c0e5)

- updated ensure gitignore cache to support update cache parameter (abc6959)

- manifest yml (c18924d)

- deprecated use of api to use manifest yml files (f919ae4)

- use manifest.yml instead of using github api (22af63f)

- download_all_templates fn to use manifest instead of Github api (bb10a8b)

- combined update changelog and readme version to release automation (566303d)


### Tests

- added CLI tests (b394f92)


## [v0.0.52]



### Chore

- update CHANGELOG [skip ci] (4eaed24)


### Fix

- pull_request_template removed first line comment (68d570f)

- PR template (4eea059)


### Refactor

- removed templates inside the repo (d646c49)

- removed emojis in PR template (de759eb)


## [v0.0.51]



### Chore

- update CHANGELOG [skip ci] (0ecad29)


### Feat

- added pr_template in the repo workflow (96d8399)


## [v0.0.50]



### Chore

- update CHANGELOG [skip ci] (1a35a2a)

- update CHANGELOG [skip ci] (e858214)


### Feat

- added pr template integration (b4c60d0)


### Refactor

- Adopt Clap Subcommand Pattern for Clean and Extensible CLI Design (3d7df1b)


## [v0.0.49]



### Chore

- update CHANGELOG [skip ci] (34e66fd)


### Feature

- specification of dir to write output (a560a18)


### Refactor

- main functions in cmds  will be plugin managers (ed406a6)

- added util function save_file (20c5ed5)


## [v0.0.48]



### Chore

- update CHANGELOG [skip ci] (26b911d)

- rename update_changelog.yml to update-changelog.yml for consistency (d6dda59)


### Feat

- add licenses (089d63b)

- #30 allow multiple downloads (e3b0e51)


### Fix

- surpress dead code warning in remote.rs (dc062fd)


### License

- issue.rs to a dir (16655c5)


### Refactor

- issue.rs to a dir (e0feebf)

- rm issue.rs and license.rs (a4cf38e)


## [v0.0.47]



### Chore

- update CHANGELOG [skip ci] (1935023)


### Fix

- update  remote.rs to be standard (df9248c)


## [v0.0.46]



### Chore

- update CHANGELOG [skip ci] (939628f)


### Feat

- add GitHub Actions workflow for publishing Python package to PyPI on release (9cf70d6)

- add GitHub Actions workflow for publishing Python package to TestPyPI (770d0f6)

- add GitHub Actions workflow to validate requirements.txt against lock files (4b9ab41)


## [v0.0.45]



### Restructure

- Organize CI templates into group-specific subfolders (2aadf62)


### Chore

- update CHANGELOG [skip ci] (7ef4982)

- update CHANGELOG [skip ci] (fd876fa)


### Feat

- add Flake8 GitHub Actions workflow for PEP8 compliance (b9a503d)

- add GitHub Actions workflow for Black code formatter (beb7f08)

- add GitHub Actions workflow for Ruff code linter and formatter (6ec904b)

- add GitHub Actions workflow for Pylint code quality check (c72c721)

- add GitHub Actions workflow for Mypy static type checking (022f0f8)

- add GitHub Actions workflow for Pyright type checking (52be534)

- add GitHub Actions workflow for Pytest testing (8f532ca)

- add GitHub Actions workflow for Unittest testing suite (fde7f81)

- add GitHub Actions workflow for Nose2 test suite (afa17a0)

- add GitHub Actions workflow for generating code coverage reports (528a5f1)

- add GitHub Actions workflow for uploading coverage reports to Codecov (8eded48)

- add GitHub Actions workflow for building Python packages with Poetry (f05362d)

- add GitHub Actions workflow for building documentation with MkDocs (bf133db)

- add GitHub Actions workflow for auto-formatting code with Black (6d04102)

- add GitHub Actions workflow for sorting and grouping imports with isort (1d4a216)

- add GitHub Actions workflow for building documentation with Sphinx (fabf852)

- add GitHub Actions workflow for building documentation with MkDocs (b21a6a0)

- add GitHub Actions workflow for Bandit security scan (150f405)

- add GitHub Actions workflow for Safety security scan (a2aef6b)

- add GitHub Actions workflow for pip-audit security scan (e3a8743)

- add GitHub Actions workflow for checking dependency updates (4d039e2)

- add GitHub Actions workflow for pip dependency check (8f32c28)

- add GitHub Actions workflow for matrix installability check across multiple Python/OS versions (619dbcf)

- add GitHub Actions workflow for running pre-commit hooks on Python files (e92382e)

- add GitHub Actions workflow for caching Poetry dependencies (c68d07d)

- add GitHub Actions workflow for caching pip dependencies (d894c1f)

- add GitHub Actions workflow for validating poetry.lock against pyproject.toml (87be0fd)


### Fix

- correct comment formatting in coverage.yml (5e85372)

- update Python version matrix to allow for flexible version specification (d3df8e3)

- update Ruff command to explicitly check code quality (829c847)

- update GitHub Actions workflow to build Python package using setuptools (d9f83d5)


## [v0.0.44]



### Chore

- update CHANGELOG [skip ci] (ab2c33b)

- update CHANGELOG [skip ci] (c2a3c27)


### Fix

- installation instructions: made it clearer (e29fd27)


## [v0.0.43]



### Fix

- syntax error in change log update workflow (8bf28b7)


## [v0.0.42]



### Chore

- update CHANGELOG [skip ci] (5c4f775)


### Fix

- added rebase on changelog update workflow  and readme update workflow (34e6205)


## [v0.0.41]



### Fix

- run: 16012346792 (fc00066)


## [v0.0.40]



### Chore

- update CHANGELOG [skip ci] (3fda110)


## [v0.0.39]



### Feat

- updated README:: piece of info to new contributors (0e2b1ae)


### Fix

- update README to read the latest version (add3290)

- enforced semantic versioning in build yml, updated changelog yml logic (87c1b23)

- run: 16012293293 (71d000c)


## [v0.0.38]



### Fix

- updated sed command (ed6bf68)


## [v0.0.37]



### Fix

- sed command to correctly update markdown text part (350a7f8)


## [v0.0.36]



### Fix

- handle the sed cmd correctly to correctly update the versions (0891bb1)


## [v0.0.34]



### Fix

- replace the version in both markdown and plain-text (ff6b9d1)


## [v0.0.33]



### Fix

- removed redundant v while giving versions (24c3aae)


## [v0.0.32]



### Fix

- update sed commands (62dee53)


## [v0.0.31]



### Fix

- Escape the forward slashes and parentheses properly in the sed pattern (3df872e)


## [v0.0.3]



### Fix

- both link part and displayed message part will be displayed (3c382e8)


## [v0.0.29]



### Fix

- added a sed command on account of a missing line (a36662c)


## [v0.0.28]



### Chore

- update CHANGELOG [skip ci] (991edf0)


### Feat

- enable dynamic update of version in  README (869aa23)


### Fic

- typo in git cliff toml  and workflow (ed73387)


### Fix

- changelog workflow  and updated git-cliff toml (1e399ca)

- unexpected command in git-cliff (472e8ec)

- git-cliff command: appended Changelog (6a4cc7e)

- renamed git-cliff to cliff storing changes to temp changelog (e668151)

- git ref branch name (f590ee6)

- trigger to happen only  on push (46e6c2e)

- typo in README (3979eb1)


## [v0.0.27]



### Feat

- update remote:list to fetch with comments (19e1b9c)


## [v0.0.26]



### Chore

- update CHANGELOG [skip ci] (ccd9627)

- update CHANGELOG [skip ci] (d0eb53c)

- update CHANGELOG [skip ci] (f1a6551)

- update CHANGELOG [skip ci] (79eedb4)

- update CHANGELOG [skip ci] (30d5991)

- update CHANGELOG [skip ci] (d02c35d)

- update CHANGELOG [skip ci] (dcf5059)

- update CHANGELOG [skip ci] (56b6a0c)


### Feat

- enhanced template files by adding appropriate tags to them (f71dedf)

- added test and workflow that verifies all templates start with a comment in the first line (b99ae07)

- updated the tests to capture all files\n\nUpdated the tests to capture all files instead of failing the first time it finds an invalid template\n (3e3b2ff)


### Fix

- untamper with MPL 2.0  License (391076a)

- space typo (08b2d4b)

- removed sha256 checksums from README (fc50455)

- updated labels to prevent only one being chosen (1bd173f)

- updated README to enable using repo labels (8d50a8b)

- added red triangle to easily spot the file (c905276)

- added comment to the first line of all our templates that were missing (c2ea0cf)


### Refactor

- renamed naming of licenses (9886d80)

- renamed issue templates and added first line as comment (52b85b0)

- renamed pr templates (c5022f6)

- updated the exising issue templates (978d353)


## [v0.0.22]



### Update

- version in README (86174d4)


### Chore

- update CHANGELOG [skip ci] (6c4f705)


### Fix

- refactor from git-templates to gh templates (398af77)


## [v0.0.21]



### Chore

- update CHANGELOG [skip ci] (fd1f611)


## [v0.0.18]



### Fix

- typo in License (7c0581d)


### Update

- Added License (893a227)

- Added Contribution guidelines and Code of Conduct (33561cc)

- Added my email in CodeOfConduct (c930dc0)

- inserted email inside Code Of Conduct (cafdff4)


### Fix

- refactor from git templates to gh templates (921fc5c)


## [v0.0.24]



### Update

- README, showing distribution for git-templates bin (39a409b)

- README to fix typo and maintain consistency in Doc (8c1a2e9)


### Feat

- introduced update change log in its own workflow with its own trigger (c6a6824)


## [v0.0.17]



### Fix

- avoid pushing  on tag ref (821152e)


## [v0.0.16]



### Fix

- added branch ref to allow commiting (5fe9cf1)


## [v0.0.15]



### Feat

- added use of git cliff to update CHANGELOG (8a48f39)


## [v0.0.14]



### Fix

- Build errors due to wrong ext appending (windows build) (0f9794a)


## [v0.0.13]



### Fix

- Upload assets due to permission errors (8bc6431)


## [v0.0.12]



### Fix

- SSL issues during build (c315928)


## [v0.0.11]



### Feature

- Added Issue template yml (7975e6a)

- Copied the Issue templates to .github (3a6b915)

- Add config yml file (e3e13d0)

- Added PR templates and CodeOfConduct template (b4d0542)

- Added Licenses (4b1e114)

- added issue cli (d822523)

- added build artifact yml workflow (9dffcab)


### Fix

- removed ':' inside bug report yml\n\ntrailing ':' was found under body => dropdown => options at Other validations\n (608ef3a)

- Moved required: true to validations (401b1b1)

- Remove typos (1529b51)

- Updated the remote functions to have better error handling when there is network error (c278694)


### Update

- Added README (f38eeef)

- Renamed the issue templates, Added Docs to README, Added issue template script installer (4e3cd70)

- from Github to Git templates && added default ext in  pretty print (5ed8a83)

Generated by git-cliff
