import type { AdminProfile } from '../types/api'

const TOKEN_KEY = 'admin_token'
const PROFILE_KEY = 'admin_profile'

export function getToken(): string | null {
  return localStorage.getItem(TOKEN_KEY)
}

export function getProfile(): AdminProfile | null {
  const raw = localStorage.getItem(PROFILE_KEY)
  if (!raw) return null
  try {
    return JSON.parse(raw) as AdminProfile
  } catch {
    return null
  }
}

export function setSession(token: string, profile: AdminProfile) {
  localStorage.setItem(TOKEN_KEY, token)
  localStorage.setItem(PROFILE_KEY, JSON.stringify(profile))
}

export function updateToken(token: string) {
  localStorage.setItem(TOKEN_KEY, token)
}

export function clearSession() {
  localStorage.removeItem(TOKEN_KEY)
  localStorage.removeItem(PROFILE_KEY)
}
