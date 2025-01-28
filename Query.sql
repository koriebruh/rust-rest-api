CREATE DATABASE restrust;
USE restrust;

SHOW TABLES;

CREATE TABLE users
(
    id         int auto_increment primary key,
    username  varchar(255),
    password   varchar(255),
    email      varchar(255),
    created_at bigint,
    updated_at bigint
);

select * from users;

select 1;