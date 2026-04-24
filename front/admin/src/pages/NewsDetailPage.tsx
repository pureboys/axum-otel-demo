import { ArrowLeftOutlined, DeleteOutlined, EyeOutlined, SaveOutlined } from '@ant-design/icons'
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
  InputNumber,
  Row,
  Select,
  Space,
  Spin,
  Typography,
  theme,
} from 'antd'
import { useCallback, useEffect, useMemo, useState } from 'react'
import { Link, useNavigate, useParams } from 'react-router-dom'
import { ImageUrlField } from '../components/ImageUrlField'
import { HtmlRichEditor } from '../components/HtmlRichEditor'
import { labelForProductNewsStatus, productNewsStatusOptions } from '../constants/status'
import { mockServer } from '../mock/server'
import type { Category, NewsItem } from '../types/api'

export function NewsDetailPage() {
  const { token } = theme.useToken()
  const { message, modal } = App.useApp()
  const navigate = useNavigate()
  const { id } = useParams<{ id?: string }>()
  const isCreate = id === undefined
  const newsId = id != null ? Number(id) : NaN

  const [form] = Form.useForm<Partial<NewsItem>>()

  const [loading, setLoading] = useState(!isCreate)
  const [saving, setSaving] = useState(false)
  const [categories, setCategories] = useState<Category[]>([])
  const [snapshot, setSnapshot] = useState<NewsItem | null>(null)

  const newsCategories = useMemo(() => categories.filter((c) => c.category_type === 'news'), [categories])

  const loadMeta = useCallback(async () => {
    const res = await mockServer.listCategories()
    if (res.code === 0) setCategories(res.data)
  }, [])

  const loadNews = useCallback(async () => {
    if (isCreate) return
    if (!Number.isFinite(newsId)) {
      setLoading(false)
      message.error('无效的新闻 ID')
      navigate('/news', { replace: true })
      return
    }
    setLoading(true)
    const res = await mockServer.getNews(newsId)
    setLoading(false)
    if (res.code !== 0) {
      message.error(res.msg)
      navigate('/news', { replace: true })
      return
    }
    const n = res.data
    setSnapshot(n)
    form.setFieldsValue({
      title: n.title,
      slug: n.slug,
      content: n.content,
      excerpt: n.excerpt ?? undefined,
      cover_image: n.cover_image,
      category_id: n.category_id,
      author: n.author,
      status: n.status,
      is_featured: n.is_featured,
      published_at: n.published_at ?? undefined,
      meta_title: n.meta_title ?? undefined,
      meta_description: n.meta_description ?? undefined,
      view_count: n.view_count,
    })
  }, [isCreate, newsId, form, message, navigate])

  useEffect(() => {
    void loadMeta()
  }, [loadMeta])

  useEffect(() => {
    if (isCreate) {
      form.resetFields()
      form.setFieldsValue({ status: 1, is_featured: 0 })
      setSnapshot(null)
      return
    }
    void loadNews()
  }, [isCreate, loadNews, form])

  const titleVal = Form.useWatch('title', form)
  const categoryIdVal = Form.useWatch('category_id', form)

  const categoryName = useMemo(() => {
    if (categoryIdVal == null) return '—'
    return newsCategories.find((c) => c.id === categoryIdVal)?.name ?? `ID ${categoryIdVal}`
  }, [categoryIdVal, newsCategories])

  const onSave = async () => {
    const v = await form.validateFields()
    setSaving(true)
    if (isCreate) {
      const res = await mockServer.createNews({
        title: v.title!,
        slug: v.slug!,
        content: v.content!,
        cover_image: v.cover_image!,
        category_id: v.category_id!,
        author: v.author!,
        excerpt: v.excerpt,
        status: v.status,
        is_featured: v.is_featured,
        published_at: v.published_at,
        meta_title: v.meta_title,
        meta_description: v.meta_description,
      })
      setSaving(false)
      if (res.code !== 0) {
        message.error(res.msg)
        return
      }
      message.success('已创建')
      navigate(`/news/${res.data.id}`, { replace: true })
      return
    }
    const res = await mockServer.updateNews(newsId, v)
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
      title: '确定删除该新闻？',
      okType: 'danger',
      onOk: async () => {
        const res = await mockServer.deleteNews(newsId)
        if (res.code !== 0) {
          message.error(res.msg)
          return
        }
        message.success('已删除')
        navigate('/news', { replace: true })
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
          { title: <Link to="/news">新闻管理</Link> },
          { title: isCreate ? '新建新闻' : snapshot ? `编辑 · ${snapshot.title}` : '编辑新闻' },
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
            <Button icon={<ArrowLeftOutlined />} onClick={() => navigate('/news')}>
              返回列表
            </Button>
            <Divider type="vertical" />
            <EyeOutlined style={{ fontSize: 20, color: token.colorPrimary }} />
            <div>
              <Typography.Title level={4} style={{ margin: 0 }}>
                {isCreate ? '新建新闻' : titleVal || snapshot?.title || '编辑新闻'}
              </Typography.Title>
              <Typography.Text type="secondary">
                {isCreate
                  ? '左侧编辑正文与摘要，右侧设置封面、发布渠道与 SEO'
                  : `ID ${newsId} · 阅读 ${snapshot?.view_count ?? 0} · ${labelForProductNewsStatus(snapshot?.status ?? 1)}`}
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
            <Card title="标题与链接" variant="borderless" style={{ marginBottom: 24 }}>
              <Row gutter={16}>
                <Col xs={24} md={16}>
                  <Form.Item name="title" label="标题" rules={[{ required: true, message: '请输入标题' }]}>
                    <Input size="large" placeholder="醒目、准确的新闻标题" />
                  </Form.Item>
                </Col>
                <Col xs={24} md={8}>
                  <Form.Item name="slug" label="URL 别名" rules={[{ required: true, message: '请输入别名' }]}>
                    <Input size="large" placeholder="如 company-news" />
                  </Form.Item>
                </Col>
              </Row>
              <Form.Item name="excerpt" label="摘要" extra="用于列表与分享卡片，建议 1～2 句话概括全文">
                <Input.TextArea rows={3} showCount maxLength={500} placeholder="一句话概括新闻要点" />
              </Form.Item>
            </Card>

            <Card title="正文内容" variant="borderless" style={{ marginBottom: 24 }}>
              <Typography.Paragraph type="secondary" style={{ marginTop: 0 }}>
                可视化编辑与 HTML 源码可切换；支持插入图片与视频（演示环境上传为 Data URL）。
              </Typography.Paragraph>
              <Form.Item name="content" label="正文" rules={[{ required: true, message: '请输入正文' }]}>
                <HtmlRichEditor placeholder="撰写新闻正文…" minHeight={320} />
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
            <Card title="封面图" variant="borderless" style={{ marginBottom: 24 }}>
              <Form.Item
                name="cover_image"
                extra="可上传或粘贴地址（演示为本地 Data URL）"
                rules={[{ required: true, message: '请上传封面或填写地址' }]}
              >
                <ImageUrlField minHeight={200} />
              </Form.Item>
            </Card>

            <Card title="发布设置" variant="borderless" style={{ marginBottom: 24 }}>
              <Form.Item name="category_id" label="新闻分类" rules={[{ required: true, message: '请选择分类' }]}>
                <Select
                  size="large"
                  showSearch
                  optionFilterProp="label"
                  placeholder="选择分类"
                  options={newsCategories.map((c) => ({
                    value: c.id,
                    label: `${c.name} · ${c.slug}`,
                  }))}
                />
              </Form.Item>
              <Form.Item name="author" label="作者" rules={[{ required: true, message: '请输入作者' }]}>
                <Input placeholder="署名" />
              </Form.Item>
              <Row gutter={16}>
                <Col span={12}>
                  <Form.Item name="status" label="状态">
                    <Select options={productNewsStatusOptions} allowClear placeholder="状态" />
                  </Form.Item>
                </Col>
                <Col span={12}>
                  <Form.Item name="is_featured" label="精选">
                    <Select
                      options={[
                        { value: 0, label: '否' },
                        { value: 1, label: '是' },
                      ]}
                    />
                  </Form.Item>
                </Col>
              </Row>
              <Form.Item name="published_at" label="发布时间" extra="格式示例：2024-03-18 10:00:00">
                <Input placeholder="可留空由系统默认" />
              </Form.Item>
              <Descriptions size="small" column={1} bordered>
                <Descriptions.Item label="当前分类">{categoryName}</Descriptions.Item>
              </Descriptions>
            </Card>

            {!isCreate && snapshot && (
              <Card title="数据与审计" variant="borderless">
                <Descriptions column={1} size="small" style={{ marginBottom: 16 }}>
                  <Descriptions.Item label="新闻 ID">{snapshot.id}</Descriptions.Item>
                  <Descriptions.Item label="创建时间">{snapshot.created_at}</Descriptions.Item>
                  <Descriptions.Item label="最近更新">{snapshot.updated_at}</Descriptions.Item>
                </Descriptions>
                <Form.Item
                  name="view_count"
                  label="阅读量（可修正）"
                  extra="一般来自前台统计；Mock 下可手工调整"
                >
                  <InputNumber min={0} style={{ width: '100%' }} />
                </Form.Item>
              </Card>
            )}
          </Col>
        </Row>
      </Form>
    </div>
  )
}
