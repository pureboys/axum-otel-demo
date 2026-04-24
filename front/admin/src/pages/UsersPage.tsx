import { App, Button, Form, Input, Modal, Popconfirm, Space, Table, Typography } from 'antd'
import { useCallback, useEffect, useState } from 'react'
import { mockServer } from '../mock/server'
import type { EndUser } from '../types/api'

export function UsersPage() {
  const { message } = App.useApp()
  const [loading, setLoading] = useState(false)
  const [data, setData] = useState<EndUser[]>([])
  const [open, setOpen] = useState(false)
  const [editing, setEditing] = useState<EndUser | null>(null)
  const [form] = Form.useForm<{ username: string; email: string }>()

  const load = useCallback(async () => {
    setLoading(true)
    const res = await mockServer.listUsers()
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

  const openCreate = () => {
    setEditing(null)
    form.resetFields()
    setOpen(true)
  }

  const openEdit = (row: EndUser) => {
    setEditing(row)
    form.setFieldsValue({ username: row.username, email: row.email })
    setOpen(true)
  }

  const submit = async () => {
    const v = await form.validateFields()
    if (editing) {
      const res = await mockServer.updateUser(editing.id, v)
      if (res.code !== 0) {
        message.error(res.msg)
        return
      }
      message.success('已更新')
    } else {
      const res = await mockServer.createUser(v)
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
          用户管理
        </Typography.Title>
        <Button type="primary" onClick={openCreate}>
          新建用户
        </Button>
      </Space>
      <Table
        rowKey="id"
        loading={loading}
        dataSource={data}
        columns={[
          { title: 'ID', dataIndex: 'id', width: 72 },
          { title: '用户名', dataIndex: 'username' },
          { title: '邮箱', dataIndex: 'email' },
          {
            title: '操作',
            key: 'actions',
            width: 160,
            render: (_, row) => (
              <Space>
                <Button type="link" size="small" onClick={() => openEdit(row)}>
                  编辑
                </Button>
                <Popconfirm
                  title="确定删除？"
                  onConfirm={async () => {
                    const res = await mockServer.deleteUser(row.id)
                    if (res.code !== 0) {
                      message.error(res.msg)
                      return
                    }
                    message.success('已删除')
                    void load()
                  }}
                >
                  <Button type="link" size="small" danger>
                    删除
                  </Button>
                </Popconfirm>
              </Space>
            ),
          },
        ]}
      />
      <Modal
        title={editing ? '编辑用户' : '新建用户'}
        open={open}
        onOk={() => void submit()}
        onCancel={() => setOpen(false)}
        destroyOnHidden
      >
        <Form form={form} layout="vertical">
          <Form.Item name="username" label="用户名" rules={[{ required: true }]}>
            <Input />
          </Form.Item>
          <Form.Item name="email" label="邮箱" rules={[{ required: true, type: 'email' }]}>
            <Input />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  )
}
