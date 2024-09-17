CREATE TABLE Students (
                          id SERIAL PRIMARY KEY,
                          first_name VARCHAR(255) NOT NULL,
                          last_name VARCHAR(255) NOT NULL
);

-- Create an ENUM type for Subject
CREATE TYPE Subject AS ENUM ('Math', 'German', 'History', 'Art');

-- Create the Books table
CREATE TABLE Books (
                       id SERIAL PRIMARY KEY,
                       name VARCHAR(255) NOT NULL,
                       subject Subject NOT NULL
);

-- Create the StudentsBooks table
CREATE TABLE StudentsBooks (
                               student_id INT NOT NULL,
                               book_id INT NOT NULL,
                               PRIMARY KEY (student_id, book_id),
                               FOREIGN KEY (student_id) REFERENCES Students(id) ON DELETE CASCADE,
                               FOREIGN KEY (book_id) REFERENCES Books(id) ON DELETE CASCADE
);