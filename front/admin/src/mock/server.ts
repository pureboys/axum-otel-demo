import type {
  ApiResponse,
  AdminProfile,
  Category,
  CategoryType,
  CmsPage,
  EndUser,
  LoginData,
  NewsItem,
  Paginated,
  Product,
  ProductWithTags,
  RefreshData,
  Tag,
} from '../types/api'
import { sleep } from '../utils/sleep'

function ok<T>(data: T): ApiResponse<T> {
  return { code: 0, msg: '', data }
}

function fail<T = null>(code: number, msg: string, data: T = null as T): ApiResponse<T> {
  return { code, msg, data }
}

function readFileAsDataUrl(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const r = new FileReader()
    r.onload = () => resolve(String(r.result))
    r.onerror = () => reject(r.error ?? new Error('读取文件失败'))
    r.readAsDataURL(file)
  })
}

const ts = '2024-03-18 10:00:00'

let nextUserId = 3
let nextTagId = 3
let nextCatId = 3
let nextProductId = 2
let nextNewsId = 2
let nextPageId = 2

const users: EndUser[] = [
  { id: 1, username: 'user1', email: 'user1@example.com' },
  { id: 2, username: 'user2', email: 'user2@example.com' },
]

const tags: Tag[] = [
  { id: 1, name: '热门', slug: 'hot', created_at: ts, updated_at: ts },
  { id: 2, name: '新品', slug: 'new', created_at: ts, updated_at: ts },
]

const categories: Category[] = [
  {
    id: 1,
    name: '电子产品',
    slug: 'electronics',
    description: '电子产品分类',
    category_type: 'product',
    parent_id: null,
    created_at: ts,
    updated_at: ts,
  },
  {
    id: 2,
    name: '公司动态',
    slug: 'company',
    description: '新闻分类',
    category_type: 'news',
    parent_id: null,
    created_at: ts,
    updated_at: ts,
  },
]

const products: Product[] = [
  {
    id: 1,
    name: 'iPhone 15',
    description: '最新款iPhone',
    price: 6999,
    stock: 100,
    category_id: 1,
    image_url: 'https://picsum.photos/seed/iphone/400/300',
    status: 1,
    meta_title: 'iPhone 15',
    meta_description: '苹果最新款手机',
    created_at: ts,
    updated_at: ts,
  },
]

const productTags: Record<number, number[]> = {
  1: [1, 2],
}

const news: NewsItem[] = [
  {
    id: 1,
    title: '公司新闻',
    slug: 'company-news',
    content: '<p>新闻内容示例</p>',
    excerpt: '新闻摘要',
    cover_image: 'https://picsum.photos/seed/news/800/400',
    category_id: 2,
    author: 'admin',
    view_count: 100,
    status: 1,
    is_featured: 1,
    published_at: ts,
    meta_title: '公司新闻 - 官网',
    meta_description: '公司最新新闻动态',
    created_at: ts,
    updated_at: ts,
  },
]

const pages: CmsPage[] = [
  {
    id: 1,
    title: '关于我们',
    slug: 'about-us',
    content: '<p>公司介绍...</p>',
    meta_title: '关于我们',
    meta_description: '了解我们的公司',
    status: 1,
    created_at: ts,
    updated_at: ts,
  },
]

function nowStr() {
  const d = new Date()
  const p = (n: number) => String(n).padStart(2, '0')
  return `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())} ${p(d.getHours())}:${p(d.getMinutes())}:${p(d.getSeconds())}`
}

function paginate<T>(
  all: T[],
  page: number,
  limit: number,
): Paginated<T> {
  const p = Math.max(1, page)
  const l = Math.min(100, Math.max(1, limit))
  const total = all.length
  const start = (p - 1) * l
  return { items: all.slice(start, start + l), total, page: p, limit: l }
}

