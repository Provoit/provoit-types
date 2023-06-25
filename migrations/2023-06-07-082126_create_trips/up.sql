-- Your SQL goes here

CREATE TABLE `trips` (
  `id` bigint unsigned PRIMARY KEY AUTO_INCREMENT,
  `start` text NOT NULL,
  `end` text NOT NULL,
  `max_people` tinyint unsigned NOT NULL,
  `price` float NOT NULL,
  `recurring` boolean NOT NULL,
  `id_frequency` bigint unsigned,
  `id_vehicle` bigint unsigned NOT NULL,
  `id_start_timing` bigint unsigned UNIQUE NOT NULL,
  `id_end_timing` bigint unsigned UNIQUE NOT NULL,
  FOREIGN KEY (`id_frequency`) REFERENCES `frequencies` (`id`),
  FOREIGN KEY (`id_vehicle`) REFERENCES `vehicles` (`id`),
  FOREIGN KEY (`id_start_timing`) REFERENCES `timings` (`id`),
  FOREIGN KEY (`id_end_timing`) REFERENCES `timings` (`id`)
);
