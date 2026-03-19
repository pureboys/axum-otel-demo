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

-- -----------------------------------------------------
-- 分类表
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(100) NOT NULL COMMENT '分类名称',
    slug VARCHAR(100) NOT NULL COMMENT '分类别名',
    description TEXT COMMENT '分类描述',
    category_type VARCHAR(20) NOT NULL DEFAULT 'product' COMMENT '分类类型: product-商品分类, news-新闻分类',
    parent_id INTEGER COMMENT '父分类ID',
    created_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '更新时间',
    FOREIGN KEY (parent_id) REFERENCES categories(id) ON DELETE SET NULL
);

-- -----------------------------------------------------
-- 商品表
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(200) NOT NULL COMMENT '商品名称',
    description TEXT COMMENT '商品描述',
    price DECIMAL(10, 2) NOT NULL COMMENT '商品价格',
    stock INTEGER NOT NULL DEFAULT 0 COMMENT '库存数量',
    category_id INTEGER NOT NULL COMMENT '分类ID',
    image_url VARCHAR(500) COMMENT '商品图片URL',
    status TINYINT NOT NULL DEFAULT 1 COMMENT '状态: 0-下架, 1-上架',
    meta_title VARCHAR(200) COMMENT 'SEO标题',
    meta_description TEXT COMMENT 'SEO描述',
    created_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '更新时间',
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);

-- -----------------------------------------------------
-- 标签表
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL COMMENT '标签名称',
    slug VARCHAR(50) NOT NULL COMMENT '标签别名',
    created_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '更新时间'
);

-- -----------------------------------------------------
-- 商品标签关联表
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS product_tags (
    product_id INTEGER NOT NULL COMMENT '商品ID',
    tag_id INTEGER NOT NULL COMMENT '标签ID',
    PRIMARY KEY (product_id, tag_id),
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- -----------------------------------------------------
-- 索引
-- -----------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_categories_parent_id ON categories(parent_id);
CREATE INDEX IF NOT EXISTS idx_categories_slug ON categories(slug);
CREATE INDEX IF NOT EXISTS idx_products_category_id ON products(category_id);
CREATE INDEX IF NOT EXISTS idx_products_status ON products(status);
CREATE INDEX IF NOT EXISTS idx_tags_slug ON tags(slug);
CREATE INDEX IF NOT EXISTS idx_product_tags_tag_id ON product_tags(tag_id);

-- -----------------------------------------------------
-- 新闻表
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS news (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(200) NOT NULL COMMENT '新闻标题',
    slug VARCHAR(200) NOT NULL COMMENT '新闻别名',
    content TEXT NOT NULL COMMENT '新闻内容',
    excerpt TEXT COMMENT '摘要',
    cover_image VARCHAR(500) NOT NULL COMMENT '封面图片',
    category_id INTEGER NOT NULL COMMENT '分类ID',
    author VARCHAR(50) NOT NULL COMMENT '作者',
    view_count INTEGER NOT NULL DEFAULT 0 COMMENT '浏览数',
    status TINYINT NOT NULL DEFAULT 0 COMMENT '状态: 0-草稿, 1-发布',
    is_featured TINYINT NOT NULL DEFAULT 0 COMMENT '是否推荐: 0-普通, 1-推荐',
    published_at DATETIME COMMENT '发布时间',
    meta_title VARCHAR(200) COMMENT 'SEO标题',
    meta_description TEXT COMMENT 'SEO描述',
    created_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '更新时间',
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);

-- -----------------------------------------------------
-- 单页面表
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS pages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(200) NOT NULL COMMENT '页面标题',
    slug VARCHAR(200) NOT NULL COMMENT '页面别名(URL标识)',
    content TEXT NOT NULL COMMENT '页面内容',
    meta_title VARCHAR(200) COMMENT 'SEO标题',
    meta_description TEXT COMMENT 'SEO描述',
    status TINYINT NOT NULL DEFAULT 0 COMMENT '状态: 0-草稿, 1-发布',
    created_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT (datetime('now')) COMMENT '更新时间'
);

-- -----------------------------------------------------
-- 索引
-- -----------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_news_category_id ON news(category_id);
CREATE INDEX IF NOT EXISTS idx_news_slug ON news(slug);
CREATE INDEX IF NOT EXISTS idx_news_status ON news(status);
CREATE INDEX IF NOT EXISTS idx_news_published_at ON news(published_at);
CREATE INDEX IF NOT EXISTS idx_pages_slug ON pages(slug);
CREATE INDEX IF NOT EXISTS idx_pages_status ON pages(status);
