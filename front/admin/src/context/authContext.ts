import { createContext } from 'react'
import type { AdminProfile } from '../types/api'

export type AuthContextValue = {
  token: string | null
  profile: AdminProfile | null
  loading: boolean
  login: (username: string, password: string, altcha: string) => Promise<void>
  logout: () => Promise<void>
  refresh: () => Promise<void>
}

export const AuthContext = createContext<AuthContextValue | null>(null)
