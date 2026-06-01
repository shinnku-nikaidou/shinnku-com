import { SearchItem, SearchItemSchema } from "./validation";

export async function ai_search(q: string, n: number): Promise<SearchItem[]> {
  const serviceUrl = process.env.BACKEND_URL || "http://localhost:2999";

  const raw = await fetch(
    `${serviceUrl}/aisearch?q=${encodeURIComponent(q)}&n=${n}`,
  )
    .then((res) => res.json())
    .catch(() => []);

  try {
    return SearchItemSchema.array().parse(raw);
  } catch {
    return [];
  }
}

export async function default_search(
  q: string,
  n: number,
): Promise<SearchItem[]> {
  const serviceUrl = process.env.BACKEND_URL || "http://localhost:2999";

  const raw = await fetch(
    `${serviceUrl}/search?q=${encodeURIComponent(q)}&n=${n}`,
  )
    .then((res) => res.json())
    .catch(() => []);

  try {
    return SearchItemSchema.array().parse(raw);
  } catch {
    return [];
  }
}
