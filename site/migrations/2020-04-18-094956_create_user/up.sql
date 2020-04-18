CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  firstname VARCHAR(50) NOT NULL,
  lastname VARCHAR(50) NOT NULL,
  token VARCHAR(32) NOT NULL DEFAULT md5(random()::text),
  activated BOOLEAN NOT NULL DEFAULT FALSE
)