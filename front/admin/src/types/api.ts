export interface ApiResponse<T> {
  code: number
  msg: string
  data: T
}

export interface AdminProfile {
  id: number
  username: string
  nickname: string
  role: string
  created_at?: string
}

export interface LoginData {
  token: string
  token_type: string
  expires_in: number
  admin: AdminProfile
}

export interface RefreshData {
  token: string
  token_type: string
  expires_in: number
}

export interface EndUser {
  id: number
  username: string
  email: string
}

export interface Tag {
  id: number
  name: string
  slug: string
  created_at: string
  updated_at: string
}

export type CategoryType = 'product' | 'news'

export interface Category {
  id: number
  name: string
  slug: string
  description: string
  category_type: CategoryType
  parent_id: number | null
  created_at: string
  updated_at: string
}

export interface Product {
  id: number
  name: string
  description: string
  price: number
  stock: number
  category_id: number
  image_url: string
  status: number
  meta_title: string | null
  meta_description: string | null
  created_at: string
  updated_at: string
}

export interface Paginated<T> {
  items: T[]
  total: number
  page: number
  limit: number
}

export interface ProductWithTags {
  product: Product
  tags: Pick<Tag, 'id' | 'name' | 'slug'>[]
}

export interface NewsItem {
  id: number
  title: string
  slug: string
  content: string
  excerpt: string | null
  cover_image: string
  category_id: number
  author: string
  view_count: number
  status: number
  is_featured: number
  published_at: string | null
  meta_title: string | null
  meta_description: string | null
  created_at: string
  updated_at: string
}

export interface CmsPage {
  id: number
  title: string
  slug: string
  content: string
  meta_title: string | null
  meta_description: string | null
  status: number
  created_at: string
  updated_at: string
}
