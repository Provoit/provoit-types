-- Your SQL goes here

CREATE TABLE `user_trips` (
  `id_user` bigint unsigned NOT NULL,
  `id_trip` bigint unsigned NOT NULL,
  PRIMARY KEY (`id_user`, `id_trip`),
  FOREIGN KEY (`id_user`) REFERENCES `users` (`id`),
  FOREIGN KEY (`id_trip`) REFERENCES `trips` (`id`)
);
