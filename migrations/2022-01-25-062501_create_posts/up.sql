-- Your SQL goes here

CREATE TABLE "posts" (
  id SERIAL PRIMARY KEY NOT NULL,
  title VARCHAR NOT NULL,
  description VARCHAR,
  author UUID NOT NULL,
  image VARCHAR,
  created_at TIMESTAMP NOT NULL DEFAULT current_date
--updated_at DATE NOT NULL DEFAULT current_date
);