CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  participants VARCHAR Default ' ',
  schedule VARCHAR Default ' '
);
