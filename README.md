# Clean Architecture Test

## healthcheck

- curl -X GET http://localhost:8787/api/v1/healthcheck/

## invitation token

- curl -X POST http://localhost:8787/api/v1/register/invitation/ --header 'content-type: application/json' --data '{"email":"name@domain.com"}'

# 疑問

- uuid や chrono を使わないと、invitations の entity を表現できないけれど、外部のパッケージに依存することになる？
  他にやり方を思いつかないので、practical にやっていく

- error response を変更する。ApiResponse の方がわかりやすい

# 発見

- この実装では、外部(db, http, email service)とのやりとりは adapters の api と spi で行なっている
