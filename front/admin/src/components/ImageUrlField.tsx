import { PlusOutlined } from '@ant-design/icons'
import { App, Input, theme, Upload, type UploadFile, type UploadProps } from 'antd'
import { useEffect, useState } from 'react'
import { uploadRichMediaToUrl } from '../utils/uploadRichMedia'

export type ImageUrlFieldProps = {
  value?: string
  onChange?: (url: string) => void
  disabled?: boolean
  /** 上传选择区域最小高度 */
  minHeight?: number
  /** 是否显示「或粘贴 URL」输入框 */
  showUrlInput?: boolean
}

/**
 * 单图：Ant Design `listType="picture-card"` 上传 + 可选手动 URL。
 */
export function ImageUrlField({
  value = '',
  onChange,
  disabled = false,
  minHeight = 200,
  showUrlInput = true,
}: ImageUrlFieldProps) {
  const { token } = theme.useToken()
  const { message } = App.useApp()
  const [fileList, setFileList] = useState<UploadFile[]>([])

  useEffect(() => {
    if (value) {
      setFileList([{ uid: 'image-1', name: 'image', status: 'done', url: value, thumbUrl: value }])
    } else {
      setFileList((fl) => (fl[0]?.status === 'uploading' ? fl : []))
    }
  }, [value])

  const customRequest: UploadProps['customRequest'] = async (options) => {
    const { file, onError, onSuccess } = options
    try {
      const url = await uploadRichMediaToUrl(file as File)
      onSuccess?.(url)
      onChange?.(url)
    } catch (e) {
      onError?.(e instanceof Error ? e : new Error(String(e)))
      message.error(e instanceof Error ? e.message : '上传失败')
    }
  }

  const onUploadChange: UploadProps['onChange'] = (info) => {
    setFileList(info.fileList)
    if (info.fileList.length === 0) onChange?.('')
  }

  const beforeUpload: UploadProps['beforeUpload'] = (file) => {
    if (!file.type.startsWith('image/')) {
      message.error('请选择图片文件')
      return Upload.LIST_IGNORE
    }
    return true
  }

  return (
    <div className="image-url-field" style={{ width: '100%' }}>
      <style>{`
        .image-url-field .ant-upload-select {
          width: 100% !important;
          height: ${minHeight}px !important;
          margin: 0 !important;
        }
        .image-url-field .ant-upload-list-item-container {
          width: 100% !important;
          height: ${minHeight}px !important;
        }
        .image-url-field .ant-upload-list-item {
          padding: 8px;
        }
      `}</style>
      <Upload
        listType="picture-card"
        accept="image/*"
        fileList={fileList}
        disabled={disabled}
        maxCount={1}
        beforeUpload={beforeUpload}
        customRequest={customRequest}
        onChange={onUploadChange}
      >
        {fileList.length >= 1 ? null : (
          <div
            style={{
              color: token.colorTextSecondary,
              display: 'flex',
              width: '100%',
              height: '100%',
              flexDirection: 'column',
              alignItems: 'center',
              justifyContent: 'center',
            }}
          >
            <PlusOutlined style={{ fontSize: 24, marginBottom: 8, color: token.colorPrimary }} />
            <span style={{ fontSize: 13 }}>上传</span>
          </div>
        )}
      </Upload>
      {showUrlInput && (
        <Input
          style={{ marginTop: 12 }}
          placeholder="或粘贴图片 URL"
          value={value}
          disabled={disabled}
          onChange={(e) => onChange?.(e.target.value)}
          allowClear
        />
      )}
    </div>
  )
}
