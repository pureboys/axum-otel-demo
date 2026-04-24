# Admin API 文档

## 基础信息

- **Base URL**: `/api/admin`
- **认证方式**: Bearer Token (JWT)
- **响应格式**: JSON

## 通用响应格式

```json
{
  "code": 0,
  "msg": "",
  "data": ...
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| code | int | 状态码，0=成功，其他=失败 |
| msg | string | 错误信息（失败时返回） |
| data | object/null | 响应数据 |

### 错误码说明

| HTTP Status | code | 说明 |
|-------------|------|------|
| 400 | 400 | 请求参数错误 |
| 401 | 401 | 未授权/认证失败 |
| 404 | 404 | 资源不存在 |
| 500 | 500 | 服务器内部错误 |

---

## 认证模块 `/api/admin/auth`

### 公开接口（无需认证）

#### POST /auth/login - 管理员登录

登录后台管理系统，获取访问令牌。

**请求参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| username | string | 是 | 用户名 |
| password | string | 是 | 密码 |

**请求示例**

```json
{
  "username": "admin",
  "password": "your_password"
}
```

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 86400,
    "admin": {
      "id": 1,
      "username": "admin",
      "nickname": "管理员",
      "role": "admin"
    }
  }
}
```

---

### 受保护接口（需认证）

需要在请求头中携带 `Authorization: Bearer <token>`

#### POST /auth/logout - 登出

**请求示例**

```
POST /api/admin/auth/logout
Authorization: Bearer <token>
```

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": null
}
```

#### GET /auth/info - 获取当前管理员信息

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "id": 1,
    "username": "admin",
    "nickname": "管理员",
    "role": "admin",
    "created_at": "2024-03-18 10:00:00"
  }
}
```

#### POST /auth/refresh - 刷新 Token

**请求参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| refresh_token | string | 否 | 刷新令牌（可选） |

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 86400
  }
}
```

---

## 用户管理 `/api/admin/users`

### GET /users - 获取所有用户

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": [
    {
      "id": 1,
      "username": "user1",
      "email": "user1@example.com"
    }
  ]
}
```

### GET /users/{id} - 获取用户详情

**路径参数**

| 参数名 | 类型 | 说明 |
|--------|------|------|
| id | int | 用户ID |

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "id": 1,
    "username": "user1",
    "email": "user1@example.com"
  }
}
```

### POST /users - 创建用户

**请求参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| username | string | 是 | 用户名 |
| email | string | 是 | 邮箱 |

**请求示例**

```json
{
  "username": "newuser",
  "email": "newuser@example.com"
}
```

**响应示例** (HTTP 201 Created)

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "id": 2,
    "username": "newuser",
    "email": "newuser@example.com"
  }
}
```

### PUT /users/{id} - 更新用户

**路径参数**

| 参数名 | 类型 | 说明 |
|--------|------|------|
| id | int | 用户ID |

**请求参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| username | string | 否 | 用户名 |
| email | string | 否 | 邮箱 |

**请求示例**

```json
{
  "username": "updated_user",
  "email": "updated@example.com"
}
```

### DELETE /users/{id} - 删除用户

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": null
}
```

---

## 标签管理 `/api/admin/tags`

### GET /tags - 获取所有标签

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": [
    {
      "id": 1,
      "name": "热门",
      "slug": "hot",
      "created_at": "2024-03-18 10:00:00",
      "updated_at": "2024-03-18 10:00:00"
    }
  ]
}
```

### GET /tags/{id} - 获取标签详情

### POST /tags - 创建标签

**请求参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| name | string | 是 | 标签名称 |
| slug | string | 是 | 标签别名（URL友好） |

**请求示例**

```json
{
  "name": "新品",
  "slug": "new"
}
```

### PUT /tags/{id} - 更新标签

**请求参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| name | string | 否 | 标签名称 |
| slug | string | 否 | 标签别名 |

