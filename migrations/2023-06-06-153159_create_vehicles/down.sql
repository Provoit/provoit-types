-- This file should undo anything in `up.sql`
ALTER TABLE `users` DROP CONSTRAINT fk_user_favorite_vehicle;
DROP TABLE IF EXISTS `vehicles`;
