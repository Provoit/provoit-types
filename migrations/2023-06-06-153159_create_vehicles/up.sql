-- Your SQL goes here

CREATE TABLE `vehicles` (
  `id` bigint unsigned PRIMARY KEY AUTO_INCREMENT,
  `name` text NOT NULL,
  `year` smallint unsigned NOT NULL,
  `nb_doors` tinyint unsigned NOT NULL,
  `nb_seats` tinyint unsigned NOT NULL,
  `trunk_size_L` smallint unsigned NOT NULL,
  `pic` blob,
  `id_user` bigint unsigned NOT NULL,
  `id_type` bigint unsigned NOT NULL
);
