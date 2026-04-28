import {
  AppstoreOutlined,
  FileTextOutlined,
  FolderOutlined,
  LogoutOutlined,
  MailOutlined,
  ShoppingOutlined,
  TagsOutlined,
  UserOutlined,
} from '@ant-design/icons'
import { Avatar, Button, Dropdown, Layout, Menu, theme, Typography } from 'antd'
import type { MenuProps } from 'antd'
import { Outlet, useLocation, useNavigate } from 'react-router-dom'
import { useAuth } from '../context/useAuth'

const { Header, Sider, Content } = Layout

const menuItems: MenuProps['items'] = [
  { key: '/admins', icon: <UserOutlined />, label: '管理员管理' },
  { key: '/tags', icon: <TagsOutlined />, label: '标签管理' },
  { key: '/categories', icon: <FolderOutlined />, label: '分类管理' },
  { key: '/products', icon: <ShoppingOutlined />, label: '产品管理' },
  { key: '/news', icon: <FileTextOutlined />, label: '新闻管理' },
  { key: '/site-pages', icon: <AppstoreOutlined />, label: '页面管理' },
  { key: '/inquiries', icon: <MailOutlined />, label: '询盘管理' },
]

export function AdminLayout() {
  const navigate = useNavigate()
  const location = useLocation()
  const { profile, logout } = useAuth()
  const { token } = theme.useToken()

  const pathname = location.pathname
  const menuKey =
    menuItems
      ?.map((i) => (i && 'key' in i ? String(i.key) : ''))
      .filter(Boolean)
      .find((key) => pathname === key || pathname.startsWith(`${key}/`)) ?? null
  const selected = menuKey ? [menuKey] : []

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sider breakpoint="lg" collapsedWidth={64} theme="dark" width={220}>
        <div
          style={{
            height: 56,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            fontWeight: 600,
            color: '#fff',
            fontSize: 15,
            borderBottom: '1px solid rgba(255,255,255,0.08)',
          }}
        >
          管理后台
        </div>
        <Menu
          theme="dark"
          mode="inline"
          selectedKeys={selected}
          items={menuItems}
          onClick={({ key }) => navigate(String(key))}
        />
      </Sider>
      <Layout>
        <Header
          style={{
            padding: '0 20px',
            background: token.colorBgContainer,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'flex-end',
            borderBottom: `1px solid ${token.colorBorderSecondary}`,
          }}
        >
          <Dropdown
            menu={{
              items: [
                {
                  key: 'logout',
                  icon: <LogoutOutlined />,
                  label: '退出登录',
                  onClick: () => void logout().then(() => navigate('/login', { replace: true })),
                },
              ],
            }}
            placement="bottomRight"
          >
            <Button type="text" style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
              <Avatar size="small" style={{ backgroundColor: token.colorPrimary }}>
                {(profile?.nickname || profile?.username || 'A').slice(0, 1)}
              </Avatar>
              <Typography.Text>{profile?.nickname || profile?.username}</Typography.Text>
            </Button>
          </Dropdown>
        </Header>
        <Content style={{ margin: 16 }}>
          <div
            style={{
              padding: 20,
              minHeight: 360,
              background: token.colorBgContainer,
              borderRadius: token.borderRadiusLG,
            }}
          >
            <Outlet />
          </div>
        </Content>
      </Layout>
    </Layout>
  )
}
