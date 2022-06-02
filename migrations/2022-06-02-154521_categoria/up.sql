-- Your SQL goes here
create table IF NOT EXISTS categoria(
 id serial primary key,
 name varchar(50) unique not null,
 created_on TIMESTAMP NOT NULL,
 last_login TIMESTAMP

);