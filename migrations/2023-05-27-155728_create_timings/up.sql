CREATE TABLE `timings` (
  `id` bigint unsigned PRIMARY KEY AUTO_INCREMENT,
  `date` date,
  `time` time NOT NULL,
  `id_day` bigint unsigned,
  FOREIGN KEY (`id_day`) REFERENCES `days` (`id`)
);

