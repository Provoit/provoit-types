CREATE TABLE `frequencies` (
  `id` bigint unsigned PRIMARY KEY AUTO_INCREMENT,
  `label` text NOT NULL
);

INSERT INTO `frequencies`(`id`, `label`)
VALUES
(1, "Journalier"),
(2, "Hebdomadaire"),
(3, "Mensuel");
