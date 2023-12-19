## actix-web を使ってみる

// TODO: 情報を整理する

### ゴール

- [x] actix-web を使ってみる
- [ ] ASP.NET Core Blazor WebAssembly をホストする
- [ ] コンテナ化する
- [ ] ECS にデプロイする

### actix-web を使ってみる

`actix_files` を利用することで静的ファイルを配信できる。

https://actix.rs/docs/static-files/

これを利用して Blazor WebAssembly をホストしてみる

### ASP.NET Core Blazor WebAssembly をホストする

`dotnet new blazorwasm -o aspnetwasm` で Blazor WebAssembly のプロジェクトを作成する。

`dotnet publish -c Release -o ../.` でビルドする。
