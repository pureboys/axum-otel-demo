import { App, Button, Popconfirm, Space, Table, Tag, Typography } from 'antd'
import { useCallback, useEffect, useState } from 'react'
import { mockServer } from '../mock/server'
import type { Inquiry } from '../types/api'

export function InquiriesPage() {
  const { message } = App.useApp()
  const [loading, setLoading] = useState(false)
  const [deleting, setDeleting] = useState(false)
  const [items, setItems] = useState<Inquiry[]>([])
  const [total, setTotal] = useState(0)
  const [page, setPage] = useState(1)
  const [limit, setLimit] = useState(20)
  const [selectedIds, setSelectedIds] = useState<number[]>([])

  const load = useCallback(async () => {
    setLoading(true)
    const res = await mockServer.listInquiries({ page, limit })
    setLoading(false)
    if (res.code !== 0) {
      message.error(res.msg)
      return
    }
    setItems(res.data.items)
    setTotal(res.data.total)
    setSelectedIds([])
  }, [message, page, limit])

  useEffect(() => {
    void load()
  }, [load])

  const handleBatchDelete = useCallback(async () => {
    setDeleting(true)
    let failCount = 0
    for (const id of selectedIds) {
      const res = await mockServer.deleteInquiry(id)
      if (res.code !== 0) failCount++
    }
    setDeleting(false)
    if (failCount > 0) {
      message.warning(`${selectedIds.length - failCount} 条已删除，${failCount} 条失败`)
    } else {
      message.success(`已删除 ${selectedIds.length} 条询盘`)
    }
    void load()
  }, [selectedIds, message, load])

  return (
    <div>
      <Space style={{ marginBottom: 16, width: '100%', justifyContent: 'space-between' }} wrap>
        <Typography.Title level={4} style={{ margin: 0 }}>
          询盘管理
        </Typography.Title>
        {selectedIds.length > 0 && (
          <Popconfirm
            title={`确定删除选中的 ${selectedIds.length} 条询盘？`}
            onConfirm={handleBatchDelete}
            okText="删除"
            okButtonProps={{ danger: true }}
          >
            <Button danger loading={deleting}>
              批量删除（{selectedIds.length}）
            </Button>
          </Popconfirm>
        )}
      </Space>
      <Table
        rowKey="id"
        loading={loading}
        dataSource={items}
        scroll={{ x: 900 }}
        rowSelection={{
          selectedRowKeys: selectedIds,
          onChange: (keys) => setSelectedIds(keys as number[]),
        }}
        pagination={{
          current: page,
          pageSize: limit,
          total,
          showSizeChanger: true,
          pageSizeOptions: ['10', '20', '50'],
          onChange: (p, ps) => {
            setPage(p)
            setLimit(ps || 20)
          },
        }}
        columns={[
          { title: 'ID', dataIndex: 'id', width: 72 },
          { title: '联系人', dataIndex: 'name', width: 120 },
          {
            title: '联系方式',
            key: 'contact',
            width: 200,
            render: (_: unknown, row: Inquiry) => (
              <Space direction="vertical" size={0}>
                {row.phone && <span>{row.phone}</span>}
                {row.email && <span style={{ color: '#666', fontSize: 12 }}>{row.email}</span>}
                {!row.phone && !row.email && <span style={{ color: '#999' }}>—</span>}
              </Space>
            ),
          },
          {
            title: '关联产品',
            key: 'product',
            width: 160,
            render: (_: unknown, row: Inquiry) =>
              row.product_name ? (
                <Tag color="blue">{row.product_name}</Tag>
              ) : (
                <span style={{ color: '#999' }}>—</span>
              ),
          },
          {
            title: '留言内容',
            dataIndex: 'message',
            ellipsis: true,
          },
          { title: '提交时间', dataIndex: 'created_at', width: 180 },
          {
            title: '操作',
            key: 'action',
            fixed: 'right',
            width: 100,
            render: (_: unknown, row: Inquiry) => (
              <Popconfirm
                title="确定删除此询盘？"
                onConfirm={async () => {
                  const res = await mockServer.deleteInquiry(row.id)
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
            ),
          },
        ]}
      />
    </div>
  )
}
