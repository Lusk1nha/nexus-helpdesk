export function generateSlug(text: string): string {
  return text
    .toLowerCase()
    .normalize("NFD") // Separa os acentos das letras
    .replace(/[\u0300-\u036f]/g, "") // Remove os acentos
    .replace(/[^a-z0-9\s-]/g, "") // Remove caracteres inválidos
    .trim()
    .replace(/\s+/g, "-") // Troca espaços por hífens
    .replace(/-+/g, "-") // Remove hífens múltiplos
    .replace(/^-+|-+$/g, "") // Remove hífens das pontas
    .slice(0, 32) // Limita a 32 caracteres
}