export const mockServer = {
  async login(username: string, password: string, altcha: string): Promise<ApiResponse<LoginData>> {
    await sleep(200)
    if (!username?.trim() || !password) return fail(400, '请求参数错误', null as unknown as LoginData)
    if (!altcha?.trim()) return fail(400, '请完成人机验证', null as unknown as LoginData)
    const admin: AdminProfile = {
      id: 1,
      username: username.trim(),
      nickname: username.trim() === 'admin' ? '管理员' : username.trim(),
      role: 'admin',
      created_at: ts,
    }
    const data: LoginData = {
      token: `mock-${Date.now()}`,
      token_type: 'Bearer',
      expires_in: 86400,
      admin,
    }
    return ok(data)
  },

  async logout(): Promise<ApiResponse<null>> {
    await sleep(150)
    return ok(null)
  },

  async refresh(): Promise<ApiResponse<RefreshData>> {
    await sleep(150)
    return ok({
      token: `mock-${Date.now()}`,
      token_type: 'Bearer',
      expires_in: 86400,
    })
  },

  async listUsers(): Promise<ApiResponse<EndUser[]>> {
    await sleep(200)
    return ok([...users])
  },

  async getUser(id: number): Promise<ApiResponse<EndUser>> {
    await sleep(150)
    const u = users.find((x) => x.id === id)
    if (!u) return fail(404, '资源不存在', null as unknown as EndUser)
    return ok({ ...u })
  },

  async createUser(body: { username: string; email: string }): Promise<ApiResponse<EndUser>> {
    await sleep(200)
    if (!body.username?.trim() || !body.email?.trim()) return fail(400, '请求参数错误', null as unknown as EndUser)
    const u: EndUser = { id: nextUserId++, username: body.username.trim(), email: body.email.trim() }
    users.push(u)
    return ok({ ...u })
  },

  async updateUser(id: number, body: Partial<Pick<EndUser, 'username' | 'email'>>): Promise<ApiResponse<EndUser>> {
    await sleep(200)
    const u = users.find((x) => x.id === id)
    if (!u) return fail(404, '资源不存在', null as unknown as EndUser)
    if (body.username != null) u.username = body.username
    if (body.email != null) u.email = body.email
    return ok({ ...u })
  },

  async deleteUser(id: number): Promise<ApiResponse<null>> {
    await sleep(150)
    const i = users.findIndex((x) => x.id === id)
    if (i === -1) return fail(404, '资源不存在')
    users.splice(i, 1)
    return ok(null)
  },

  async listTags(): Promise<ApiResponse<Tag[]>> {
    await sleep(200)
    return ok(tags.map((t) => ({ ...t })))
  },

  async getTag(id: number): Promise<ApiResponse<Tag>> {
    await sleep(120)
    const t = tags.find((x) => x.id === id)
    if (!t) return fail(404, '资源不存在', null as unknown as Tag)
    return ok({ ...t })
  },

  async createTag(body: { name: string; slug: string }): Promise<ApiResponse<Tag>> {
    await sleep(180)
    if (!body.name?.trim() || !body.slug?.trim()) return fail(400, '请求参数错误', null as unknown as Tag)
    const t0 = nowStr()
    const t: Tag = { id: nextTagId++, name: body.name.trim(), slug: body.slug.trim(), created_at: t0, updated_at: t0 }
    tags.push(t)
    return ok({ ...t })
  },

  async updateTag(id: number, body: Partial<Pick<Tag, 'name' | 'slug'>>): Promise<ApiResponse<Tag>> {
    await sleep(180)
    const t = tags.find((x) => x.id === id)
    if (!t) return fail(404, '资源不存在', null as unknown as Tag)
    if (body.name != null) t.name = body.name
    if (body.slug != null) t.slug = body.slug
    t.updated_at = nowStr()
    return ok({ ...t })
  },

  async deleteTag(id: number): Promise<ApiResponse<null>> {
    await sleep(150)
    const i = tags.findIndex((x) => x.id === id)
    if (i === -1) return fail(404, '资源不存在')
    tags.splice(i, 1)
    for (const k of Object.keys(productTags)) {
      const pid = Number(k)
      productTags[pid] = (productTags[pid] || []).filter((tid) => tid !== id)
    }
    return ok(null)
  },

  async listCategories(): Promise<ApiResponse<Category[]>> {
    await sleep(200)
    return ok(categories.map((c) => ({ ...c })))
  },

  async getCategory(id: number): Promise<ApiResponse<Category>> {
    await sleep(120)
    const c = categories.find((x) => x.id === id)
    if (!c) return fail(404, '资源不存在', null as unknown as Category)
    return ok({ ...c })
  },

  async createCategory(body: {
    name: string
    slug: string
    description: string
    category_type: CategoryType
    parent_id?: number | null
  }): Promise<ApiResponse<Category>> {
    await sleep(200)
    if (!body.name?.trim() || !body.slug?.trim() || !body.description?.trim() || !body.category_type) {
      return fail(400, '请求参数错误', null as unknown as Category)
    }
    const t0 = nowStr()
    const c: Category = {
      id: nextCatId++,
      name: body.name.trim(),
      slug: body.slug.trim(),
      description: body.description.trim(),
      category_type: body.category_type,
      parent_id: body.parent_id ?? null,
      created_at: t0,
      updated_at: t0,
    }
    categories.push(c)
    return ok({ ...c })
  },

  async updateCategory(
    id: number,
    body: Partial<Pick<Category, 'name' | 'slug' | 'description' | 'category_type' | 'parent_id'>>,
  ): Promise<ApiResponse<Category>> {
    await sleep(180)
    const c = categories.find((x) => x.id === id)
    if (!c) return fail(404, '资源不存在', null as unknown as Category)
    Object.assign(c, body)
    c.updated_at = nowStr()
    return ok({ ...c })
  },

  async deleteCategory(id: number): Promise<ApiResponse<null>> {
    await sleep(150)
    const i = categories.findIndex((x) => x.id === id)
    if (i === -1) return fail(404, '资源不存在')
    categories.splice(i, 1)
    return ok(null)
  },

  async listProducts(q: { page?: number; limit?: number; category_id?: number; status?: number }) {
    await sleep(220)
    let list = [...products]
    if (q.category_id != null) list = list.filter((p) => p.category_id === q.category_id)
    if (q.status != null) list = list.filter((p) => p.status === q.status)
    const page = q.page ?? 1
    const limit = q.limit ?? 10
    return ok(paginate(list, page, limit))
  },

  async getProduct(id: number): Promise<ApiResponse<Product>> {
    await sleep(150)
    const p = products.find((x) => x.id === id)
    if (!p) return fail(404, '资源不存在', null as unknown as Product)
    return ok({ ...p })
  },

  async getProductWithTags(id: number): Promise<ApiResponse<ProductWithTags>> {
    await sleep(180)
    const p = products.find((x) => x.id === id)
    if (!p) return fail(404, '资源不存在', null as unknown as ProductWithTags)
    const ids = productTags[id] || []
    const tagList = ids
      .map((tid) => tags.find((t) => t.id === tid))
      .filter(Boolean)
      .map((t) => ({ id: t!.id, name: t!.name, slug: t!.slug }))
    return ok({ product: { ...p }, tags: tagList })
  },

  async createProduct(body: Omit<Product, 'id' | 'created_at' | 'updated_at'> & { status?: number }) {
    await sleep(220)
    if (
      !body.name?.trim() ||
      !body.description?.trim() ||
      body.price == null ||
      body.stock == null ||
      !body.category_id ||
      !body.image_url?.trim()
    ) {
      return fail(400, '请求参数错误', null as unknown as Product)
    }
    const t0 = nowStr()
    const p: Product = {
      id: nextProductId++,
      name: body.name.trim(),
      description: body.description,
      price: body.price,
      stock: body.stock,
      category_id: body.category_id,
      image_url: body.image_url.trim(),
      status: body.status ?? 1,
      meta_title: body.meta_title ?? null,
      meta_description: body.meta_description ?? null,
      created_at: t0,
      updated_at: t0,
    }
    products.push(p)
    return ok({ ...p })
  },

  async updateProduct(id: number, body: Partial<Omit<Product, 'id' | 'created_at'>>) {
    await sleep(200)
    const p = products.find((x) => x.id === id)
    if (!p) return fail(404, '资源不存在', null as unknown as Product)
    Object.assign(p, body)
    p.updated_at = nowStr()
    return ok({ ...p })
  },

  async deleteProduct(id: number): Promise<ApiResponse<null>> {
    await sleep(150)
    const i = products.findIndex((x) => x.id === id)
    if (i === -1) return fail(404, '资源不存在')
    products.splice(i, 1)
    delete productTags[id]
    return ok(null)
  },

  async setProductTags(id: number, tag_ids: number[]): Promise<ApiResponse<null>> {
    await sleep(180)
    const p = products.find((x) => x.id === id)
    if (!p) return fail(404, '资源不存在')
    productTags[id] = [...new Set(tag_ids)]
    return ok(null)
  },

  async addProductTag(productId: number, tagId: number): Promise<ApiResponse<null>> {
    await sleep(120)
    const p = products.find((x) => x.id === productId)
    if (!p) return fail(404, '资源不存在')
    const arr = productTags[productId] || []
    if (!arr.includes(tagId)) arr.push(tagId)
    productTags[productId] = arr
    return ok(null)
  },

  async removeProductTag(productId: number, tagId: number): Promise<ApiResponse<null>> {
    await sleep(120)
    const p = products.find((x) => x.id === productId)
    if (!p) return fail(404, '资源不存在')
    productTags[productId] = (productTags[productId] || []).filter((t) => t !== tagId)
    return ok(null)
  },

  async listNews(q: { page?: number; limit?: number; category_id?: number; status?: number }) {
    await sleep(220)
    let list = [...news]
    if (q.category_id != null) list = list.filter((n) => n.category_id === q.category_id)
    if (q.status != null) list = list.filter((n) => n.status === q.status)
    return ok(paginate(list, q.page ?? 1, q.limit ?? 10))
  },

  async getNews(id: number): Promise<ApiResponse<NewsItem>> {
    await sleep(150)
    const n = news.find((x) => x.id === id)
    if (!n) return fail(404, '资源不存在', null as unknown as NewsItem)
    return ok({ ...n })
  },

  async createNews(body: Partial<NewsItem> & Pick<NewsItem, 'title' | 'slug' | 'content' | 'cover_image' | 'category_id' | 'author'>) {
    await sleep(220)
    if (!body.title?.trim() || !body.slug?.trim() || !body.content?.trim() || !body.cover_image?.trim() || !body.category_id || !body.author?.trim()) {
      return fail(400, '请求参数错误', null as unknown as NewsItem)
    }
    const t0 = nowStr()
    const n: NewsItem = {
      id: nextNewsId++,
      title: body.title.trim(),
      slug: body.slug.trim(),
      content: body.content,
      excerpt: body.excerpt ?? null,
      cover_image: body.cover_image.trim(),
      category_id: body.category_id,
      author: body.author.trim(),
      view_count: 0,
      status: body.status ?? 1,
      is_featured: body.is_featured ?? 0,
      published_at: body.published_at ?? t0,
      meta_title: body.meta_title ?? null,
      meta_description: body.meta_description ?? null,
      created_at: t0,
      updated_at: t0,
    }
    news.push(n)
    return ok({ ...n })
  },

  async updateNews(id: number, body: Partial<NewsItem>) {
    await sleep(200)
    const n = news.find((x) => x.id === id)
    if (!n) return fail(404, '资源不存在', null as unknown as NewsItem)
    Object.assign(n, body)
    n.updated_at = nowStr()
    return ok({ ...n })
  },

  async deleteNews(id: number): Promise<ApiResponse<null>> {
    await sleep(150)
    const i = news.findIndex((x) => x.id === id)
    if (i === -1) return fail(404, '资源不存在')
    news.splice(i, 1)
    return ok(null)
  },

  async listPages(q: { page?: number; limit?: number; status?: number }) {
    await sleep(220)
    let list = [...pages]
    if (q.status != null) list = list.filter((p) => p.status === q.status)
    return ok(paginate(list, q.page ?? 1, q.limit ?? 10))
  },

  async getPage(id: number): Promise<ApiResponse<CmsPage>> {
    await sleep(150)
    const p = pages.find((x) => x.id === id)
    if (!p) return fail(404, '资源不存在', null as unknown as CmsPage)
    return ok({ ...p })
  },

  async createPage(body: Pick<CmsPage, 'title' | 'slug' | 'content'> & Partial<Pick<CmsPage, 'meta_title' | 'meta_description' | 'status'>>) {
    await sleep(200)
    if (!body.title?.trim() || !body.slug?.trim() || !body.content?.trim()) {
      return fail(400, '请求参数错误', null as unknown as CmsPage)
    }
    const t0 = nowStr()
    const p: CmsPage = {
      id: nextPageId++,
      title: body.title.trim(),
      slug: body.slug.trim(),
      content: body.content,
      meta_title: body.meta_title ?? null,
      meta_description: body.meta_description ?? null,
      status: body.status ?? 1,
      created_at: t0,
      updated_at: t0,
    }
    pages.push(p)
    return ok({ ...p })
  },

  async updatePage(id: number, body: Partial<CmsPage>) {
    await sleep(180)
    const p = pages.find((x) => x.id === id)
    if (!p) return fail(404, '资源不存在', null as unknown as CmsPage)
    Object.assign(p, body)
    p.updated_at = nowStr()
    return ok({ ...p })
  },

  async deletePage(id: number): Promise<ApiResponse<null>> {
    await sleep(150)
    const i = pages.findIndex((x) => x.id === id)
    if (i === -1) return fail(404, '资源不存在')
    pages.splice(i, 1)
    return ok(null)
  },

  /** 演示用：将文件读为 Data URL，便于在无后端环境下插入编辑器 */
  async uploadRichMedia(file: File): Promise<ApiResponse<{ url: string }>> {
    await sleep(120)
    const max = 40 * 1024 * 1024
    if (file.size > max) return fail(400, '文件过大（演示上限 40MB）')
    try {
      const url = await readFileAsDataUrl(file)
      return ok({ url })
    } catch {
      return fail(500, '读取文件失败')
    }
  },
}
