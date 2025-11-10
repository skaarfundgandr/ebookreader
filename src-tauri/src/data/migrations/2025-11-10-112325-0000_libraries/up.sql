-- Your SQL goes here
CREATE TABLE `libraries` (
    `library_id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `name` TEXT NOT NULL,
    `path` TEXT NOT NULL UNIQUE,
    `added_by` INTEGER,
    `added_at` TEXT NOT NULL DEFAULT datetime('now'),
    FOREIGN KEY (`added_by`) REFERENCES `users`(`user_id`) ON DELETE SET NULL,
    UNIQUE(`name`, `added_by`),
);