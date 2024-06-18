-- Your SQL goes here
CREATE TABLE `key`(
	`id` INT4 NOT NULL PRIMARY KEY,
	`user_id` INT4 NOT NULL,
	`public_key` TEXT NOT NULL,
	`private_key` TEXT NOT NULL
);

