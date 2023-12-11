CREATE TABLE books (
	id SERIAL PRIMARY KEY NOT NULL,
	name VARCHAR(255) NOT NULL,
	author VARCHAR(255) NOT NULL,
	year INT NOT NULL
);


INSERT INTO books(name, author, year) VALUES
('Test Book', 'Test Author', '2020'),
('Test Book 2', 'Test Author 2', '2021'),
('Test Book 3', 'Test Author 3', '2022'),
('Test Book 4', 'Test Author 4', '2023'),
('Test Book 5', 'Test Author 5', '2024');
