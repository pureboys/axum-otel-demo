export const sitePageStatusOptions = [
  { value: 0, label: '禁用' },
  { value: 1, label: '启用' },
]

export function labelForSitePageStatus(status: number) {
  return sitePageStatusOptions.find((o) => o.value === status)?.label ?? String(status)
}
