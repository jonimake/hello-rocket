-- Your SQL goes here
create table users (
    id integer primary key,
    username text not null,
    password text not null,
    active boolean not null
);