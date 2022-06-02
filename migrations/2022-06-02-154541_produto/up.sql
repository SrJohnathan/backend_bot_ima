-- Your SQL goes here
create table IF NOT EXISTS produto(
 id serial primary key,
 id_categoria  int not null,
 name varchar(50) unique not null,
 preco varchar(7) not null,
 img varchar(255),
 created_on TIMESTAMP NOT NULL,
 last_login TIMESTAMP,
 FOREIGN KEY (id_categoria) REFERENCES categoria (id) ON DELETE CASCADE ON UPDATE CASCADE

/* ALTER TABLE tabela_filha
ADD CONSTRAINT nome_constraint FOREIGN KEY (coluna) REFERENCES tabela-pai(coluna);*/

);