-- CreateTable
CREATE TABLE `WordEntry` (
    `id` INTEGER NOT NULL AUTO_INCREMENT,
    `priority` INTEGER NOT NULL,
    `word` VARCHAR(191) NOT NULL,
    `meaning` VARCHAR(191) NOT NULL,
    `learning_history` JSON NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
