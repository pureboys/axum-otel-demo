import {
  BoldOutlined,
  CodeOutlined,
  ItalicOutlined,
  LinkOutlined,
  OrderedListOutlined,
  PictureOutlined,
  UnderlineOutlined,
  UnorderedListOutlined,
  VideoCameraOutlined,
} from '@ant-design/icons'
import { App, Button, Input, Space, Tooltip, theme, Upload, type UploadProps } from 'antd'
import Image from '@tiptap/extension-image'
import Link from '@tiptap/extension-link'
import Placeholder from '@tiptap/extension-placeholder'
import Underline from '@tiptap/extension-underline'
import { EditorContent, useEditor } from '@tiptap/react'
import StarterKit from '@tiptap/starter-kit'
import { useEffect, useState } from 'react'
import { Video } from './editor/videoExtension'
import { uploadRichMediaToUrl } from '../utils/uploadRichMedia'

export type HtmlRichEditorProps = {
  value?: string
  onChange?: (html: string) => void
  placeholder?: string
  disabled?: boolean
  minHeight?: number
}

export function HtmlRichEditor({
  value = '',
  onChange,
  placeholder = '输入正文…',
  disabled = false,
  minHeight = 280,
}: HtmlRichEditorProps) {
  const { token } = theme.useToken()
  const { message } = App.useApp()
  const [htmlMode, setHtmlMode] = useState(false)

  const editor = useEditor({
    extensions: [
      StarterKit.configure({
        heading: { levels: [2, 3] },
        link: false,
      }),
      Underline,
      Link.configure({
        openOnClick: false,
        autolink: true,
        HTMLAttributes: { rel: 'noopener noreferrer', target: '_blank' },
      }),
      Image.configure({ allowBase64: true }),
      Video,
      Placeholder.configure({ placeholder }),
    ],
    content: value || '',
    editable: !disabled && !htmlMode,
    onUpdate: ({ editor: ed }) => {
      onChange?.(ed.getHTML())
    },
  })

  useEffect(() => {
    if (!editor) return
    editor.setEditable(!disabled && !htmlMode)
  }, [editor, disabled, htmlMode])

  useEffect(() => {
    if (!editor || htmlMode) return
    const cur = editor.getHTML()
    if (cur === value) return
    editor.commands.setContent(value || '', { emitUpdate: false })
  }, [editor, value, htmlMode])

  const imageUpload: UploadProps['customRequest'] = async (opt) => {
    const { file, onError, onSuccess } = opt
    if (!editor) return
    try {
      const url = await uploadRichMediaToUrl(file as File)
      onSuccess?.(url)
      editor.chain().focus().setImage({ src: url }).run()
    } catch (err) {
      onError?.(err instanceof Error ? err : new Error(String(err)))
      message.error(err instanceof Error ? err.message : '图片上传失败')
    }
  }

  const videoUpload: UploadProps['customRequest'] = async (opt) => {
    const { file, onError, onSuccess } = opt
    if (!editor) return
    try {
      const url = await uploadRichMediaToUrl(file as File)
      onSuccess?.(url)
      editor.chain().focus().insertContent({ type: 'video', attrs: { src: url } }).run()
    } catch (err) {
      onError?.(err instanceof Error ? err : new Error(String(err)))
      message.error(err instanceof Error ? err.message : '视频上传失败')
    }
  }

  const addLink = () => {
    if (!editor) return
    const prev = editor.getAttributes('link').href as string | undefined
    const url = window.prompt('链接地址', prev ?? 'https://')
    if (url === null) return
    const t = url.trim()
    if (t === '') {
      editor.chain().focus().extendMarkRange('link').unsetLink().run()
      return
    }
    editor.chain().focus().extendMarkRange('link').setLink({ href: t }).run()
  }

  const toggleHtml = () => {
    if (!editor) return
    if (!htmlMode) {
      onChange?.(editor.getHTML())
      setHtmlMode(true)
      return
    }
    setHtmlMode(false)
  }

  const border = `1px solid ${token.colorBorder}`
  const radius = token.borderRadiusLG

  return (
    <div className="html-rich-editor">
      <Space wrap style={{ marginBottom: 8 }}>
        <Tooltip title="粗体">
          <Button
            size="small"
            disabled={disabled || htmlMode || !editor}
            type={editor?.isActive('bold') ? 'primary' : 'default'}
            icon={<BoldOutlined />}
            onClick={() => editor?.chain().focus().toggleBold().run()}
          />
        </Tooltip>
        <Tooltip title="斜体">
          <Button
            size="small"
            disabled={disabled || htmlMode || !editor}
            type={editor?.isActive('italic') ? 'primary' : 'default'}
            icon={<ItalicOutlined />}
            onClick={() => editor?.chain().focus().toggleItalic().run()}
          />
        </Tooltip>
        <Tooltip title="下划线">
          <Button
            size="small"
            disabled={disabled || htmlMode || !editor}
            type={editor?.isActive('underline') ? 'primary' : 'default'}
            icon={<UnderlineOutlined />}
            onClick={() => editor?.chain().focus().toggleUnderline().run()}
          />
        </Tooltip>
        <Tooltip title="无序列表">
          <Button
            size="small"
            disabled={disabled || htmlMode || !editor}
            type={editor?.isActive('bulletList') ? 'primary' : 'default'}
            icon={<UnorderedListOutlined />}
            onClick={() => editor?.chain().focus().toggleBulletList().run()}
          />
        </Tooltip>
        <Tooltip title="有序列表">
          <Button
            size="small"
            disabled={disabled || htmlMode || !editor}
            type={editor?.isActive('orderedList') ? 'primary' : 'default'}
            icon={<OrderedListOutlined />}
            onClick={() => editor?.chain().focus().toggleOrderedList().run()}
          />
        </Tooltip>
        <Tooltip title="链接">
          <Button
            size="small"
            disabled={disabled || htmlMode || !editor}
            type={editor?.isActive('link') ? 'primary' : 'default'}
            icon={<LinkOutlined />}
            onClick={addLink}
          />
        </Tooltip>
        <Tooltip title="上传图片">
          <Upload
            accept="image/*"
            showUploadList={false}
            customRequest={imageUpload}
            disabled={disabled || htmlMode || !editor}
            multiple={false}
          >
            <Button size="small" icon={<PictureOutlined />} />
          </Upload>
        </Tooltip>
        <Tooltip title="上传视频">
          <Upload
            accept="video/*"
            showUploadList={false}
            customRequest={videoUpload}
            disabled={disabled || htmlMode || !editor}
            multiple={false}
          >
            <Button size="small" icon={<VideoCameraOutlined />} />
          </Upload>
        </Tooltip>
        <Tooltip title={htmlMode ? '返回可视化编辑' : '编辑 HTML 源码'}>
          <Button
            size="small"
            type={htmlMode ? 'primary' : 'default'}
            disabled={disabled || !editor}
            icon={<CodeOutlined />}
            onClick={toggleHtml}
          >
            HTML
          </Button>
        </Tooltip>
      </Space>

      {htmlMode ? (
        <Input.TextArea
          value={value}
          disabled={disabled}
          onChange={(e) => onChange?.(e.target.value)}
          rows={14}
          style={{ fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace', fontSize: 13 }}
        />
      ) : (
        <div
          style={{
            border,
            borderRadius: radius,
            background: token.colorBgContainer,
          }}
        >
          <EditorContent
            editor={editor}
            className="html-rich-editor-content"
            style={{ minHeight }}
          />
        </div>
      )}

      <style>{`
        .html-rich-editor .html-rich-editor-content .tiptap {
          min-height: ${minHeight}px;
          padding: 12px 14px;
          outline: none;
        }
        .html-rich-editor .html-rich-editor-content .tiptap:focus {
          outline: none;
        }
        .html-rich-editor .html-rich-editor-content .tiptap p.is-empty::before {
          content: attr(data-placeholder);
          float: left;
          color: ${token.colorTextPlaceholder};
          pointer-events: none;
          height: 0;
        }
        .html-rich-editor .html-rich-editor-content .tiptap img {
          max-width: 100%;
          height: auto;
          border-radius: 6px;
        }
        .html-rich-editor .html-rich-editor-content .tiptap video {
          max-width: 100%;
          border-radius: 8px;
        }
        .html-rich-editor .html-rich-editor-content .tiptap h2 {
          font-size: 1.35em;
          margin: 0.75em 0 0.35em;
        }
        .html-rich-editor .html-rich-editor-content .tiptap h3 {
          font-size: 1.15em;
          margin: 0.65em 0 0.3em;
        }
        .html-rich-editor .html-rich-editor-content .tiptap ul,
        .html-rich-editor .html-rich-editor-content .tiptap ol {
          padding-left: 1.25em;
        }
      `}</style>
    </div>
  )
}
