export function slugifyToFilename(content: string): string {
  const base = content
    .toLowerCase()
    .replace(/[^a-z0-9\s-]/g, '')
    .trim()
    .replace(/\s+/g, '-')
    .replace(/-+/g, '-')
    .slice(0, 50)
    .replace(/-+$/, '')

  const slug = base || 'note'
  return `${slug}-${Date.now()}.json`
}
