import { App, Button, Image, Popconfirm, Select, Space, Table, Typography } from 'antd'
import { useCallback, useEffect, useMemo, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { labelForProductNewsStatus, productNewsStatusOptions } from '../constants/status'
import { mockServer } from '../mock/server'
import type { Category, NewsItem } from '../types/api'

export function NewsAdminPage() {
  const { message } = App.useApp()
  const navigate = useNavigate()
  const [loading, setLoading] = useState(false)
  const [items, setItems] = useState<NewsItem[]>([])
  const [total, setTotal] = useState(0)
  const [page, setPage] = useState(1)
  const [limit, setLimit] = useState(10)
  const [categoryId, setCategoryId] = useState<number | undefined>()
  const [status, setStatus] = useState<number | undefined>()
  const [categories, setCategories] = useState<Category[]>([])

  const newsCategories = useMemo(() => categories.filter((c) => c.category_type === 'news'), [categories])

  const loadMeta = useCallback(async () => {
    const res = await mockServer.listCategories()
    if (res.code === 0) setCategories(res.data)
  }, [])

  const load = useCallback(async () => {
    setLoading(true)
    const res = await mockServer.listNews({ page, limit, category_id: categoryId, status })
    setLoading(false)
    if (res.code !== 0) {
      message.error(res.msg)
      return
    }
    setItems(res.data.items)
    setTotal(res.data.total)
  }, [message, page, limit, categoryId, status])

  useEffect(() => {
    void loadMeta()
  }, [loadMeta])

  useEffect(() => {
    void load()
  }, [load])

  return (
    <div>
      <Space style={{ marginBottom: 16, width: '100%', justifyContent: 'space-between' }} wrap>
        <Typography.Title level={4} style={{ margin: 0 }}>
          新闻管理
        </Typography.Title>
        <Button type="primary" onClick={() => navigate('/news/new')}>
          新建新闻
        </Button>
      </Space>
      <Space style={{ marginBottom: 16 }} wrap>
        <Select
          allowClear
          placeholder="分类筛选"
          style={{ width: 200 }}
          value={categoryId}
          onChange={(v) => {
            setPage(1)
            setCategoryId(v)
          }}
          options={newsCategories.map((c) => ({ value: c.id, label: `${c.name} (#${c.id})` }))}
        />
        <Select
          allowClear
          placeholder="状态筛选"
          style={{ width: 160 }}
          value={status}
          onChange={(v) => {
            setPage(1)
            setStatus(v)
          }}
          options={productNewsStatusOptions}
        />
      </Space>
      <Table
        rowKey="id"
        loading={loading}
        dataSource={items}
        scroll={{ x: 1200 }}
        pagination={{
          current: page,
          pageSize: limit,
          total,
          showSizeChanger: true,
          onChange: (p, ps) => {
            setPage(p)
            setLimit(ps || 10)
          },
        }}
        columns={[
          { title: 'ID', dataIndex: 'id', width: 72 },
          {
            title: '封面',
            dataIndex: 'cover_image',
            width: 88,
            render: (url: string) => <Image src={url} width={64} height={40} style={{ objectFit: 'cover' }} />,
          },
          { title: '标题', dataIndex: 'title', width: 200, ellipsis: true },
          { title: '别名', dataIndex: 'slug', width: 140, ellipsis: true },
          { title: '作者', dataIndex: 'author', width: 100 },
          {
            title: '精选',
            dataIndex: 'is_featured',
            width: 72,
            render: (v: number) => (v ? '是' : '否'),
          },
          {
            title: '状态',
            dataIndex: 'status',
            width: 100,
            render: (s: number) => labelForProductNewsStatus(s),
          },
          { title: '发布时间', dataIndex: 'published_at', width: 180 },
          {
            title: '操作',
            key: 'a',
            fixed: 'right',
            width: 160,
            render: (_, row) => (
              <Space>
                <Button type="link" size="small" onClick={() => navigate(`/news/${row.id}`)}>
                  详情
                </Button>
                <Popconfirm
                  title="确定删除？"
                  onConfirm={async () => {
                    const res = await mockServer.deleteNews(row.id)
                    if (res.code !== 0) {
                      message.error(res.msg)
                      return
                    }
                    message.success('已删除')
                    void load()
                  }}
                >
                  <Button type="link" danger size="small">
                    删除
                  </Button>
                </Popconfirm>
              </Space>
            ),
          },
        ]}
      />
    </div>
  )
}
