-- Your SQL goes here

CREATE TABLE "User" (
id uuid primary key,
username varchar not null,
password varchar not null,
email varchar not null,
created_at date not null default current_date,
updated_at date not null default current_date 
)