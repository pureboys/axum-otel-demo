import { App } from 'antd'
import type { ReactNode } from 'react'
import { useCallback, useEffect, useMemo, useState } from 'react'
import * as authApi from '../api/auth'
import { clearSession, getProfile, getToken, setSession, updateToken } from '../auth/storage'
import type { AdminProfile } from '../types/api'
import { AuthContext, type AuthContextValue } from './authContext'

export function AuthProvider({ children }: { children: ReactNode }) {
  const { message } = App.useApp()
  const [token, setToken] = useState<string | null>(() => getToken())
  const [profile, setProfile] = useState<AdminProfile | null>(() => getProfile())
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    let cancelled = false
    ;(async () => {
      const t = getToken()
      if (!t) {
        if (!cancelled) setLoading(false)
        return
      }
      const res = await authApi.fetchAdminInfo()
      if (cancelled) return
      if (res.code !== 0) {
        clearSession()
        setToken(null)
        setProfile(null)
      } else {
        setProfile(res.data)
      }
      setLoading(false)
    })()
    return () => {
      cancelled = true
    }
  }, [])

  const login = useCallback(
    async (username: string, password: string, altcha: string) => {
      const res = await authApi.login(username, password, altcha)
      if (res.code !== 0) {
        message.error(res.msg || '登录失败')
        throw new Error(res.msg)
      }
      setSession(res.data.token, res.data.admin)
      setToken(res.data.token)
      setProfile(res.data.admin)
      message.success('登录成功')
    },
    [message],
  )

  const logout = useCallback(async () => {
    await authApi.logout()
    clearSession()
    setToken(null)
    setProfile(null)
    message.success('已退出')
  }, [message])

  const refresh = useCallback(async () => {
    const res = await authApi.refreshToken()
    if (res.code !== 0) {
      message.error(res.msg || '刷新失败')
      return
    }
    updateToken(res.data.token)
    setToken(res.data.token)
  }, [message])

  const value = useMemo<AuthContextValue>(
    () => ({
      token,
      profile,
      loading,
      login,
      logout,
      refresh,
    }),
    [token, profile, loading, login, logout, refresh],
  )

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
}
