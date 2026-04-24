import 'react'

type AltchaWidgetProps = {
  /** PoW 挑战地址（与后端 challenge 端点一致） */
  challenge?: string
  /** JSON 配置字符串，例如 '{"test":true}' 开启测试模式 */
  configuration?: string
  language?: string
  name?: string
  auto?: string
  display?: string
  theme?: string
  type?: string
  workers?: number
}

declare global {
  namespace React {
    namespace JSX {
      interface IntrinsicElements {
        'altcha-widget': React.DetailedHTMLProps<
          React.HTMLAttributes<HTMLElement> & AltchaWidgetProps,
          HTMLElement
        >
      }
    }
  }
}

export {}
