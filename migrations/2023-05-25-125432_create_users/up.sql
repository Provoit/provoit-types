CREATE TABLE `users` (
  `id` bigint unsigned PRIMARY KEY AUTO_INCREMENT,
  `firstname` text NOT NULL,
  `lastname` text NOT NULL,
  `mail` varchar(255) UNIQUE NOT NULL,
  `passwd` text NOT NULL,
  `token` text,
  `token_gentime` text,
  `profile_pic` blob,
  `smoker` bool NOT NULL,
  `id_favorite_vehicle` bigint unsigned UNIQUE
);

