-- =====================================================
-- 企业网站后端接口 - 数据库表结构
-- =====================================================

-- -----------------------------------------------------
-- 管理员表
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS admins (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE COMMENT '用户名',
    password_hash VARCHAR(255) NOT NULL COMMENT '密码哈希',
    nickname VARCHAR(50) COMMENT '昵称',
    role VARCHAR(20) NOT NULL DEFAULT 'admin' COMMENT '角色: admin-管理员, super_admin-超级管理员',
    status TINYINT NOT NULL DEFAULT 1 COMMENT '状态: 0-禁用, 1-启用',
    last_login_at DATETIME COMMENT '最后登录时间',
    created_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '更新时间'
);

-- 初始管理员账号 (密码: admin123)
-- 密码使用 bcrypt 加密
INSERT INTO admins (username, password_hash, nickname, role, status)
VALUES ('admin', '$2b$12$atntxdFH/sJZSLF08WtlKeKC262pmAodNblmEJ1/m8GZpAFZjA9AC', '系统管理员', 'super_admin', 1);

-- -----------------------------------------------------
-- 用户表 (前台用户)
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE COMMENT '用户名',
    email VARCHAR(100) NOT NULL COMMENT '邮箱',
    created_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '更新时间'
);

-- 初始测试用户
INSERT INTO users (username, email) VALUES ('test', 'test@example.com');

-- -----------------------------------------------------
-- 索引
-- -----------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_admins_username ON admins(username);
CREATE INDEX IF NOT EXISTS idx_admins_status ON admins(status);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
