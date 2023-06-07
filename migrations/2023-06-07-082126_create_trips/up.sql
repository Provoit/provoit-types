-- Your SQL goes here

CREATE TABLE `trips` (
  `id` bigint unsigned PRIMARY KEY AUTO_INCREMENT,
  `start` text NOT NULL,
  `end` text NOT NULL,
  `max_people` tinyint unsigned NOT NULL,
  `price` float NOT NULL,
  `id_frequency` bigint unsigned NOT NULL,
  `id_vehicle` bigint unsigned NOT NULL,
  `id_start_timing` bigint unsigned UNIQUE NOT NULL,
  `id_end_timing` bigint unsigned UNIQUE NOT NULL
);

FOREIGN KEY (`id_frequency`) REFERENCIES `frequencies` (`id`);
FOREIGN KEY (`id_vehicle`) REFERENCIES `vehicles` (`id`);
FOREIGN KEY (`id_start_timing`) REFERENCIES `timings` (`id`);
FOREIGN KEY (`id_end_timing`) REFERENCIES `timings` (`id`);
