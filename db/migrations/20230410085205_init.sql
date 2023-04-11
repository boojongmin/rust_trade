-- Add migration script here
create table if not exists users (
    id serial primary key,
    name varchar(255) not null,
    email varchar(255) not null,
    password varchar(255) not null,
    created_at timestamp default now(),
    updated_at timestamp default now()
);

create table if not exists candles (
    id serial primary key,
    close double precision not null,
    created_at timestamp not null
);

create unique index uidx_candles_created_at on candles (created_at);

-- Equity = (현재 주식 가격 x 보유 주식 수) - 총 비용
create table if not exists account_history (
    id serial primary key,
    strategy_id integer not null,
    balance double precision not null,
    price double precision not null,
    quantity double precision not null,
    created_at timestamp default now() not null
);

do $$
begin
  FOR i IN 10..110 LOOP
    INSERT INTO account_history(strategy_id, balance, price, quantity)
    VALUES (i, 1000000, 0, 0);
  END LOOP;
end; $$




