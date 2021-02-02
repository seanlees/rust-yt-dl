-- Your SQL goes here
CREATE TABLE "dl_list"
(
    "id"              INTEGER NOT NULL UNIQUE,
    "dl_url"          TEXT    NOT NULL, -- 下载地址
    "dl_status"       INTEGER NOT NULL, --下载状态：0未开始 1正在下载  2完毕 3失败
    "dl_progress"     NUMERIC DEFAULT 0,
    "dl_create_time"  TEXT    NOT NULL, --开始下载时间
    "dl_end_time"     TEXT    NOT NULL, --结束下载时间
    "dl_type"         TEXT    NOT NULL, --下载类型：video,audio
    "file_size"       TEXT,             --文件大小
    "file_store_path" TEXT,             --文件存储路径
    PRIMARY KEY ("id" AUTOINCREMENT)
);