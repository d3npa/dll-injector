# dll-injector
A small Rust library for injecting DLLs into processes on Windows via LoadLibraryA

機能検証のためにいくつかのexampleを用意しました。

## How to use

単純にこの関数を呼び出せばいいのです。
```rust
pub fn inject_dll(target_pid: u32, dll_path: &str) -> Result<(), &str> {
```

例:
```rust
if let Err(e) = dll_injector::inject_dll(pid, &dll_path) {
    eprintln!(e);
    std::process::exit(1);
}
```

## 検証用DLL

検証用DLLイメージを作成するには次のコマンドを実行してください。
```
cargo build --example samplehook
```
ビルドの後、DLLイメージは `target/debug/example/samplehook.dll` に保存されます。

## 標的プログラム

単純な標的プログラムを用意しました。次のコマンドでビルドおよび実行できます。
```
cargo run --example infiniteloop
```
実行すると、PIDが表示され、無限ループに入ります。
このプロセスにDLLを注入することで新たな処理を行うことが可能になります。

## DLLインジェクション

DLLインジェクション（DLL注入）を行うためこのクレートを利用したバイナリを用意しました。
なお、コマンド引数を取りますので以下のように実行してください。
```
cargo run --example inject_by_pid target\debug\example\samplehook.dll $pid
```
※`$pid`の部分は先ほどinfiniteloopを実行したとき表示されたPIDを代入してください。

参考
- https://snoozy.hatenablog.com/entry/2019/12/22/195234
- http://epcnt19.hatenablog.com/entry/2017/12/10/000611
- https://github.com/darfink/detour-rs
- https://github.com/amcarthur/hammer
- https://docs.rs/winapi/0.3.9
