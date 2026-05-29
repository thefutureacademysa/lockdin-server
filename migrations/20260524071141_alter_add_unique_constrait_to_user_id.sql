-- Add migration script here
ALTER TABLE "otps" ADD UNIQUE (user_id);