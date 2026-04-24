import { getProfile, getToken } from '../auth/storage'
import { mockServer } from '../mock/server'
import type { AdminProfile, ApiResponse, LoginData, RefreshData } from '../types/api'
import { sleep } from '../utils/sleep'

export async function login(username: string, password: string, altcha: string) {
  return mockServer.login(username, password, altcha)
}

export async function logout() {
  return mockServer.logout()
}

export async function refreshToken() {
  return mockServer.refresh()
}

export async function fetchAdminInfo(): Promise<ApiResponse<AdminProfile>> {
  await sleep(80)
  const t = getToken()
  if (!t) return { code: 401, msg: '未授权/认证失败', data: null as unknown as AdminProfile }
  const p = getProfile()
  if (!p) return { code: 401, msg: '未授权/认证失败', data: null as unknown as AdminProfile }
  return { code: 0, msg: '', data: { ...p } }
}

export type { LoginData, RefreshData }
