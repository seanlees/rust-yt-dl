-- Your SQL goes here
CREATE TABLE "sys_user"
(
    "id"    INTEGER NOT NULL UNIQUE,
    "uname" TEXT    NOT NULL,
    "pwd"   TEXT    NOT NULL,
    "email" TEXT,
    PRIMARY KEY ("id" AUTOINCREMENT)
);