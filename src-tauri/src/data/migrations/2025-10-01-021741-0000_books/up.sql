-- Your SQL goes here
CREATE TABLE `books` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `title` TEXT NOT NULL,
    `author_id` INTEGER,
    `published_date` TEXT,
    `publisher_id` INTEGER,
    `isbn` TEXT,
    `file_type` TEXT,
    `file_path` TEXT NOT NULL,
    `added_at` TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (`author_id`) REFERENCES `authors`(`author_id`) ON DELETE SET NULL,
    FOREIGN KEY (`publisher_id`) REFERENCES `publishers`(`publisher_id`) ON DELETE SET NULL
);
