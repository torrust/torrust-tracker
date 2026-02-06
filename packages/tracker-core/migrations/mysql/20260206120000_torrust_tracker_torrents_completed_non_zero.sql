-- Remove any rows with completed = 0 (should not exist in normal operation)
DELETE FROM torrents WHERE completed = 0;

-- Add CHECK constraint to enforce completed >= 1
-- Note: MySQL 8.0.16+ enforces CHECK constraints
ALTER TABLE torrents ADD CONSTRAINT chk_completed_non_zero CHECK (completed >= 1);

-- Change the default value from 0 to 1
ALTER TABLE torrents ALTER completed SET DEFAULT 1;
