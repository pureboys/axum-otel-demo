import { App, Button, Image, Popconfirm, Select, Space, Table, Typography } from 'antd'
import { useCallback, useEffect, useMemo, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { labelForProductNewsStatus, productNewsStatusOptions } from '../constants/status'
import { mockServer } from '../mock/server'
import type { Category, Product } from '../types/api'

export function ProductsPage() {
  const { message } = App.useApp()
  const navigate = useNavigate()
  const [loading, setLoading] = useState(false)
  const [items, setItems] = useState<Product[]>([])
  const [total, setTotal] = useState(0)
  const [page, setPage] = useState(1)
  const [limit, setLimit] = useState(10)
  const [categoryId, setCategoryId] = useState<number | undefined>()
  const [status, setStatus] = useState<number | undefined>()
  const [categories, setCategories] = useState<Category[]>([])

  const productCategories = useMemo(
    () => categories.filter((c) => c.category_type === 'product'),
    [categories],
  )

  const loadMeta = useCallback(async () => {
    const res = await mockServer.listCategories()
    if (res.code === 0) setCategories(res.data)
  }, [])

  const load = useCallback(async () => {
    setLoading(true)
    const res = await mockServer.listProducts({ page, limit, category_id: categoryId, status })
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
          产品管理
        </Typography.Title>
        <Button type="primary" onClick={() => navigate('/products/new')}>
          新建产品
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
          options={productCategories.map((c) => ({ value: c.id, label: `${c.name} (#${c.id})` }))}
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
        scroll={{ x: 1100 }}
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
            title: '图片',
            dataIndex: 'image_url',
            width: 88,
            render: (url: string) => <Image src={url} width={56} height={56} style={{ objectFit: 'cover' }} />,
          },
          { title: '名称', dataIndex: 'name', width: 160, ellipsis: true },
          {
            title: '价格',
            dataIndex: 'price',
            width: 100,
            render: (p: number) => `¥${p.toFixed(2)}`,
          },
          { title: '库存', dataIndex: 'stock', width: 88 },
          { title: '分类', dataIndex: 'category_id', width: 120, render: (cid: number) => categories.find((c) => c.id === cid)?.name ?? cid },
          {
            title: '状态',
            dataIndex: 'status',
            width: 100,
            render: (s: number) => labelForProductNewsStatus(s),
          },
          { title: '更新时间', dataIndex: 'updated_at', width: 180 },
          {
            title: '操作',
            key: 'a',
            fixed: 'right',
            width: 160,
            render: (_, row) => (
              <Space>
                <Button type="link" size="small" onClick={() => navigate(`/products/${row.id}`)}>
                  详情
                </Button>
                <Popconfirm
                  title="确定删除？"
                  onConfirm={async () => {
                    const res = await mockServer.deleteProduct(row.id)
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
