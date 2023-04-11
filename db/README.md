# sqlx 실행 스크립트

- [참고](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)

```
docker compose up -d

cargo install sqlx-cli
# cargo install sqlx-cli --no-default-features --features native-tls,postgres
sqlx database create

sqlx migrate add init
sqlx migrate run


```