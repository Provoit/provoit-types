-- Your SQL goes here

CREATE TABLE `vehicle_types` (
  `id` bigint unsigned PRIMARY KEY AUTO_INCREMENT,
  `label` text NOT NULl
);

INSERT INTO `vehicle_types`(`id`, `label`)
VALUES
(1, "Berline"),
(2, "Citadine"),
(3, "Utilitaire"),
(4, "Franchisseur"),
(5, "SUV");