### DELETE /tags/{id} - 删除标签

---

## 分类管理 `/api/admin/categories`

### GET /categories - 获取所有分类

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": [
    {
      "id": 1,
      "name": "电子产品",
      "slug": "electronics",
      "description": "电子产品分类",
      "category_type": "product",
      "parent_id": null,
      "created_at": "2024-03-18 10:00:00",
      "updated_at": "2024-03-18 10:00:00"
    }
  ]
}
```

### GET /categories/{id} - 获取分类详情

### POST /categories - 创建分类

**请求参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| name | string | 是 | 分类名称 |
| slug | string | 是 | 分类别名 |
| description | string | 是 | 分类描述 |
| category_type | string | 是 | 分类类型：`product` 或 `news` |
| parent_id | int | 否 | 父分类ID |

**请求示例**

```json
{
  "name": "手机",
  "slug": "phones",
  "description": "手机产品分类",
  "category_type": "product",
  "parent_id": 1
}
```

### PUT /categories/{id} - 更新分类

### DELETE /categories/{id} - 删除分类

---

## 产品管理 `/api/admin/products`

### GET /products - 获取产品列表（分页）

**查询参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| page | int | 否 | 页码（默认1） |
| limit | int | 否 | 每页数量（默认10，最大100） |
| category_id | int | 否 | 分类ID |
| status | int | 否 | 产品状态 |

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "items": [
      {
        "id": 1,
        "name": "iPhone 15",
        "description": "最新款iPhone",
        "price": 6999.00,
        "stock": 100,
        "category_id": 1,
        "image_url": "https://example.com/iphone15.jpg",
        "status": 1,
        "meta_title": "iPhone 15 - 苹果官网",
        "meta_description": "苹果最新款手机",
        "created_at": "2024-03-18 10:00:00",
        "updated_at": "2024-03-18 10:00:00"
      }
    ],
    "total": 50,
    "page": 1,
    "limit": 10
  }
}
```

### GET /products/{id} - 获取产品详情

### GET /products/{id}/tags - 获取产品详情（含标签）

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "product": { ... },
    "tags": [
      { "id": 1, "name": "热门", "slug": "hot" },
      { "id": 2, "name": "新品", "slug": "new" }
    ]
  }
}
```

### POST /products - 创建产品

**请求参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| name | string | 是 | 产品名称 |
| description | string | 是 | 产品描述 |
| price | float | 是 | 价格 |
| stock | int | 是 | 库存 |
| category_id | int | 是 | 分类ID |
| image_url | string | 是 | 图片URL |
| status | int | 否 | 状态（默认1） |
| meta_title | string | 否 | SEO标题 |
| meta_description | string | 否 | SEO描述 |

**请求示例**

```json
{
  "name": "iPhone 15 Pro",
  "description": "苹果旗舰手机",
  "price": 8999.00,
  "stock": 50,
  "category_id": 1,
  "image_url": "https://example.com/iphone15pro.jpg",
  "meta_title": "iPhone 15 Pro - 苹果官网",
  "meta_description": "苹果最新旗舰手机"
}
```

### PUT /products/{id} - 更新产品

### DELETE /products/{id} - 删除产品

### PUT /products/{id}/tags - 设置产品标签

**请求参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| tag_ids | array[int] | 是 | 标签ID数组 |

**请求示例**

```json
{
  "tag_ids": [1, 2, 3]
}
```

### POST /products/{id}/tags/{tag_id} - 添加产品标签

### DELETE /products/{id}/tags/{tag_id} - 移除产品标签

---

## 新闻管理 `/api/admin/news`

### GET /news - 获取新闻列表（分页）

**查询参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| page | int | 否 | 页码（默认1） |
| limit | int | 否 | 每页数量（默认10，最大100） |
| category_id | int | 否 | 分类ID |
| status | int | 否 | 新闻状态 |

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "items": [
      {
        "id": 1,
        "title": "公司新闻",
        "slug": "company-news",
        "content": "<p>新闻内容...</p>",
        "excerpt": "新闻摘要",
        "cover_image": "https://example.com/news1.jpg",
        "category_id": 1,
        "author": "admin",
        "view_count": 100,
        "status": 1,
        "is_featured": 1,
        "published_at": "2024-03-18 10:00:00",
        "meta_title": "公司新闻 - 官网",
        "meta_description": "公司最新新闻动态",
        "created_at": "2024-03-18 10:00:00",
        "updated_at": "2024-03-18 10:00:00"
      }
    ],
    "total": 20,
    "page": 1,
    "limit": 10
  }
}
```

