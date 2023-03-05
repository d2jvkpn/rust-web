-- Add migration script here
ALTER TYPE user_status ADD VALUE 'frozen' AFTER 'ok';
