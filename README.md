## buck2-build-rust-feature

```bash
buck2 run //:rust-features                             
File changed: root//config.bzl
File changed: root//README.md
Build ID: b28ffbd2-3244-453f-80fc-ffab59e4ee2c
Loading targets.   Remaining     0/1                                                                                                                                                                          56 dirs read, 1 targets declared
Analyzing targets. Remaining     0/50                                                                                                                                                                         35 actions, 64 artifacts declared
Executing actions. Remaining     0/7                                                                                                                                                                          0.8s exec time total
Command: run.      Finished 1 local                                                                                                                                                                                                                                                
Time elapsed: 0.8s
BUILD SUCCEEDED
Starting RUN of `//:rust-features`
Running defined output located at: `/Users/eli/GitMono/buck2-build-rust-feature/buck-out/v2/gen/root/200212f73efcd57d/__rust-features__/rust_features`
Hello, world!
```

```bash
buck2 run //:rust-features --config rust.features=debug
Build ID: daf63b18-34a0-4a1d-80cc-da10e64f7983
Loading targets.   Remaining     0/1                                                                                                                                                                          1 targets declared
Analyzing targets. Remaining     0/1                                                                                                                                                                          35 actions, 64 artifacts declared
Executing actions. Remaining     0/5                                                                                                                                                                          0.4s exec time total
Command: run.      Finished 1 local                                                                                                                                                                                                                                                
Time elapsed: 0.4s
BUILD SUCCEEDED
Starting RUN of `//:rust-features`
Running defined output located at: `/Users/eli/GitMono/buck2-build-rust-feature/buck-out/v2/gen/root/200212f73efcd57d/__rust-features__/rust_features`
Debug mode is enabled!
Hello, world
```

```bash
buck2 run //:rust-features --config rust.features=debug,release
File changed: root//README.md
Build ID: a6c85fd0-8e8e-44eb-9a60-48c9223d41e1
Loading targets.   Remaining     0/1                                                                                                                                                                          1 targets declared
Analyzing targets. Remaining     0/1                                                                                                                                                                          35 actions, 64 artifacts declared
Executing actions. Remaining     0/5                                                                                                                                                                          0.4s exec time total
Command: run.      Finished 1 local                                                                                                                                                                                                                                                
Time elapsed: 0.4s
BUILD SUCCEEDED
Starting RUN of `//:rust-features`
Running defined output located at: `/Users/eli/GitMono/buck2-build-rust-feature/buck-out/v2/gen/root/200212f73efcd57d/__rust-features__/rust_features`
Debug mode is enabled!
Release mode is enabled!
Hello, world!
```