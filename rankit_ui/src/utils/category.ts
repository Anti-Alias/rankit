import { Page } from "./page";

export interface Category {
  id: number,
  name: string,
  image: string,
  color: string,
}

export async function fetchCategories(search: string, cursor?: string): Promise<Page<Category>> {
  await sleep(1000);
  const data = allCategories().filter(cat => cat.name.includes(search));
  const curs = cursor ? parseInt(cursor) : 0;
  const nextCursor = curs + pageSize;
  if (data.length - 1 > nextCursor) {
    const splicedData = data.splice(curs, nextCursor - curs);
    return { data: splicedData, cursor: nextCursor.toString() };
  }
  else {
    const splicedData = data.splice(curs, nextCursor - curs);
    return { data: splicedData }
  }
}

function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const pageSize: number = 32;

function allCategories(): Category[] {
  const result: Category[] = [];
  for (let i = 1; i <= 100; i += 3) {
    result.push({
      id: i,
      name: `Movies ${i}`,
      image: 'movies.svg',
      color: '#443388',
    });
    result.push({
      id: i + 1,
      name: `TV Shows ${i + 1}`,
      image: 'tv.svg',
      color: '#884433',
    });
    result.push({
      id: i + 2,
      name: `Games ${i + 2}`,
      image: 'games.svg',
      color: '#338844',
    });
  }
  return result;
}

