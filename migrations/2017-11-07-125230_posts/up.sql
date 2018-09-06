-- Your SQL goes here
create table posts (
  id integer primary key not null,
  user_id integer not null,
  title text not null,
  body text not null,
  foreign key(user_id) references user(id)
);