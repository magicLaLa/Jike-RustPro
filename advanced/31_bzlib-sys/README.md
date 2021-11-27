-  在 windows for linux 运行下会方便许多
  - 问题：
    - 需要更新 packages: `sudo apt-get update`
    - 安装 `build-essential`，用来解决 `linker '`cc`' not found`
    - 报错：` thread 'main' panicked at 'Unable to find libclang: "couldn't find any valid shared libraries matching: ['libclang.so', 'libclang-*.so', 'libclang.so.*', 'libclang-*.so.*'], set the `LIBCLANG_PATH` environment variable to a path where one of these files can be found (invalid: [])"'`
    - 安装：`sudo apt-get install libclang-dev`
    - 发现报错，未找到 `wrapper.h:1:10: fatal error: 'bzlib.h' file not found`
    - 安装：`sudo apt-get install libboost-all-dev`、`sudo apt-get install libbz2-dev`
    - 运行 `cargo test` 报错：`linking with `cc` failed: exit status: 1`
    - 尝试设置 `code  ~/.cargo/config`

      ```
        [target.'cfg(target_os = "linux")']
        rustflags = ["-C", "link-arg=-nostartfiles"]

        # [target.x86_64-apple-darwin]
        # rustflags = [
        #   "-C", "link-arg=-undefined",
        #   "-C", "link-arg=dynamic_lookup",
        # ]

        # [target.aarch64-apple-darwin]
        # rustflags = [
        #   "-C", "link-arg=-undefined",
        #   "-C", "link-arg=dynamic_lookup",
        # ]
      ```
    - 继续运行 `cargo test`
    - 报错:

      ```sh
        error: failed to run custom build command for `memchr v2.4.1`
        Caused by:
          process didn't exit successfully: `/mnt/e/Github/Jike-RustPro/target/debug/build/libc-b15f646c8f9c2428/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
        warning: build failed, waiting for other jobs to finish...
      ```
      - gg ~，还是用原生 Linux 或者 macos 吧 ~ 累了
