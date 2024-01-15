create table user (
    "id"            bigserial primary key,
    "created_at"    timestamptz(3) not null default current_timestamp,
    "updated_at"    timestamptz(3) not null default current_timestamp,
    "user_name"     varchar(50) not null
    "email"         varchar(100) not null
    "password"      varchar(100) not null
);

create table post (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "message" text not null,
    "user_id" bigserial not null,

    constraint fk_user foreign key(user_id) references user(id)
);