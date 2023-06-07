-- Your SQL goes here

CREATE TABLE `trip_road_types` (
  `id_trip` bigint unsigned NOT NULL,
  `id_road_type` bigint unsigned NOT NULL,
  PRIMARY KEY (`id_trip`, `id_road_type`),
  FOREIGN KEY (`id_trip`) REFERENCES `trips` (`id`),
  FOREIGN KEY (`id_road_type`) REFERENCES `road_types` (`id`)
);
