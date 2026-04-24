import { mockServer } from '../mock/server'

/** 演示环境：与富媒体编辑器相同的上传逻辑（Data URL） */
export async function uploadRichMediaToUrl(file: File): Promise<string> {
  const res = await mockServer.uploadRichMedia(file)
  if (res.code !== 0) throw new Error(res.msg)
  return res.data.url
}
