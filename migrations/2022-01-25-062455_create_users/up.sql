-- Your SQL goes here

CREATE TABLE "users" (
  id SERIAL PRIMARY KEY NOT NULL,
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  role VARCHAR NOT NULL DEFAULT 'Normal',
  created_at TIMESTAMP NOT NULL DEFAULT current_date
);