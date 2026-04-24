import { ArrowLeftOutlined, DeleteOutlined, SaveOutlined, ShoppingOutlined } from '@ant-design/icons'
import {
  App,
  Breadcrumb,
  Button,
  Card,
  Col,
  Descriptions,
  Divider,
  Form,
  Image,
  Input,
  InputNumber,
  Row,
  Select,
  Space,
  Spin,
  Tag,
  Typography,
  theme,
} from 'antd'
import { useCallback, useEffect, useMemo, useState } from 'react'
import { Link, useNavigate, useParams } from 'react-router-dom'
import { HtmlRichEditor } from '../components/HtmlRichEditor'
import { labelForProductNewsStatus, productNewsStatusOptions } from '../constants/status'
import { mockServer } from '../mock/server'
import type { Category, Product, Tag as ProductTagEntity } from '../types/api'

export function ProductDetailPage() {
  const { token } = theme.useToken()
  const { message, modal } = App.useApp()
  const navigate = useNavigate()
  const { id } = useParams<{ id?: string }>()
  const isCreate = id === undefined
  const productId = id != null ? Number(id) : NaN

  const [form] = Form.useForm<{
    name: string
    description: string
    price: number
    stock: number
    category_id: number
    image_url: string
    status?: number
    meta_title?: string
    meta_description?: string
  }>()

  const [loading, setLoading] = useState(!isCreate)
  const [saving, setSaving] = useState(false)
  const [categories, setCategories] = useState<Category[]>([])
  const [allTags, setAllTags] = useState<ProductTagEntity[]>([])
  const [tagIds, setTagIds] = useState<number[]>([])
  const [snapshot, setSnapshot] = useState<Product | null>(null)

  const productCategories = useMemo(
    () => categories.filter((c) => c.category_type === 'product'),
    [categories],
  )

  const loadMeta = useCallback(async () => {
    const [c, t] = await Promise.all([mockServer.listCategories(), mockServer.listTags()])
    if (c.code === 0) setCategories(c.data)
    if (t.code === 0) setAllTags(t.data)
  }, [])

  const loadProduct = useCallback(async () => {
    if (isCreate) return
    if (!Number.isFinite(productId)) {
      setLoading(false)
      message.error('无效的产品 ID')
      navigate('/products', { replace: true })
      return
    }
    setLoading(true)
    const [pRes, ptRes] = await Promise.all([
      mockServer.getProduct(productId),
      mockServer.getProductWithTags(productId),
    ])
    setLoading(false)
    if (pRes.code !== 0) {
      message.error(pRes.msg)
      navigate('/products', { replace: true })
      return
    }
    const p = pRes.data
    setSnapshot(p)
    form.setFieldsValue({
      name: p.name,
      description: p.description,
      price: p.price,
      stock: p.stock,
      category_id: p.category_id,
      image_url: p.image_url,
      status: p.status,
      meta_title: p.meta_title ?? undefined,
      meta_description: p.meta_description ?? undefined,
    })
    if (ptRes.code === 0) setTagIds(ptRes.data.tags.map((x) => x.id))
  }, [isCreate, productId, form, message, navigate])

  useEffect(() => {
    void loadMeta()
  }, [loadMeta])

  useEffect(() => {
    if (isCreate) {
      form.resetFields()
      form.setFieldsValue({ status: 1 })
      setTagIds([])
      setSnapshot(null)
      return
    }
    void loadProduct()
  }, [isCreate, loadProduct, form])

  const imageUrl = Form.useWatch('image_url', form)
  const nameVal = Form.useWatch('name', form)
  const categoryIdVal = Form.useWatch('category_id', form)

  const categoryName = useMemo(() => {
    if (categoryIdVal == null) return '—'
    return productCategories.find((c) => c.id === categoryIdVal)?.name ?? `ID ${categoryIdVal}`
  }, [categoryIdVal, productCategories])

  const onSave = async () => {
    const v = await form.validateFields()
    setSaving(true)
    if (isCreate) {
      const res = await mockServer.createProduct({
        ...v,
        status: v.status ?? 1,
        meta_title: v.meta_title ?? null,
        meta_description: v.meta_description ?? null,
      })
      setSaving(false)
      if (res.code !== 0) {
        message.error(res.msg)
        return
      }
      const newId = res.data.id
      const tagRes = await mockServer.setProductTags(newId, tagIds)
      if (tagRes.code !== 0) {
        message.warning('产品已创建，但标签保存失败：' + tagRes.msg)
        navigate(`/products/${newId}`, { replace: true })
        return
      }
      message.success('已创建')
      navigate(`/products/${newId}`, { replace: true })
      return
    }
    const res = await mockServer.updateProduct(productId, {
      ...v,
      meta_title: v.meta_title ?? null,
      meta_description: v.meta_description ?? null,
    })
    if (res.code !== 0) {
      setSaving(false)
      message.error(res.msg)
      return
    }
    const tagRes = await mockServer.setProductTags(productId, tagIds)
    setSaving(false)
    if (tagRes.code !== 0) {
      message.error(tagRes.msg)
      return
    }
    message.success('已保存')
    setSnapshot(res.data)
  }

  const onDelete = () => {
    if (isCreate) return
    modal.confirm({
      title: '确定删除该产品？',
      okType: 'danger',
      onOk: async () => {
        const res = await mockServer.deleteProduct(productId)
        if (res.code !== 0) {
          message.error(res.msg)
          return
        }
        message.success('已删除')
        navigate('/products', { replace: true })
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
          { title: <Link to="/products">产品管理</Link> },
          { title: isCreate ? '新建产品' : snapshot ? `编辑 · ${snapshot.name}` : '编辑产品' },
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
            <Button icon={<ArrowLeftOutlined />} onClick={() => navigate('/products')}>
              返回列表
            </Button>
            <Divider type="vertical" />
            <ShoppingOutlined style={{ fontSize: 20, color: token.colorPrimary }} />
            <div>
              <Typography.Title level={4} style={{ margin: 0 }}>
                {isCreate ? '新建产品' : nameVal || snapshot?.name || '编辑产品'}
              </Typography.Title>
              <Typography.Text type="secondary">
                {isCreate ? '填写下方信息完成创建，右侧可预览主图并设置分类与标签' : `ID ${productId} · ${labelForProductNewsStatus(snapshot?.status ?? 1)}`}
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
            <Card title="基本信息" variant="borderless" style={{ marginBottom: 24 }}>
              <Form.Item name="name" label="产品名称" rules={[{ required: true, message: '请输入名称' }]}>
                <Input size="large" placeholder="清晰、易搜索的商品名" />
              </Form.Item>
              <Form.Item
                name="description"
                label="产品描述"
                rules={[{ required: true, message: '请输入描述' }]}
                extra="用于详情页展示；支持图文、视频与 HTML 源码编辑（演示环境上传为 Data URL）"
              >
                <HtmlRichEditor placeholder="描述卖点、规格与使用场景…" minHeight={260} />
              </Form.Item>
            </Card>

            <Card title="SEO 与搜索" variant="borderless">
              <Typography.Paragraph type="secondary" style={{ marginTop: 0 }}>
                优化搜索引擎展示效果，留空时部分渠道将使用名称与描述自动填充。
              </Typography.Paragraph>
              <Form.Item name="meta_title" label="Meta 标题">
                <Input placeholder="建议 30 字以内" maxLength={120} showCount />
              </Form.Item>
              <Form.Item name="meta_description" label="Meta 描述">
                <Input.TextArea rows={3} placeholder="建议 80～160 字" maxLength={300} showCount />
              </Form.Item>
            </Card>
          </Col>

          <Col xs={24} lg={9} xl={8}>
            <Card title="主图预览" variant="borderless" style={{ marginBottom: 24 }}>
              <div
                style={{
                  background: token.colorFillAlter,
                  borderRadius: token.borderRadiusLG,
                  padding: 16,
                  textAlign: 'center',
                  marginBottom: 16,
                }}
              >
                {imageUrl ? (
                  <Image src={imageUrl} alt="" style={{ maxWidth: '100%', maxHeight: 280, objectFit: 'contain' }} />
                ) : (
                  <Typography.Text type="secondary">填写图片 URL 后在此预览</Typography.Text>
                )}
              </div>
              <Form.Item name="image_url" label="图片 URL" rules={[{ required: true, message: '请填写图片地址' }]}>
                <Input placeholder="https://..." />
              </Form.Item>
            </Card>

            <Card title="销售与库存" variant="borderless" style={{ marginBottom: 24 }}>
              <Row gutter={16}>
                <Col span={12}>
                  <Form.Item name="price" label="售价（元）" rules={[{ required: true }]}>
                    <InputNumber min={0} step={0.01} style={{ width: '100%' }} addonBefore="¥" />
                  </Form.Item>
                </Col>
                <Col span={12}>
                  <Form.Item name="stock" label="库存" rules={[{ required: true }]}>
                    <InputNumber min={0} style={{ width: '100%' }} />
                  </Form.Item>
                </Col>
              </Row>
              <Form.Item name="category_id" label="所属分类" rules={[{ required: true, message: '请选择分类' }]}>
                <Select
                  size="large"
                  showSearch
                  optionFilterProp="label"
                  placeholder="选择产品分类"
                  options={productCategories.map((c) => ({
                    value: c.id,
                    label: `${c.name} · ${c.slug}`,
                  }))}
                />
              </Form.Item>
              <Form.Item name="status" label="上架状态">
                <Select options={productNewsStatusOptions} placeholder="默认已发布" allowClear />
              </Form.Item>
              <Descriptions size="small" column={1} bordered style={{ marginTop: 8 }}>
                <Descriptions.Item label="当前分类">{categoryName}</Descriptions.Item>
              </Descriptions>
            </Card>

            <Card title="标签" variant="borderless" style={{ marginBottom: 24 }}>
              <Typography.Paragraph type="secondary" style={{ marginTop: 0 }}>
                用于前台筛选与推荐，可多选。
              </Typography.Paragraph>
              <Select
                mode="multiple"
                style={{ width: '100%' }}
                placeholder="选择标签"
                value={tagIds}
                onChange={setTagIds}
                options={allTags.map((t) => ({ value: t.id, label: `${t.name} (${t.slug})` }))}
              />
              <div style={{ marginTop: 12 }}>
                <Space wrap size={[8, 8]}>
                  {tagIds.map((tid) => {
                    const t = allTags.find((x) => x.id === tid)
                    return t ? <Tag key={tid}>{t.name}</Tag> : null
                  })}
                </Space>
              </div>
            </Card>

            {!isCreate && snapshot && (
              <Card title="系统信息" variant="borderless">
                <Descriptions column={1} size="small">
                  <Descriptions.Item label="产品 ID">{snapshot.id}</Descriptions.Item>
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
