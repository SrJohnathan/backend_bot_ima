-- Your SQL goes here

create table IF NOT EXISTS usuario(
 id serial primary key,
 name varchar(50) unique not null,
 password varchar(50) not null,
 numero varchar(15),
 email varchar(50) unique not null,
 id_firebase varchar(90) unique not null,
 token_firebase varchar(255) unique,
 created_on TIMESTAMP NOT NULL,
 last_login TIMESTAMP

);