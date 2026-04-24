import { App, Button, Popconfirm, Select, Space, Table, Typography } from 'antd'
import { useCallback, useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { labelForSitePageStatus, sitePageStatusOptions } from '../constants/sitePage'
import { mockServer } from '../mock/server'
import type { CmsPage } from '../types/api'

export function SitePagesPage() {
  const { message } = App.useApp()
  const navigate = useNavigate()
  const [loading, setLoading] = useState(false)
  const [items, setItems] = useState<CmsPage[]>([])
  const [total, setTotal] = useState(0)
  const [page, setPage] = useState(1)
  const [limit, setLimit] = useState(10)
  const [status, setStatus] = useState<number | undefined>()

  const load = useCallback(async () => {
    setLoading(true)
    const res = await mockServer.listPages({ page, limit, status })
    setLoading(false)
    if (res.code !== 0) {
      message.error(res.msg)
      return
    }
    setItems(res.data.items)
    setTotal(res.data.total)
  }, [message, page, limit, status])

  useEffect(() => {
    void load()
  }, [load])

  return (
    <div>
      <Space style={{ marginBottom: 16, width: '100%', justifyContent: 'space-between' }} wrap>
        <Typography.Title level={4} style={{ margin: 0 }}>
          页面管理
        </Typography.Title>
        <Button type="primary" onClick={() => navigate('/site-pages/new')}>
          新建页面
        </Button>
      </Space>
      <Space style={{ marginBottom: 16 }} wrap>
        <Select
          allowClear
          placeholder="状态筛选"
          style={{ width: 160 }}
          value={status}
          onChange={(v) => {
            setPage(1)
            setStatus(v)
          }}
          options={sitePageStatusOptions}
        />
      </Space>
      <Table
        rowKey="id"
        loading={loading}
        dataSource={items}
        scroll={{ x: 1000 }}
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
          { title: '标题', dataIndex: 'title', width: 180, ellipsis: true },
          { title: '别名', dataIndex: 'slug', width: 140, ellipsis: true },
          {
            title: '状态',
            dataIndex: 'status',
            width: 88,
            render: (s: number) => labelForSitePageStatus(s),
          },
          { title: '更新时间', dataIndex: 'updated_at', width: 180 },
          {
            title: '操作',
            key: 'a',
            fixed: 'right',
            width: 160,
            render: (_, row) => (
              <Space>
                <Button type="link" size="small" onClick={() => navigate(`/site-pages/${row.id}`)}>
                  详情
                </Button>
                <Popconfirm
                  title="确定删除？"
                  onConfirm={async () => {
                    const res = await mockServer.deletePage(row.id)
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
