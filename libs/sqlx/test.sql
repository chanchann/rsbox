CREATE TABLE test_user (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_name VARCHAR(255) NOT NULL UNIQUE,
    user_age INT
);

INSERT INTO test_user (user_name, user_age) VALUES ('Alice', 25);
INSERT INTO test_user (user_name, user_age) VALUES ('Bob', 30);
INSERT INTO test_user (user_name, user_age) VALUES ('Charlie', 22);
INSERT INTO test_user (user_name, user_age) VALUES ('David', 28);