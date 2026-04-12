CREATE DATABASE IF NOT EXISTS demo DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

USE demo;

drop table if exists sys_user;

CREATE TABLE IF NOT EXISTS sys_user
(
    id           BIGINT                             NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name         VARCHAR(16)                        NOT NULL,
    gender       VARCHAR(8)                         NOT NULL,
    account      VARCHAR(16)                        NOT NULL,
    password     VARCHAR(64)                        NOT NULL,
    mobile_phone VARCHAR(16)                        NOT NULL,
    birthday     DATE                               NOT NULL,
    enabled      BOOLEAN  DEFAULT TRUE              NOT NULL,
    created_at   DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at   DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL ON UPDATE CURRENT_TIMESTAMP
);

INSERT INTO sys_user (id, name, gender, account, password, mobile_phone, birthday, enabled, created_at, updated_at)
VALUES (1, '李四', 'female', 'lisi', '$2b$12$PsumwxjxX/o1RNOKpkc.Kuxea0izqSuhaod4PCudXoRh3zet1TASK',
        '17361631996', '2025-05-13', TRUE, '2025-05-18 12:39:53', '2025-05-18 12:39:53');
INSERT INTO sys_user (id, name, gender, account, password, mobile_phone, birthday, enabled, created_at, updated_at)
VALUES (2, '张三', 'male', 'admin', '$2b$12$PsumwxjxX/o1RNOKpkc.Kuxea0izqSuhaod4PCudXoRh3zet1TASK',
        '19909407240', '2025-05-18', FALSE, '2025-05-18 09:51:54', '2025-05-18 09:51:54');
INSERT INTO sys_user (id, name, gender, account, password, mobile_phone, birthday, enabled, created_at, updated_at)
VALUES (3, '赵六', 'female', 'zhaoliu', '$2b$12$EJOKHLJLnfHrgrXbZl8uge3N4VEgR9FWHwq3a6pgTIM8O66Lf/9DW',
        '18361631783', '2025-06-11', TRUE, '2025-06-02 09:39:36', '2025-06-02 09:39:36');
