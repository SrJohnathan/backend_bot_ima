-- Your SQL goes here

create table IF NOT EXISTS promo(
 id serial primary key,
 id_produto  int not null,
 name varchar(50) unique not null,
 img varchar(255),
 dia int unique not null,
 created_on TIMESTAMP NOT NULL,
 last_login TIMESTAMP,
FOREIGN KEY (id_produto) REFERENCES produto (id)
);