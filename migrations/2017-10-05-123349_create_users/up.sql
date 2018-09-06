-- Your SQL goes here
create table users (
    id integer primary key not null,
    username text not null,
    password text not null,
    active boolean not null
);