CREATE TABLE `days` (
  `id` bigint unsigned PRIMARY KEY AUTO_INCREMENT,
  `label` text NOT NULL
);

INSERT INTO `days`(`id`, `label`)
VALUES
(1, "Lundi"),
(2, "Mardi"),
(3, "Mercredi"),
(4, "Jeudi"),
(5, "Vendredi"),
(6, "Samedi"),
(7, "Dimanche");

