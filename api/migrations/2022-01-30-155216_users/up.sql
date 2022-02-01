-- Your SQL goes here
CREATE Table users (
	id SERIAL PRIMARY KEY,
	email VARCHAR NOT NULL,
	password VARCHAR NOT NULL
);

INSERT INTO users (email, password) 
VALUES 
	('jhon.smith@email.com', '1234'),
	('therock@actor.com', '0000');