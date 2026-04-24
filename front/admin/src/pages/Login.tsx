import { LockOutlined, UserOutlined } from '@ant-design/icons'
import { App, Button, Card, Form, Input, Typography } from 'antd'
import { useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '../context/useAuth'

/** 未设置时：开发环境用 test 模式；设置 VITE_ALTCHA_LIVE=1 时走真实 /api 挑战（需后端与 Vite 代理） */
const altchaTestMode = import.meta.env.DEV && import.meta.env.VITE_ALTCHA_LIVE !== '1'

export function LoginPage() {
  const { message } = App.useApp()
  const navigate = useNavigate()
  const { login, token } = useAuth()

  useEffect(() => {
    if (token) navigate('/users', { replace: true })
  }, [token, navigate])

  return (
    <div
      style={{
        minHeight: '100vh',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        background: 'linear-gradient(135deg, #f0f5ff 0%, #ffffff 50%, #f6ffed 100%)',
      }}
    >
      <Card style={{ width: 400, boxShadow: '0 8px 24px rgba(0,0,0,0.08)' }}>
        <Typography.Title level={3} style={{ textAlign: 'center', marginBottom: 24 }}>
          管理员登录
        </Typography.Title>
        <Form
          id="admin-login-form"
          layout="vertical"
          onFinish={async (v) => {
            const inForm =
              (document.getElementById('admin-login-form') as HTMLFormElement | null)?.querySelector<
                HTMLInputElement
              >('input[name="altcha"]')?.value?.trim() ?? ''
            const altcha =
              inForm || (document.querySelector<HTMLInputElement>('input[name="altcha"]')?.value?.trim() ?? '')
            if (!altcha) {
              message.error('请完成人机验证')
              return
            }
            await login(v.username, v.password, altcha)
            navigate('/users', { replace: true })
          }}
        >
          <Form.Item name="username" label="用户名" rules={[{ required: true, message: '请输入用户名' }]}>
            <Input prefix={<UserOutlined />} placeholder="admin" autoComplete="username" />
          </Form.Item>
          <Form.Item name="password" label="密码" rules={[{ required: true, message: '请输入密码' }]}>
            <Input.Password prefix={<LockOutlined />} placeholder="任意非空（Mock）" autoComplete="current-password" />
          </Form.Item>
          <Form.Item label="验证">
            <div style={{ minHeight: 56 }}>
              <altcha-widget
                language="zh-cn"
                name="altcha"
                configuration={altchaTestMode ? '{"test":true}' : undefined}
                challenge={altchaTestMode ? undefined : `${window.location.origin}/api/common/altcha/challenge`}
                auto="onload"
              />
            </div>
            <Typography.Text type="secondary" style={{ display: 'block', marginTop: 8, fontSize: 12 }}>
              {altchaTestMode
                ? '开发模式：已启用模拟验证。联调真实后端时请在 .env 设置 VITE_ALTCHA_LIVE=1。'
                : '使用服务端 PoW 挑战。'}
            </Typography.Text>
          </Form.Item>
          <Form.Item>
            <Button type="primary" htmlType="submit" block size="large">
              登录
            </Button>
          </Form.Item>
        </Form>
      </Card>
    </div>
  )
}
