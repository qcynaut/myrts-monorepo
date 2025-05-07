-- Your SQL goes here

alter TABLE avs DROP COLUMN IF EXISTS gateway;

alter TABLE avs DROP COLUMN IF EXISTS ipv4;

alter TABLE avs DROP COLUMN IF EXISTS netmask;

alter TABLE avs ADD COLUMN networks TEXT;

alter TABLE avs ADD COLUMN mem_total TEXT;

alter TABLE avs ADD COLUMN mem_free TEXT;

alter TABLE avs ADD COLUMN disk_total TEXT;

alter TABLE avs ADD COLUMN disk_free TEXT;

alter TABLE avs ADD COLUMN cpu_temp TEXT;