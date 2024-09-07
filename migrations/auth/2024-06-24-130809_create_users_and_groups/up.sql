-- Your SQL goes here
CREATE TABLE `users`(
	`id` INT2 NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`password` TEXT NOT NULL,
	`group` INTEGER NOT NULL,
	`email` TEXT NOT NULL
);
