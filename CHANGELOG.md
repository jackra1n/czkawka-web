# Changelog

## [1.2.0](https://github.com/jackra1n/czkawka-web/compare/1.1.0...1.2.0) (2026-06-19)


### Features

* better handle already deleted files ([#15](https://github.com/jackra1n/czkawka-web/issues/15)) ([531f172](https://github.com/jackra1n/czkawka-web/commit/531f172bdf0e9ec0ab97905608f41a712f444d36))
* improve exif scanner results view ([#13](https://github.com/jackra1n/czkawka-web/issues/13)) ([271f320](https://github.com/jackra1n/czkawka-web/commit/271f3203ec0494a6375aafbedcf258fab043b89e))

## [1.1.0](https://github.com/jackra1n/czkawka-web/compare/1.0.0...1.1.0) (2026-06-05)


### Features

* add curl dependency and healthcheck to Dockerfile ([#4](https://github.com/jackra1n/czkawka-web/issues/4)) ([920bda3](https://github.com/jackra1n/czkawka-web/commit/920bda34ad0a0c50a5bb3ee38572f0434db172cb))
* add deploy script for testing ([#7](https://github.com/jackra1n/czkawka-web/issues/7)) ([81ef6bf](https://github.com/jackra1n/czkawka-web/commit/81ef6bf40ebedf38530bdd22b395f5aecc588cff))
* add initial settings page with hardlink toggle ([#11](https://github.com/jackra1n/czkawka-web/issues/11)) ([e2eaa64](https://github.com/jackra1n/czkawka-web/commit/e2eaa643eb35ad6b1e6fed4033bcd676dbd2ba7f))
* implement linking files ([#10](https://github.com/jackra1n/czkawka-web/issues/10)) ([6def5a2](https://github.com/jackra1n/czkawka-web/commit/6def5a21bc41955ddaa626196f7b76da95ffd6c1))
* implement scan cancellation ([#8](https://github.com/jackra1n/czkawka-web/issues/8)) ([c90352f](https://github.com/jackra1n/czkawka-web/commit/c90352f49092d80bd40069522f5ec2040e3805c3))
* implement virtual list rendering for improved performance ([#5](https://github.com/jackra1n/czkawka-web/issues/5)) ([2c381f1](https://github.com/jackra1n/czkawka-web/commit/2c381f1f4dba64d3a17f5fc4d96eea9b4fede55b))
* make server port configurable via PORT environment variable ([#2](https://github.com/jackra1n/czkawka-web/issues/2)) ([08d4e68](https://github.com/jackra1n/czkawka-web/commit/08d4e683aab4ec072aba5436c6d92c92a563d4fb))


### Performance

* **docker:** optimize docker build process ([#9](https://github.com/jackra1n/czkawka-web/issues/9)) ([d2ec70d](https://github.com/jackra1n/czkawka-web/commit/d2ec70df83c8cccda34a00bbf3a210312fa22cb1))


### Refactoring

* adjust padding for scan results table and header components ([76926ca](https://github.com/jackra1n/czkawka-web/commit/76926ca5276fa6cfb54ee5e01dbb3074d534777b))
* adjust padding of buttons in scan config ([1ad4846](https://github.com/jackra1n/czkawka-web/commit/1ad4846d2091fbfd304d3e27d560ba8ba9109d2a))

## 1.0.0 (2026-05-20)


### Features

* add "~" path handling ([dc0ce97](https://github.com/jackra1n/czkawka-web/commit/dc0ce9798b389577dc4dbf0302414d32d5a94880))
* add api to list directories ([876ee4e](https://github.com/jackra1n/czkawka-web/commit/876ee4e57d46df1705a74f1ce275960a1f1b8748))
* add app state and ui persistence ([3aadf3a](https://github.com/jackra1n/czkawka-web/commit/3aadf3a97ec129cc6349cc91e709f9079304711a))
* add aria roles to scan results for a11y ([2f185b1](https://github.com/jackra1n/czkawka-web/commit/2f185b1f7efc00ea93ee6d2ec193a7f8eced381c))
* add bad extensions, bad names, exif remover ([13d7c31](https://github.com/jackra1n/czkawka-web/commit/13d7c312bbb5393365944683272ccd0751a7329a))
* add bad-extensions and bad-names frontend ([4e3c1f1](https://github.com/jackra1n/czkawka-web/commit/4e3c1f158cb5ead1647f189d7ba8793d86c08258))
* add basic logging and config path setup to backend ([e2fc1b5](https://github.com/jackra1n/czkawka-web/commit/e2fc1b5a13ce224382f2ec9dfa84b4f6563dbb64))
* add big-files scanner and config ([c4c2e13](https://github.com/jackra1n/czkawka-web/commit/c4c2e13c7a82ae5d8283353e8b6bc3ccb864729d))
* add broken files scanning support ([e22922f](https://github.com/jackra1n/czkawka-web/commit/e22922fee41c339ff1934bcae1557ae80b88d418))
* add broken-files frontend ([eda8436](https://github.com/jackra1n/czkawka-web/commit/eda843638dcd042c1f7e8d990a819fec7a34fce5))
* add collapsible config panel in scan config ([42c53da](https://github.com/jackra1n/czkawka-web/commit/42c53da5d279c7ed1a2466ffe1d0cecc20adb0b2))
* add color coding for compare ([e597cd4](https://github.com/jackra1n/czkawka-web/commit/e597cd43161cd5daa6311cc301760252472e8bf7))
* add delete files API and frontend ([4149386](https://github.com/jackra1n/czkawka-web/commit/4149386104e767609f4ca8638f9b6c5ccb215225))
* add docker task and docs for mise ([163ac39](https://github.com/jackra1n/czkawka-web/commit/163ac3949adb64fa2a16943d1f77936833ecad3c))
* add empty files scanner ([238ea8f](https://github.com/jackra1n/czkawka-web/commit/238ea8f340c11e709823a4295ce60173bf80da8e))
* add empty folders scanner ([a9f0087](https://github.com/jackra1n/czkawka-web/commit/a9f008780d3947009b1c1763112bcaf8b39a7d68))
* add entrypoint for non-root execution ([2bf2e74](https://github.com/jackra1n/czkawka-web/commit/2bf2e7401ca998a7fd8ea8273fc902cc2be9a90f))
* add fix action buttons and API to frontend ([585310c](https://github.com/jackra1n/czkawka-web/commit/585310caa846ff4dfe45daae20021cd70aec5f44))
* add fix endpoint for exif-remover, bad-names, and bad-extensions ([f7a5cd1](https://github.com/jackra1n/czkawka-web/commit/f7a5cd1d6b926e439b7711d21939df83e0d72492))
* add frontend for big files tool ([244848f](https://github.com/jackra1n/czkawka-web/commit/244848f69ae24f01cef440e5d8e3e11840b7daee))
* add frontend for empty files tool ([87dd108](https://github.com/jackra1n/czkawka-web/commit/87dd10879d0b85e5683917ab2058ffd6f7c724b0))
* add hidden file switch and fix path handling ([12f32b5](https://github.com/jackra1n/czkawka-web/commit/12f32b52fe90e01c30c2a8a9b2d38eb88cfa9069))
* add image similarity support ([bd92e93](https://github.com/jackra1n/czkawka-web/commit/bd92e931a66f7823922c51864376100941c3fcf0))
* add in-app file preview for various media and text types ([612f489](https://github.com/jackra1n/czkawka-web/commit/612f489033ad8b9a31f307b5279e6502690f7ac3))
* add initial rust project ([b647d38](https://github.com/jackra1n/czkawka-web/commit/b647d3860afa0b5cfdc36631264b6de3d816ac39))
* add initial simple backend for duplicate file scanning ([77925f6](https://github.com/jackra1n/czkawka-web/commit/77925f601a3051c7dc020d46bd4a6a388ce4c563))
* add initial simple frontend ([91f7d84](https://github.com/jackra1n/czkawka-web/commit/91f7d84f951451f131137fc82f543e217653bbdd))
* add initial SvelteKit project ([ca994fd](https://github.com/jackra1n/czkawka-web/commit/ca994fde00e23ffae46a3edd2f7df9556aa64b59))
* add invalid_symlinks scanner ([a0ad255](https://github.com/jackra1n/czkawka-web/commit/a0ad2552a122a7b8780e5a2cd884fa54e684ba7c))
* add invalid-symlinks tool UI ([116307e](https://github.com/jackra1n/czkawka-web/commit/116307e3cc86230b20ca7bc7fad72ac6f686964c))
* add lucide-svelte ([e73daec](https://github.com/jackra1n/czkawka-web/commit/e73daec2542d56503c51291806e704bf514be51f))
* add maximize and minimize for file comparison ([8b4978a](https://github.com/jackra1n/czkawka-web/commit/8b4978a2a99ee6f032b8a53e53a4d32690e89924))
* add mime_guess ([62dcecf](https://github.com/jackra1n/czkawka-web/commit/62dcecfbc4ec1bea6f1b8927f8f97a18534a2df4))
* add modified date to duplicate file data ([214bb42](https://github.com/jackra1n/czkawka-web/commit/214bb42cfa65fad58d218555f3943395158a2712))
* add multi-mode image compare UI ([56a1279](https://github.com/jackra1n/czkawka-web/commit/56a1279d5824c758d55b3e237a81617d71a61d78))
* add progress bar UI to scan results ([37b40e4](https://github.com/jackra1n/czkawka-web/commit/37b40e4532ef210a1b4d7aca90023bd0f4d73de4))
* add progress stage label mapping and channel helpers ([9f79930](https://github.com/jackra1n/czkawka-web/commit/9f79930c1980599e541fd7e04aaa3e1274bdd193))
* add reset to defaults for directories and tool settings ([9d0783c](https://github.com/jackra1n/czkawka-web/commit/9d0783cb0160efd7f1e427c49b17ba9283429877))
* add resizable columns to the scan results table ([dff11aa](https://github.com/jackra1n/czkawka-web/commit/dff11aac5bf20f8899246b4e2d32ecdadf5824b7))
* add resizable file preview panel ([c688059](https://github.com/jackra1n/czkawka-web/commit/c688059e122aa61cde77f3a862b5620789f26ee2))
* add same music scanner ([e474e33](https://github.com/jackra1n/czkawka-web/commit/e474e336aefc4dc1b3314db103ab60fd13fe8a91))
* add same-music scan tool frontend ([3b5fa78](https://github.com/jackra1n/czkawka-web/commit/3b5fa78fafbbfb0db67a023abc822624ffe9eb77))
* add scan actions and confirm modal ([331804a](https://github.com/jackra1n/czkawka-web/commit/331804a52fed5a67e9c295d0ab57dea729d9da60))
* add ScanProgress struct and wire into scan status response ([b90aaf6](https://github.com/jackra1n/czkawka-web/commit/b90aaf600b97358e994b8c62e0bd7c3dc79660fa))
* add ScanProgress type to frontend API client ([63065da](https://github.com/jackra1n/czkawka-web/commit/63065da76ebd81b772ce866cf85d5c3d4e729949))
* add similar-videos scanner ([6a981bd](https://github.com/jackra1n/czkawka-web/commit/6a981bdb36a2bc2ced65c7d66c1603499742ff73))
* add size field to scanned files ([26e76a8](https://github.com/jackra1n/czkawka-web/commit/26e76a8aa060b5298cd3e4864f6e02c74a4c23b5))
* add sorting for scan results ([60bc014](https://github.com/jackra1n/czkawka-web/commit/60bc01475efd23ed3fc06cdd714b591f864c9232))
* add support for excluded items and defaults ([660540e](https://github.com/jackra1n/czkawka-web/commit/660540e7d4fcd35c628bd1df60ba469e858139df))
* add temporary files scanner frontend ([a0122e6](https://github.com/jackra1n/czkawka-web/commit/a0122e67efe5c44bda301ccb52541e2b6c711f2a))
* add temporary scanner ([d621d18](https://github.com/jackra1n/czkawka-web/commit/d621d184c6a985e94066c16b88548ca80fd8f619))
* add video similarity tool frontend ([c3ffda9](https://github.com/jackra1n/czkawka-web/commit/c3ffda90809943affc6eea78a9e3cd32afb9ff3b))
* add vite proxy for api backend ([10ba705](https://github.com/jackra1n/czkawka-web/commit/10ba705571d6011c157760063eb2452db4676690))
* **backend:** add browser directory state and API endpoints ([74678f9](https://github.com/jackra1n/czkawka-web/commit/74678f94997fbf9b8578c7c8b310431d7c7e3ce3))
* **backend:** add default_directory to defaults ([9a6b259](https://github.com/jackra1n/czkawka-web/commit/9a6b25940a78c00043f0577cbdfaad6c7c3ccb56))
* create progress channel in scan execution ([d1b842f](https://github.com/jackra1n/czkawka-web/commit/d1b842f5103d3612fdea0160e88209b7f7646bb1))
* directory browser use home by default and remember last path ([014917e](https://github.com/jackra1n/czkawka-web/commit/014917e86b78ec296a41d3e469785ae8b15e1241))
* **docker:** add docker support and compose files ([ff70169](https://github.com/jackra1n/czkawka-web/commit/ff701697ed343b7adbfef4c23fd4bfd40bd35d15))
* **frontend:** add toggle to hide default excluded directories ([35f2aef](https://github.com/jackra1n/czkawka-web/commit/35f2aef695e42ec31cdb04ab67ab9293af6e085d))
* **frontend:** display dimensions in preview ([107c99e](https://github.com/jackra1n/czkawka-web/commit/107c99e5a84ca9c45bb0339ece12b78ad60240f6))
* improve directory selection ui and add flow ([2fb17dd](https://github.com/jackra1n/czkawka-web/commit/2fb17dd8c1d8d31dae90ec1e7f2a2bdd6619ec36))
* include scan progress in poll response ([63f9b1b](https://github.com/jackra1n/czkawka-web/commit/63f9b1b107bf0097c237c67454092d51fb4cd378))
* init last directory from defaults when unset ([6d67393](https://github.com/jackra1n/czkawka-web/commit/6d673935e8f9e43e9979d1a8f10cd5d7b3a24bca))
* make tool sidebar collapsible ([89956ed](https://github.com/jackra1n/czkawka-web/commit/89956ed475dac2e262640e1ba007b3e0fb5ba61c))
* pass progress sender to all 13 scanners ([ce05ed6](https://github.com/jackra1n/czkawka-web/commit/ce05ed622e154c95e28d8292f08f9e4145e4e188))
* persist file selections per tool ([8c66ee5](https://github.com/jackra1n/czkawka-web/commit/8c66ee5fa080c482a874843489beb27df8527f1e))
* rename same-music to music duplicates and reorder sidebar ([d1e28da](https://github.com/jackra1n/czkawka-web/commit/d1e28daaca21fafe94a8bdc63de2aebff359ffb4))
* replace manual file streaming with ServeFile ([d3b0635](https://github.com/jackra1n/czkawka-web/commit/d3b0635260b42525ff544dd77ede6f226f91dee9))
* **scan:** add select biggest and smallest ([cfc7ff7](https://github.com/jackra1n/czkawka-web/commit/cfc7ff7da3b15de4d9c5408fd8cb1b27a954d67c))
* serve frontend build from backend ([8874f91](https://github.com/jackra1n/czkawka-web/commit/8874f91c1b4a100c18e91c8f645e863b8bbaa4e2))
* set default volume for media on load ([2ed08bd](https://github.com/jackra1n/czkawka-web/commit/2ed08bd9f42f18bfef5ee53a8688c62ee260fde2))
* **state:** add persistent state and api ([3309ce9](https://github.com/jackra1n/czkawka-web/commit/3309ce9026f3bb8b61b7191c94ec67d71623b454))
* switch scan config to tabbed layout ([d531709](https://github.com/jackra1n/czkawka-web/commit/d53170958ac2dca40303dcb634498038a1c0374f))
* track scan progress state in page component ([ff94264](https://github.com/jackra1n/czkawka-web/commit/ff942643d82c4c258e6a7e59a30eaeea07091e00))
* use real api for directory browser modal ([f0d57c7](https://github.com/jackra1n/czkawka-web/commit/f0d57c7a6d77e129358097060598ac35fbb545ec))


### Bug Fixes

* add fetchVoid and use it for updates ([76adc6f](https://github.com/jackra1n/czkawka-web/commit/76adc6f7babeb17ab2942960996b643c25571841))
* **frontend:** add min-h-0 to layout containers ([31b6b3d](https://github.com/jackra1n/czkawka-web/commit/31b6b3decd6d7650a329f73e8df0a4ea87788eb1))
* initialize env_logger with default level ([bd9f643](https://github.com/jackra1n/czkawka-web/commit/bd9f64336a5c083391a5765d055c5d19bc78cf5a))
* only call set_config_cache_path once ([9918a28](https://github.com/jackra1n/czkawka-web/commit/9918a28476d86054ac7fe35989f2583ffb1d5a3b))
* remove volumes section from readme ([cacf8bb](https://github.com/jackra1n/czkawka-web/commit/cacf8bb113a562c6f45d5e033e88ef6451c14a98))
* side-by-side compare layout for tall images ([2ccd296](https://github.com/jackra1n/czkawka-web/commit/2ccd2962add6a827cbcabecace9ce18cd3a6ad5a))
* sync tool state with backend after updates ([9a38b39](https://github.com/jackra1n/czkawka-web/commit/9a38b39c4c26fc65052fc948f0e1255bc22e9969))
* use local vars for api updates in page ([1de24c8](https://github.com/jackra1n/czkawka-web/commit/1de24c896f98d1f997de41417c14ded57fa3820d))


### Refactoring

* **backend:** collapse nested if in get_browser_directory ([573d65d](https://github.com/jackra1n/czkawka-web/commit/573d65db334a7febd2213c29ca81af585701ec4d))
* **backend:** modularize handlers and add models ([7e31d66](https://github.com/jackra1n/czkawka-web/commit/7e31d6682ee0bfd60011de1745eb635dbbe591e1))
* cleanup leftover code related to not implemented tools ([0dc9360](https://github.com/jackra1n/czkawka-web/commit/0dc936004129671a45b9aeed81ef4d7c0521ed89))
* delegate scan to scanners ([f16cc31](https://github.com/jackra1n/czkawka-web/commit/f16cc319a8814ae0076ad10a4ff9b8a2c779695e))
* extract DirectoryList and ToolSettings, simplify +page.svelte ([faa0f76](https://github.com/jackra1n/czkawka-web/commit/faa0f7659a160ce9cb1824131bebd00139bc2cc2))
* format state handler code ([055c8aa](https://github.com/jackra1n/czkawka-web/commit/055c8aa284dade8853d08f026dfcda138ca68f2b))
* **frontend:** improve checkbox style and support label reversing ([64b78c3](https://github.com/jackra1n/czkawka-web/commit/64b78c381bfcfe56396066f7199083aee12485f3))
* **frontend:** make tooltip boundary-aware to prevent screen overflow ([cb86953](https://github.com/jackra1n/czkawka-web/commit/cb869533806701269fa7cd78c2e3e13d8613fb07))
* **frontend:** move custom UI components to dedicated ui subfolder ([2b87f2f](https://github.com/jackra1n/czkawka-web/commit/2b87f2f1feb720e783bc37cdfce946a9f204ec81))
* **frontend:** replace lastDirectory store with backend-driven directory ([5a2510a](https://github.com/jackra1n/czkawka-web/commit/5a2510a3928187568279fa985eaf5b78dc9e1e1c))
* **frontend:** unify checkboxes across scan results and compare settings ([263b52c](https://github.com/jackra1n/czkawka-web/commit/263b52c2a484b4a5af88df8459dbddaff2517bdd))
* improve include and exclude directories components ([255b411](https://github.com/jackra1n/czkawka-web/commit/255b4110e5fcde7062d4898ad02756d3178d137c))
* make stats bar sticky in scan results ([298f938](https://github.com/jackra1n/czkawka-web/commit/298f9381819bcc4e58185def6f916ce6f04943da))
* rename metrics to total_groups, total_items, wasted_bytes ([f1b5a07](https://github.com/jackra1n/czkawka-web/commit/f1b5a07ff1f6ff1d7d2d8f97583992f9f8b09080))
* replace dir inputs with spans ([1171623](https://github.com/jackra1n/czkawka-web/commit/117162369c14eb1cb18479bff86315fdb81d3fa5))
* show individual files in scan results with selection and arrow navigation ([bb950a3](https://github.com/jackra1n/czkawka-web/commit/bb950a3809a683ca8d0b183d8da5115fe6124534))
* simplify side-by-side compare layout ([af19bc4](https://github.com/jackra1n/czkawka-web/commit/af19bc49126de7c1dc203dc1671a6892f0bc9950))
* simplify side-by-side compare layout ([cc9b327](https://github.com/jackra1n/czkawka-web/commit/cc9b327e9542051887cc9cc607c6342661efa26e))
* split main page into smaller components ([ce5015c](https://github.com/jackra1n/czkawka-web/commit/ce5015ccf198353f4b3ed94f8098404ab2e6fa4a))
* streamline side-by-side compare layout ([bcc2da9](https://github.com/jackra1n/czkawka-web/commit/bcc2da9f58f12ebe5af36e9a5c360ef4812674a9))
* switch lastDirectory to getter/setter ([8ba018a](https://github.com/jackra1n/czkawka-web/commit/8ba018ae1c9fb191359029aca502379f1dc787e5))
* update lucide icons to current names ([5245950](https://github.com/jackra1n/czkawka-web/commit/524595024344ab4ec9536e8c1031aa621a228412))


### Styles

* add py-2 padding to similarity row ([49afc41](https://github.com/jackra1n/czkawka-web/commit/49afc41c3b52dc002e6e2ba03c18b335ac9f071e))
* improve tool settings layout for responsive wrap ([f58bd33](https://github.com/jackra1n/czkawka-web/commit/f58bd337da9a16cee5b58821d06264b7ff0d7555))
* improve truncation in scan results table and resizing ([3441e13](https://github.com/jackra1n/czkawka-web/commit/3441e13a621db100e3122813b25925709be30c78))
* lay out tool settings in two columns ([60c0bdb](https://github.com/jackra1n/czkawka-web/commit/60c0bdb2898cd9d2cc4c75d6569bdfe608598dbd))
* make sidebar buttons full width ([170727a](https://github.com/jackra1n/czkawka-web/commit/170727a10659efba562edb4f06a5aef9eefce0c7))
* make sidebar toggle button full width ([d21938c](https://github.com/jackra1n/czkawka-web/commit/d21938c06d95d8c57451b5e74ef6699b4ec4eaa9))
* refine tool sidebar spacing and sizing ([940c683](https://github.com/jackra1n/czkawka-web/commit/940c6839c654078bf8a2e852b86c9de764f5f70f))
* remove gap-2 around directory list container ([073323c](https://github.com/jackra1n/czkawka-web/commit/073323c9c1afd2963c92cc453dc6e8bcfd71532a))
* run formatter ([af4d446](https://github.com/jackra1n/czkawka-web/commit/af4d4464a1cc1418a2139cb052a6c4906fb25112))
* switch scan results colors to accent ([2c2903d](https://github.com/jackra1n/czkawka-web/commit/2c2903d00b03d137374ec97f545a40fb98b907a7))
* switch small text to xs and set base font ([265db93](https://github.com/jackra1n/czkawka-web/commit/265db93857c42aeeb6bed23013b3bfd68f6e7e9a))
* tidy backend formatting and imports ([d2e3a9b](https://github.com/jackra1n/czkawka-web/commit/d2e3a9b235080bb2a8335c4da6f878bf2b1276e2))
* update file preview resize handle ([7a4bfea](https://github.com/jackra1n/czkawka-web/commit/7a4bfeabedddd276e94ee9ddc20ae5df221148ad))


### Documentation

* add acknowledgment for czkawka ([fd4fc92](https://github.com/jackra1n/czkawka-web/commit/fd4fc9213244cd0a7f156725910fda35ab7f37e6))
* add AGPL v3 license ([78c6267](https://github.com/jackra1n/czkawka-web/commit/78c62675ab39f31b1d2357a14ad59ab2a8a3d205))
* add preview image ([5aab675](https://github.com/jackra1n/czkawka-web/commit/5aab675d4758576848150a75b3e81da95219e66c))
* remove important notice and add mkdir to compose section ([7de5675](https://github.com/jackra1n/czkawka-web/commit/7de5675bf9839c2fb04f57534b35c32c1811de67))
* replace preview image with webp ([b159f36](https://github.com/jackra1n/czkawka-web/commit/b159f3675a93e93a516686e2ca0c036ec9208fa5))
* simple readme.me ([da6d775](https://github.com/jackra1n/czkawka-web/commit/da6d7758328f742bc237a8fc07a1f8cef23a2de7))
* update czkawka web preview image ([4c10ddd](https://github.com/jackra1n/czkawka-web/commit/4c10dddaa6081fed1f7b8e9712b71566c01dfa8f))
* update project documentation with image previews and feature highlights ([0783e65](https://github.com/jackra1n/czkawka-web/commit/0783e653d77fb27fd390964a4bdb2241a3803317))
* update readme ([53a8221](https://github.com/jackra1n/czkawka-web/commit/53a8221dbca2842cdd27d85c948fb5db79f11d6e))
* update readme ([a518fbf](https://github.com/jackra1n/czkawka-web/commit/a518fbf1f4a7d4c1ff38ce51ac68472ca3281f6e))
* update readme to use prebuilt image ([8155d02](https://github.com/jackra1n/czkawka-web/commit/8155d02928fce6128d62be413975c830cb8c5ed6))


### Dependencies

* update frontend dependencies ([bacfabc](https://github.com/jackra1n/czkawka-web/commit/bacfabc99f09262cad0045e03156025a12986b46))


### Build

* add crossbeam-channel dependency for progress tracking ([70edff8](https://github.com/jackra1n/czkawka-web/commit/70edff81fe1c57d0cb815bfb9727ea99ea4e7aa5))
* set backend log level to info ([5f41d9d](https://github.com/jackra1n/czkawka-web/commit/5f41d9dab8a17efea98ad0049ce57e66ac1c5f1a))
* set rust log level in dockerfile ([8f8f3d3](https://github.com/jackra1n/czkawka-web/commit/8f8f3d3b8e3d40fcb84af5b2d09e0f0b353e2f45))


### CI/CD

* add ci and pr-title-lint workflows ([2d75740](https://github.com/jackra1n/czkawka-web/commit/2d75740c5500459d2329fd90fcaa29d18e7441d8))
* add manual docker build and push workflow ([5a9a76b](https://github.com/jackra1n/czkawka-web/commit/5a9a76ba2a181a7d3ade89e51c218b9d16427f36))
* add release-please config and workflows ([821607b](https://github.com/jackra1n/czkawka-web/commit/821607b93584359c9a52fcb8bca46bc788030dcc))
* update docker actions to latest ([c96fb86](https://github.com/jackra1n/czkawka-web/commit/c96fb864e0d9748515a144d234923519538cebe2))
