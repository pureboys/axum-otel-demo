import { ArrowLeftOutlined, DeleteOutlined, FileTextOutlined, SaveOutlined } from '@ant-design/icons'
import {
  App,
  Breadcrumb,
  Button,
  Card,
  Col,
  Descriptions,
  Divider,
  Form,
  Input,
  Row,
  Select,
  Space,
  Spin,
  Typography,
  theme,
} from 'antd'
import { useCallback, useEffect, useState } from 'react'
import { Link, useNavigate, useParams } from 'react-router-dom'
import { HtmlRichEditor } from '../components/HtmlRichEditor'
import { labelForSitePageStatus, sitePageStatusOptions } from '../constants/sitePage'
import { mockServer } from '../mock/server'
import type { CmsPage } from '../types/api'

export function SitePageDetailPage() {
  const { token } = theme.useToken()
  const { message, modal } = App.useApp()
  const navigate = useNavigate()
  const { id } = useParams<{ id?: string }>()
  const isCreate = id === undefined
  const pageId = id != null ? Number(id) : NaN

  const [form] = Form.useForm<Partial<CmsPage>>()

  const [loading, setLoading] = useState(!isCreate)
  const [saving, setSaving] = useState(false)
  const [snapshot, setSnapshot] = useState<CmsPage | null>(null)

  const loadPage = useCallback(async () => {
    if (isCreate) return
    if (!Number.isFinite(pageId)) {
      setLoading(false)
      message.error('无效的页面 ID')
      navigate('/site-pages', { replace: true })
      return
    }
    setLoading(true)
    const res = await mockServer.getPage(pageId)
    setLoading(false)
    if (res.code !== 0) {
      message.error(res.msg)
      navigate('/site-pages', { replace: true })
      return
    }
    const p = res.data
    setSnapshot(p)
    form.setFieldsValue({
      title: p.title,
      slug: p.slug,
      content: p.content,
      meta_title: p.meta_title ?? undefined,
      meta_description: p.meta_description ?? undefined,
      status: p.status,
    })
  }, [isCreate, pageId, form, message, navigate])

  useEffect(() => {
    if (isCreate) {
      form.resetFields()
      form.setFieldsValue({ status: 1 })
      setSnapshot(null)
      return
    }
    void loadPage()
  }, [isCreate, loadPage, form])

  const titleVal = Form.useWatch('title', form)
  const slugVal = Form.useWatch('slug', form)
  const statusVal = Form.useWatch('status', form)

  const onSave = async () => {
    const v = await form.validateFields()
    setSaving(true)
    if (isCreate) {
      const res = await mockServer.createPage({
        title: v.title!,
        slug: v.slug!,
        content: v.content!,
        meta_title: v.meta_title,
        meta_description: v.meta_description,
        status: v.status,
      })
      setSaving(false)
      if (res.code !== 0) {
        message.error(res.msg)
        return
      }
      message.success('已创建')
      navigate(`/site-pages/${res.data.id}`, { replace: true })
      return
    }
    const res = await mockServer.updatePage(pageId, v)
    setSaving(false)
    if (res.code !== 0) {
      message.error(res.msg)
      return
    }
    message.success('已保存')
    setSnapshot(res.data)
  }

  const onDelete = () => {
    if (isCreate) return
    modal.confirm({
      title: '确定删除该页面？',
      okType: 'danger',
      onOk: async () => {
        const res = await mockServer.deletePage(pageId)
        if (res.code !== 0) {
          message.error(res.msg)
          return
        }
        message.success('已删除')
        navigate('/site-pages', { replace: true })
      },
    })
  }

  if (!isCreate && loading) {
    return (
      <div style={{ textAlign: 'center', padding: 80 }}>
        <Spin size="large" />
      </div>
    )
  }

  return (
    <div style={{ maxWidth: 1280, margin: '0 auto' }}>
      <Breadcrumb
        style={{ marginBottom: 16 }}
        items={[
          { title: <Link to="/site-pages">页面管理</Link> },
          { title: isCreate ? '新建页面' : snapshot ? `编辑 · ${snapshot.title}` : '编辑页面' },
        ]}
      />

      <div
        style={{
          position: 'sticky',
          top: 0,
          zIndex: 5,
          marginBottom: 20,
          paddingBottom: 12,
          background: token.colorBgContainer,
          borderBottom: `1px solid ${token.colorBorderSecondary}`,
        }}
      >
        <Space wrap style={{ width: '100%', justifyContent: 'space-between' }}>
          <Space wrap align="center">
            <Button icon={<ArrowLeftOutlined />} onClick={() => navigate('/site-pages')}>
              返回列表
            </Button>
            <Divider type="vertical" />
            <FileTextOutlined style={{ fontSize: 20, color: token.colorPrimary }} />
            <div>
              <Typography.Title level={4} style={{ margin: 0 }}>
                {isCreate ? '新建页面' : titleVal || snapshot?.title || '编辑页面'}
              </Typography.Title>
              <Typography.Text type="secondary">
                {isCreate
                  ? '左侧编辑页面结构与正文，右侧配置访问状态与 SEO 摘要'
                  : `ID ${pageId} · /${slugVal || snapshot?.slug || '…'} · ${labelForSitePageStatus(snapshot?.status ?? 1)}`}
              </Typography.Text>
            </div>
          </Space>
          <Space wrap>
            {!isCreate && (
              <Button danger icon={<DeleteOutlined />} onClick={onDelete}>
                删除
              </Button>
            )}
            <Button type="primary" icon={<SaveOutlined />} loading={saving} onClick={() => void onSave()}>
              保存
            </Button>
          </Space>
        </Space>
      </div>

      <Form form={form} layout="vertical">
        <Row gutter={[24, 24]}>
          <Col xs={24} lg={15} xl={16}>
            <Card title="页面信息" variant="borderless" style={{ marginBottom: 24 }}>
              <Row gutter={16}>
                <Col xs={24} md={14}>
                  <Form.Item name="title" label="页面标题" rules={[{ required: true, message: '请输入标题' }]}>
                    <Input size="large" placeholder="如：关于我们、服务条款" />
                  </Form.Item>
                </Col>
                <Col xs={24} md={10}>
                  <Form.Item
                    name="slug"
                    label="URL 别名"
                    rules={[{ required: true, message: '请输入别名' }]}
                    extra="前台路径通常为 /pages/{slug} 或 /{slug}，仅小写字母、数字与连字符为佳"
                  >
                    <Input size="large" placeholder="about-us" addonBefore="/" />
                  </Form.Item>
                </Col>
              </Row>
            </Card>

            <Card title="页面正文" variant="borderless" style={{ marginBottom: 24 }}>
              <Typography.Paragraph type="secondary" style={{ marginTop: 0 }}>
                适用于公司介绍、政策说明等；可切换 HTML 源码，并上传图片与视频（演示环境为 Data URL）。
              </Typography.Paragraph>
              <Form.Item name="content" label="页面内容" rules={[{ required: true, message: '请输入正文' }]}>
                <HtmlRichEditor placeholder="编辑页面正文…" minHeight={320} />
              </Form.Item>
            </Card>

            <Card title="SEO" variant="borderless">
              <Form.Item name="meta_title" label="Meta 标题">
                <Input maxLength={120} showCount placeholder="覆盖浏览器标题，可选" />
              </Form.Item>
              <Form.Item name="meta_description" label="Meta 描述">
                <Input.TextArea rows={3} maxLength={300} showCount placeholder="搜索结果摘要" />
              </Form.Item>
            </Card>
          </Col>

          <Col xs={24} lg={9} xl={8}>
            <Card title="访问与状态" variant="borderless" style={{ marginBottom: 24 }}>
              <Typography.Paragraph type="secondary" style={{ marginTop: 0 }}>
                禁用后前台可按业务规则隐藏入口或返回 404。
              </Typography.Paragraph>
              <Form.Item name="status" label="页面状态">
                <Select size="large" options={sitePageStatusOptions} />
              </Form.Item>
              <Descriptions size="small" column={1} bordered>
                <Descriptions.Item label="当前别名">
                  {slugVal || snapshot?.slug ? `/${slugVal ?? snapshot?.slug}` : '—'}
                </Descriptions.Item>
                <Descriptions.Item label="状态说明">
                  {labelForSitePageStatus(Number(statusVal ?? snapshot?.status ?? 1))}
                </Descriptions.Item>
              </Descriptions>
            </Card>

            <Card title="编辑提示" variant="borderless" style={{ marginBottom: 24 }}>
              <ul style={{ margin: 0, paddingLeft: 20, color: token.colorTextSecondary, lineHeight: 1.8 }}>
                <li>标题与别名确定后尽量避免频繁修改，以免影响已收录 URL。</li>
                <li>工具栏「HTML」可在可视化与源码间切换；复杂结构可直接改源码。</li>
                <li>SEO 字段留空时，部分渠道会使用标题与正文前段自动生成摘要。</li>
              </ul>
            </Card>

            {!isCreate && snapshot && (
              <Card title="系统信息" variant="borderless">
                <Descriptions column={1} size="small">
                  <Descriptions.Item label="页面 ID">{snapshot.id}</Descriptions.Item>
                  <Descriptions.Item label="创建时间">{snapshot.created_at}</Descriptions.Item>
                  <Descriptions.Item label="最近更新">{snapshot.updated_at}</Descriptions.Item>
                </Descriptions>
              </Card>
            )}
          </Col>
        </Row>
      </Form>
    </div>
  )
}
