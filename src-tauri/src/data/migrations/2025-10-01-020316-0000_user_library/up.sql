-- Your SQL goes here
CREATE TABLE `user_library` (
    `user_id` INTEGER NOT NULL,
    `book_id` INTEGER NOT NULL,
    `added_at` TEXT DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`user_id`, `book_id`),
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`) ON UPDATE CASCADE,
    FOREIGN KEY (`book_id`) REFERENCES `books`(`id`) ON UPDATE CASCADE
);
