# Clean Architecture Test

## healthcheck

- curl -X GET http://localhost:8787/api/v1/healthcheck/

## invitation token

- curl -X POST http://localhost:8787/api/v1/register/invitation/ --header 'content-type: application/json' --data '{"email":"name@domain.com"}'

## complete registration

- curl --request POST --url http://localhost:8787/api/v1/register/complete/52d4c5e1-04ac-4f58-ab6e-e47c4ac475cc --header 'content-type: application/json' --data '{"user": {"email":"name@domain.com", "password":"password"}}'

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

# TODO

- error response を変更する。ApiResponse の方がわかりやすい

# 発見

- この実装では、外部(db, http, email service)とのやりとりは adapters の api と spi で行なっている
