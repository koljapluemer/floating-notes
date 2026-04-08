import { marked } from 'marked'

marked.setOptions({ breaks: true })

export function renderMarkdown(content: string): string {
  return String(marked.parse(content))
}