### GET /news/{id} - 获取新闻详情

### POST /news - 创建新闻

**请求参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| title | string | 是 | 标题 |
| slug | string | 是 | 别名 |
| content | string | 是 | 内容（HTML） |
| excerpt | string | 否 | 摘要 |
| cover_image | string | 是 | 封面图片 |
| category_id | int | 是 | 分类ID |
| author | string | 是 | 作者 |
| status | int | 否 | 状态（默认1） |
| is_featured | int | 否 | 是否精选（默认0） |
| published_at | string | 否 | 发布时间 |
| meta_title | string | 否 | SEO标题 |
| meta_description | string | 否 | SEO描述 |

**请求示例**

```json
{
  "title": "公司获得年度创新奖",
  "slug": "innovation-award-2024",
  "content": "<p>我公司荣幸获得年度创新奖...</p>",
  "excerpt": "公司荣获年度创新奖",
  "cover_image": "https://example.com/award.jpg",
  "category_id": 1,
  "author": "admin",
  "is_featured": 1
}
```

### PUT /news/{id} - 更新新闻

### DELETE /news/{id} - 删除新闻

---

## 页面管理 `/api/admin/pages`

### GET /pages - 获取页面列表（分页）

**查询参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| page | int | 否 | 页码（默认1） |
| limit | int | 否 | 每页数量（默认10，最大100） |
| status | int | 否 | 页面状态 |

**响应示例**

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "items": [
      {
        "id": 1,
        "title": "关于我们",
        "slug": "about-us",
        "content": "<p>公司介绍...</p>",
        "meta_title": "关于我们",
        "meta_description": "了解我们的公司",
        "status": 1,
        "created_at": "2024-03-18 10:00:00",
        "updated_at": "2024-03-18 10:00:00"
      }
    ],
    "total": 5,
    "page": 1,
    "limit": 10
  }
}
```

### GET /pages/{id} - 获取页面详情

### POST /pages - 创建页面

**请求参数**

| 参数名 | 类型 | 必填 | 说明 |
|--------|------|------|------|
| title | string | 是 | 页面标题 |
| slug | string | 是 | 页面别名 |
| content | string | 是 | 页面内容（HTML） |
| meta_title | string | 否 | SEO标题 |
| meta_description | string | 否 | SEO描述 |
| status | int | 否 | 状态（默认1） |

**请求示例**

```json
{
  "title": "联系我们",
  "slug": "contact-us",
  "content": "<p>欢迎联系我们...</p>",
  "meta_title": "联系我们",
  "meta_description": "获取联系方式"
}
```

### PUT /pages/{id} - 更新页面

### DELETE /pages/{id} - 删除页面

---

## 附录

### 认证流程

1. 调用 `POST /api/admin/auth/login` 获取 token
2. 在后续请求中携带 `Authorization: Bearer <token>` 头
3. Token 过期时调用 `POST /api/admin/auth/refresh` 刷新

### 状态码说明

| 状态值 | 说明 |
|--------|------|
| 0 | 禁用/下线 |
| 1 | 启用/上线 |

### 产品/新闻状态说明

| 状态值 | 说明 |
|--------|------|
| 0 | 草稿 |
| 1 | 已发布 |
| 2 | 待审核 |
