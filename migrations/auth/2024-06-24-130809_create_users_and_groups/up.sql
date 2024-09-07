-- Your SQL goes here
CREATE TABLE `users`(
	`id` INT4 NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`password` TEXT NOT NULL,
	`group` INT2 NOT NULL,
	`email` TEXT NOT NULL
);
