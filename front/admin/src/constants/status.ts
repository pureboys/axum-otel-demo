export const productNewsStatusOptions = [
  { value: 0, label: '草稿' },
  { value: 1, label: '已发布' },
  { value: 2, label: '待审核' },
]

export function labelForProductNewsStatus(status: number) {
  return productNewsStatusOptions.find((o) => o.value === status)?.label ?? String(status)
}
