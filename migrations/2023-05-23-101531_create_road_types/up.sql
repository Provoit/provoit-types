-- Your SQL goes here

CREATE TABLE `road_types` (
  `id` bigint unsigned PRIMARY KEY AUTO_INCREMENT,
  `label` text NOT NULL
);

INSERT INTO `road_types`(`id`, `label`)
VALUES
(1, "Classique"),
(2, "Autoroute"),
(3, "Ferry"),
(4, "Chemin de terre");
