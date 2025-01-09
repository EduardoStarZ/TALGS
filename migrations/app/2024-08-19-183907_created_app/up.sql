-- Your SQL goes here
CREATE TABLE `category`(
	`id` INT2 NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL
);

CREATE TABLE `product`(
	`id` INT4 NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
    `image` TEXT, 
    `price` FLOAT NOT NULL,
    `warn_at` INT4 NOT NULL,  
    `id_category` INT2 NOT NULL,
	`total_amount` INT4 NOT NULL,
    `measure` INT4,
    `measure_unit` INT2
);

CREATE TABLE `purchase`(
	`id` INT4 NOT NULL PRIMARY KEY,
	`id_user` INT4 NOT NULL,
	`time` TEXT NOT NULL,
	`status` INT2 NOT NULL,
    `total` FLOAT NOT NULL
);

CREATE TABLE `address`(
	`id` INT4 NOT NULL PRIMARY KEY,
	`id_supplier` INT4 NOT NULL,
	`cep` INT4 NOT NULL,
	`city` TEXT NOT NULL,
	`neighborhood` TEXT NOT NULL,
	`block` TEXT,
	`number` TEXT NOT NULL,
	`complement` TEXT
);

CREATE TABLE `article`(
	`id` INT4 NOT NULL PRIMARY KEY,
	`id_stock` INT4 NOT NULL,
	`id_purchase` INT4 NOT NULL,
	`amount` INT4 NOT NULL
);

CREATE TABLE `supplier`(
	`id` INT4 NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`cnpj` TEXT,
	`cpf` TEXT,
	`email` TEXT NOT NULL
);

CREATE TABLE `stock`(
	`id` INT4 NOT NULL PRIMARY KEY,
	`id_product` INT4 NOT NULL,
	`id_supplier` INT4 NOT NULL,
	`expired` BOOL NOT NULL,
	`expire_date` TIMESTAMP NOT NULL,
	`available` BOOL NOT NULL,
	`batch` INT8,
	`amount` INT4 NOT NULL
);

