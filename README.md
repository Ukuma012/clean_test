# Clean Architecture Test

## healthcheck

- curl -v http://localhost:8787/api/v1/healthcheck

- curl -v -X POST http://localhost:8787/api/v1/healthcheck/login

- curl -v -X POST http://localhost:8787/api/v1/healthcheck/add --cookie actix-session=dXY6NUEOzihwDj06aSjNdkezeMlU91JKSZdakMx0J6Q=YHRwDwTybv3jKF2y9PqTmDAdaiuRaLNU

## invitation token

- curl -X POST -v http://localhost:8787/api/v1/register/invitation/ --header 'content-type: application/json' --data '{"email":"name@domain.com"}'

## complete registration

- curl -X POST -v http://localhost:8787/api/v1/register/complete/c5e01578-1f2b-4260-b250-e9e5abaa492b \
  --header 'content-type: application/json' \
  --data '{"email":"name@domain.com", "password":"password"}'

# API 仕様に書くべきこと

- 提供するサービスの概要の説明
- 提供する動作の正常な動作＋引数や戻り値の説明
- エラーの説明
  - パラメータの不正値
  - 〇〇の順序で呼び出されなければならない + 実際に誤った順序で呼び出された場合に返されるエラー
  - 認証・認可のエラー

# 疑問

- uuid や chrono を使わないと、invitations の entity を表現できないけれど、外部のパッケージに依存することになる？
  他にやり方を思いつかないので、practical にやっていく
- infrastructure 層と adapter 層が分離されていない。adapter 層が実質 infrastructure 層を担っている。infrastructure 層が意味をなしていない。

# TODO

- カスタムエラーの取り扱いについて考える
- main, infrastructure/server が複雑なので、もっと簡単にできる
- workspace について check
- user_id を session に格納する。validation の実装

# 発見

- この実装では、外部(db, http, email service)とのやりとりは adapters の api と spi で行なっている
- サーバーが再起動すると、cookie が無効になる
