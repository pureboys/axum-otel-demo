import { App, Badge, Button, Form, Input, Modal, Popconfirm, Select, Space, Table, Tag, Typography } from 'antd'
import { useCallback, useEffect, useState } from 'react'
import { mockServer } from '../mock/server'
import type { AdminUser } from '../types/api'

const ROLE_OPTIONS = [
  { value: 'admin', label: '管理员' },
  { value: 'super_admin', label: '超级管理员' },
]

const ROLE_COLORS: Record<string, string> = {
  super_admin: 'gold',
  admin: 'blue',
}

export function UsersPage() {
  const { message } = App.useApp()
  const [loading, setLoading] = useState(false)
  const [data, setData] = useState<AdminUser[]>([])
  const [open, setOpen] = useState(false)
  const [editing, setEditing] = useState<AdminUser | null>(null)
  const [pwdOpen, setPwdOpen] = useState(false)
  const [pwdTarget, setPwdTarget] = useState<AdminUser | null>(null)
  const [createForm] = Form.useForm<{ username: string; password: string; nickname?: string; role?: string }>()
  const [editForm] = Form.useForm<{ nickname?: string; role?: string; status?: number }>()
  const [pwdForm] = Form.useForm<{ new_password: string; confirm: string }>()

  const load = useCallback(async () => {
    setLoading(true)
    const res = await mockServer.listAdmins()
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
    createForm.resetFields()
    setOpen(true)
  }

  const openEdit = (row: AdminUser) => {
    setEditing(row)
    editForm.setFieldsValue({ nickname: row.nickname ?? undefined, role: row.role, status: row.status })
    setOpen(true)
  }

  const openChangePwd = (row: AdminUser) => {
    setPwdTarget(row)
    pwdForm.resetFields()
    setPwdOpen(true)
  }

  const submitPwd = async () => {
    const v = await pwdForm.validateFields()
    if (!pwdTarget) return
    const res = await mockServer.changeAdminPassword(pwdTarget.id, v.new_password)
    if (res.code !== 0) {
      message.error(res.msg)
      return
    }
    message.success('密码已修改')
    setPwdOpen(false)
  }

  const submit = async () => {
    if (editing) {
      const v = await editForm.validateFields()
      const res = await mockServer.updateAdmin(editing.id, v)
      if (res.code !== 0) {
        message.error(res.msg)
        return
      }
      message.success('已更新')
    } else {
      const v = await createForm.validateFields()
      const res = await mockServer.createAdmin(v)
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
          管理员管理
        </Typography.Title>
        <Button type="primary" onClick={openCreate}>
          新建管理员
        </Button>
      </Space>
      <Table
        rowKey="id"
        loading={loading}
        dataSource={data}
        columns={[
          { title: 'ID', dataIndex: 'id', width: 72 },
          { title: '用户名', dataIndex: 'username' },
          {
            title: '昵称',
            dataIndex: 'nickname',
            render: (v: string | null) => v ?? '-',
          },
          {
            title: '角色',
            dataIndex: 'role',
            render: (v: string) => (
              <Tag color={ROLE_COLORS[v] ?? 'default'}>
                {ROLE_OPTIONS.find((o) => o.value === v)?.label ?? v}
              </Tag>
            ),
          },
          {
            title: '状态',
            dataIndex: 'status',
            render: (v: number) =>
              v === 1 ? <Badge status="success" text="启用" /> : <Badge status="default" text="禁用" />,
          },
          {
            title: '最后登录',
            dataIndex: 'last_login_at',
            render: (v: string | null) => v ?? '-',
          },
          {
            title: '操作',
            key: 'actions',
            width: 160,
            render: (_, row) => (
              <Space>
                <Button type="link" size="small" onClick={() => openEdit(row)}>
                  编辑
                </Button>
                <Button type="link" size="small" onClick={() => openChangePwd(row)}>
                  改密码
                </Button>
                <Popconfirm
                  title="确定删除该管理员？"
                  onConfirm={async () => {
                    const res = await mockServer.deleteAdmin(row.id)
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
        title={`修改密码 - ${pwdTarget?.username ?? ''}`}
        open={pwdOpen}
        onOk={() => void submitPwd()}
        onCancel={() => setPwdOpen(false)}
        destroyOnHidden
      >
        <Form form={pwdForm} layout="vertical">
          <Form.Item name="new_password" label="新密码" rules={[{ required: true, min: 6, message: '密码至少 6 位' }]}>
            <Input.Password />
          </Form.Item>
          <Form.Item
            name="confirm"
            label="确认密码"
            dependencies={['new_password']}
            rules={[
              { required: true },
              ({ getFieldValue }) => ({
                validator(_, value) {
                  if (!value || getFieldValue('new_password') === value) return Promise.resolve()
                  return Promise.reject(new Error('两次密码不一致'))
                },
              }),
            ]}
          >
            <Input.Password />
          </Form.Item>
        </Form>
      </Modal>

      <Modal
        title={editing ? '编辑管理员' : '新建管理员'}
        open={open}
        onOk={() => void submit()}
        onCancel={() => setOpen(false)}
        destroyOnHidden
      >
        {editing ? (
          <Form form={editForm} layout="vertical">
            <Form.Item name="nickname" label="昵称">
              <Input placeholder="可选" />
            </Form.Item>
            <Form.Item name="role" label="角色" rules={[{ required: true }]}>
              <Select options={ROLE_OPTIONS} />
            </Form.Item>
            <Form.Item name="status" label="状态" rules={[{ required: true }]}>
              <Select
                options={[
                  { value: 1, label: '启用' },
                  { value: 0, label: '禁用' },
                ]}
              />
            </Form.Item>
          </Form>
        ) : (
          <Form form={createForm} layout="vertical">
            <Form.Item name="username" label="用户名" rules={[{ required: true }]}>
              <Input />
            </Form.Item>
            <Form.Item name="password" label="密码" rules={[{ required: true, min: 6 }]}>
              <Input.Password />
            </Form.Item>
            <Form.Item name="nickname" label="昵称">
              <Input placeholder="可选" />
            </Form.Item>
            <Form.Item name="role" label="角色" initialValue="admin">
              <Select options={ROLE_OPTIONS} />
            </Form.Item>
          </Form>
        )}
      </Modal>
    </div>
  )
}
