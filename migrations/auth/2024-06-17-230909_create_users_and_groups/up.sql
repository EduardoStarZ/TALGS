-- Your SQL goes here
CREATE TABLE `groups`(
	`id` INT4 NOT NULL PRIMARY KEY,
	`name` VARCHAR NOT NULL
);

CREATE TABLE `users`(
	`id` INT4 NOT NULL PRIMARY KEY,
	`name` VARCHAR NOT NULL,
	`password` TEXT NOT NULL,
	`group` INT4 NOT NULL,
	`cpf` VARCHAR NOT NULL
);

