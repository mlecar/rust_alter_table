CREATE DATABASE  IF NOT EXISTS `test`;
USE `test`;

DROP TABLE IF EXISTS `test_alter_table`;
CREATE TABLE `test_alter_table` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_bin;