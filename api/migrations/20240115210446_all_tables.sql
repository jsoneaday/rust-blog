CREATE EXTENSION pgcrypto;

create table administrator (
    "id"            bigserial primary key,
    "created_at"    timestamptz(3) not null default current_timestamp,
    "updated_at"    timestamptz(3) not null default current_timestamp,
    "user_name"     varchar(50) not null,
    "email"         varchar(120) not null unique,
    "password"      varchar(100) not null
);

create table post (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "title" varchar(250) not null,
    "message" text not null,
    "admin_id" bigserial not null,

    constraint fk_admin foreign key(admin_id) references administrator(id)
);

create table mail (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "from" varchar(150) not null,
    "subject" varchar(250) not null,
    "message" text not null
);