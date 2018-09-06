-- This file should undo anything in `up.sql`
delete from roles where role in ('user', 'admin');