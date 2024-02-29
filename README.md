# Clean Architecture Test

## healthcheck

- curl -X GET http://localhost:8787/api/v1/healthcheck/

## invitation token

- curl -X POST http://localhost:8787/api/v1/register/invitation/ --header 'content-type: application/json' --data '{"email":"name@domain.com"}'

## complete registration

- curl --request POST --url http://localhost:8787/api/v1/register/complete/52d4c5e1-04ac-4f58-ab6e-e47c4ac475cc --header 'content-type: application/json' --data '{"user": {"email":"name@domain.com", "password":"password"}}'

# 疑問

- uuid や chrono を使わないと、invitations の entity を表現できないけれど、外部のパッケージに依存することになる？
  他にやり方を思いつかないので、practical にやっていく

- error response を変更する。ApiResponse の方がわかりやすい

# 発見

- この実装では、外部(db, http, email service)とのやりとりは adapters の api と spi で行なっている
