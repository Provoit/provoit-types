-- This file should undo anything in `up.sql`

ALTER TABLE `trips` ADD FOREIGN KEY (`id_frequency`) REFERENCES `frequencies` (`id`);

ALTER TABLE `trips` ADD FOREIGN KEY (`id_vehicle`) REFERENCES `vehicles` (`id`);

ALTER TABLE `trips` ADD FOREIGN KEY (`id_start_timing`) REFERENCES `timings` (`id`);

ALTER TABLE `trips` ADD FOREIGN KEY (`id_end_timing`) REFERENCES `timings` (`id`);
