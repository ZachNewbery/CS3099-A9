-- This file should undo anything in `up.sql`
SET FOREIGN_KEY_CHECKS = 0;
DROP TABLE IF EXISTS Users, LocalUsers, FederatedUsers, Communities, CommunitiesUsers, Posts, Text, Markdown;
SET FOREIGN_KEY_CHECKS = 1;
