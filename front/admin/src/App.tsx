import { Navigate, Route, Routes } from 'react-router-dom'
import { AuthProvider } from './context/AuthProvider'
import { AdminLayout } from './layouts/AdminLayout'
import { CategoriesPage } from './pages/CategoriesPage'
import { InquiriesPage } from './pages/InquiriesPage'
import { LoginPage } from './pages/Login'
import { NewsDetailPage } from './pages/NewsDetailPage'
import { NewsAdminPage } from './pages/NewsPage'
import { ProductDetailPage } from './pages/ProductDetailPage'
import { ProductsPage } from './pages/ProductsPage'
import { SitePageDetailPage } from './pages/SitePageDetailPage'
import { SitePagesPage } from './pages/SitePagesPage'
import { TagsPage } from './pages/TagsPage'
import { UsersPage } from './pages/UsersPage'
import { ProtectedRoute } from './routes/ProtectedRoute'

export default function App() {
  return (
    <AuthProvider>
      <Routes>
        <Route path="/login" element={<LoginPage />} />
        <Route
          element={
            <ProtectedRoute>
              <AdminLayout />
            </ProtectedRoute>
          }
        >
          <Route path="/" element={<Navigate to="/admins" replace />} />
          <Route path="/admins" element={<UsersPage />} />
          <Route path="/tags" element={<TagsPage />} />
          <Route path="/categories" element={<CategoriesPage />} />
          <Route path="/products/new" element={<ProductDetailPage />} />
          <Route path="/products/:id" element={<ProductDetailPage />} />
          <Route path="/products" element={<ProductsPage />} />
          <Route path="/news/new" element={<NewsDetailPage />} />
          <Route path="/news/:id" element={<NewsDetailPage />} />
          <Route path="/news" element={<NewsAdminPage />} />
          <Route path="/site-pages/new" element={<SitePageDetailPage />} />
          <Route path="/site-pages/:id" element={<SitePageDetailPage />} />
          <Route path="/site-pages" element={<SitePagesPage />} />
          <Route path="/inquiries" element={<InquiriesPage />} />
        </Route>
        <Route path="*" element={<Navigate to="/admins" replace />} />
      </Routes>
    </AuthProvider>
  )
}
