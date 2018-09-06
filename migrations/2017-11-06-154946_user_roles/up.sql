-- Your SQL goes here
create table user_roles (
  id integer primary key not null,
  user_id integer not null,
  role_id integer not null,
  foreign key(user_id) references users(id),
  foreign key(role_id) references roles(id)
);