import { App, Button, Form, Input, Modal, Popconfirm, Space, Table, Typography } from 'antd'
import { useCallback, useEffect, useState } from 'react'
import { mockServer } from '../mock/server'
import type { Tag } from '../types/api'

export function TagsPage() {
  const { message } = App.useApp()
  const [loading, setLoading] = useState(false)
  const [data, setData] = useState<Tag[]>([])
  const [open, setOpen] = useState(false)
  const [editing, setEditing] = useState<Tag | null>(null)
  const [form] = Form.useForm<{ name: string; slug: string }>()

  const load = useCallback(async () => {
    setLoading(true)
    const res = await mockServer.listTags()
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
      const res = await mockServer.updateTag(editing.id, v)
      if (res.code !== 0) {
        message.error(res.msg)
        return
      }
      message.success('已更新')
    } else {
      const res = await mockServer.createTag(v)
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
          标签管理
        </Typography.Title>
        <Button
          type="primary"
          onClick={() => {
            setEditing(null)
            form.resetFields()
            setOpen(true)
          }}
        >
          新建标签
        </Button>
      </Space>
      <Table
        rowKey="id"
        loading={loading}
        dataSource={data}
        columns={[
          { title: 'ID', dataIndex: 'id', width: 72 },
          { title: '名称', dataIndex: 'name' },
          { title: '别名', dataIndex: 'slug' },
          { title: '创建时间', dataIndex: 'created_at', width: 180 },
          {
            title: '操作',
            key: 'a',
            width: 160,
            render: (_, row) => (
              <Space>
                <Button
                  type="link"
                  size="small"
                  onClick={() => {
                    setEditing(row)
                    form.setFieldsValue({ name: row.name, slug: row.slug })
                    setOpen(true)
                  }}
                >
                  编辑
                </Button>
                <Popconfirm
                  title="确定删除？"
                  onConfirm={async () => {
                    const res = await mockServer.deleteTag(row.id)
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
        title={editing ? '编辑标签' : '新建标签'}
        open={open}
        onOk={() => void submit()}
        onCancel={() => setOpen(false)}
        destroyOnHidden
      >
        <Form form={form} layout="vertical">
          <Form.Item name="name" label="名称" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="slug" label="别名" rules={[{ required: true }]}>
            <Input placeholder="URL 友好，如 hot" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  )
}
