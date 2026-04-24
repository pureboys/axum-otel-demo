import { App, Button, Form, Input, InputNumber, Modal, Popconfirm, Select, Space, Table, Typography } from 'antd'
import { useCallback, useEffect, useState } from 'react'
import { mockServer } from '../mock/server'
import type { Category, CategoryType } from '../types/api'

export function CategoriesPage() {
  const { message } = App.useApp()
  const [loading, setLoading] = useState(false)
  const [data, setData] = useState<Category[]>([])
  const [open, setOpen] = useState(false)
  const [editing, setEditing] = useState<Category | null>(null)
  const [form] = Form.useForm<{
    name: string
    slug: string
    description: string
    category_type: CategoryType
    parent_id?: number | null
  }>()

  const load = useCallback(async () => {
    setLoading(true)
    const res = await mockServer.listCategories()
    setLoading(false)
    if (res.code !== 0) {
      message.error(res.msg)
      return
    }
    setData(res.data)
  }, [message])

  useEffect(() => {
    void load()
  }, [load])

  const submit = async () => {
    const v = await form.validateFields()
    if (editing) {
      const res = await mockServer.updateCategory(editing.id, v)
      if (res.code !== 0) {
        message.error(res.msg)
        return
      }
      message.success('已更新')
    } else {
      const res = await mockServer.createCategory({
        name: v.name,
        slug: v.slug,
        description: v.description,
        category_type: v.category_type,
        parent_id: v.parent_id ?? null,
      })
      if (res.code !== 0) {
        message.error(res.msg)
        return
      }
      message.success('已创建')
    }
    setOpen(false)
    void load()
  }

  return (
    <div>
      <Space style={{ marginBottom: 16, width: '100%', justifyContent: 'space-between' }}>
        <Typography.Title level={4} style={{ margin: 0 }}>
          分类管理
        </Typography.Title>
        <Button
          type="primary"
          onClick={() => {
            setEditing(null)
            form.resetFields()
            form.setFieldsValue({ category_type: 'product' })
            setOpen(true)
          }}
        >
          新建分类
        </Button>
      </Space>
      <Table
        rowKey="id"
        loading={loading}
        dataSource={data}
        scroll={{ x: 960 }}
        columns={[
          { title: 'ID', dataIndex: 'id', width: 72 },
          { title: '名称', dataIndex: 'name' },
          { title: '别名', dataIndex: 'slug' },
          { title: '描述', dataIndex: 'description', ellipsis: true },
          {
            title: '类型',
            dataIndex: 'category_type',
            width: 100,
            render: (t: CategoryType) => (t === 'product' ? '产品' : '新闻'),
          },
          { title: '父级 ID', dataIndex: 'parent_id', width: 96, render: (v: number | null) => v ?? '—' },
          { title: '创建时间', dataIndex: 'created_at', width: 180 },
          {
            title: '操作',
            key: 'a',
            width: 160,
            fixed: 'right',
            render: (_, row) => (
              <Space>
                <Button
                  type="link"
                  size="small"
                  onClick={() => {
                    setEditing(row)
                    form.setFieldsValue({
                      name: row.name,
                      slug: row.slug,
                      description: row.description,
                      category_type: row.category_type,
                      parent_id: row.parent_id,
                    })
                    setOpen(true)
                  }}
                >
                  编辑
                </Button>
                <Popconfirm
                  title="确定删除？"
                  onConfirm={async () => {
                    const res = await mockServer.deleteCategory(row.id)
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
      <Modal
        title={editing ? '编辑分类' : '新建分类'}
        open={open}
        onOk={() => void submit()}
        onCancel={() => setOpen(false)}
        width={560}
        destroyOnHidden
      >
        <Form form={form} layout="vertical">
          <Form.Item name="name" label="名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="slug" label="别名" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="description" label="描述" rules={[{ required: true }]}>
            <Input.TextArea rows={3} />
          </Form.Item>
          <Form.Item name="category_type" label="分类类型" rules={[{ required: true }]}>
            <Select
              options={[
                { value: 'product', label: '产品' },
                { value: 'news', label: '新闻' },
              ]}
              disabled={!!editing}
            />
          </Form.Item>
          <Form.Item name="parent_id" label="父分类 ID">
            <InputNumber style={{ width: '100%' }} min={1} placeholder="可选" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  )
}
